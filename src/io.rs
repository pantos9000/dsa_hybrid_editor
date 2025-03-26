use std::{fs, sync::mpsc, thread};

use crate::{character::Character, util::LogError};
use anyhow::{Context, Result};

pub struct IoThread {
    thread: Option<thread::JoinHandle<()>>,
    request: Option<mpsc::Sender<IoRequest>>,
    response: Option<mpsc::Receiver<IoResponse>>,
}

impl Default for IoThread {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for IoThread {
    fn drop(&mut self) {
        drop(self.request.take());
        drop(self.response.take());
        if let Some(thread) = self.thread.take() {
            thread.join().expect("thread panicked");
        }
    }
}

// public interface
impl IoThread {
    pub fn new() -> Self {
        let (request_send, request_recv) = mpsc::channel();
        let (response_send, response_recv) = mpsc::channel();

        let thread = thread::Builder::new()
            .name("io_thread".to_owned())
            .spawn(move || {
                Self::thread_func(request_recv, response_send);
            })
            .expect("failed to spawn thread");

        Self {
            thread: Some(thread),
            request: Some(request_send),
            response: Some(response_recv),
        }
    }

    pub fn request(&self, request: IoRequest) {
        self.request.as_ref().unwrap().send(request).unwrap();
    }

    pub fn poll_iter(&self) -> impl Iterator<Item = IoResponse> + use<'_> {
        self.response.as_ref().unwrap().try_iter()
    }
}

// private methods
impl IoThread {
    fn thread_func(request: mpsc::Receiver<IoRequest>, response: mpsc::Sender<IoResponse>) {
        log::info!("io thread started");
        'thread_loop: loop {
            match request.recv() {
                Err(_) => break 'thread_loop,
                Ok(IoRequest::Save(character)) => {
                    Self::save(character).or_log_err("failed to save character");
                    log::debug!("character saved");
                }
                Ok(IoRequest::LoadChar) => {
                    let new_char = match Self::load() {
                        Ok(Some(new_char)) => new_char,
                        Ok(None) => continue 'thread_loop,
                        Err(err) => {
                            log::error!("failed to load character: {err}");
                            continue 'thread_loop;
                        }
                    };
                    let Ok(()) = response.send(IoResponse::CharLoaded(new_char)) else {
                        break 'thread_loop;
                    };
                }
                Ok(IoRequest::LoadOpponent) => {
                    let new_opp = match Self::load() {
                        Ok(Some(new_opp)) => new_opp,
                        Ok(None) => continue 'thread_loop,
                        Err(err) => {
                            log::error!("failed to load opponent: {err}");
                            continue 'thread_loop;
                        }
                    };
                    let Ok(()) = response.send(IoResponse::OpponentLoaded(new_opp)) else {
                        break 'thread_loop;
                    };
                }
            }
        }
        log::info!("io thread stopped");
    }

    fn save(character: Character) -> Result<()> {
        let char_serialized = serde_json::to_vec_pretty(&character)
            .context("failed to convert character to JSON format")?;

        let Some(path) = rfd::FileDialog::new().save_file() else {
            log::debug!("save file dialog was canceled");
            return Ok(());
        };
        fs::write(&path, &char_serialized).context("failed to write to file")?;
        Ok(())
    }

    fn load() -> Result<Option<Character>> {
        let Some(path) = rfd::FileDialog::new().pick_file() else {
            log::debug!("load file dialog was canceled");
            return Ok(None);
        };

        let data = fs::read(path).context("failed to read from file")?;
        let new_char =
            serde_json::from_slice(&data).context("failed to convert JSON to character")?;
        Ok(Some(new_char))
    }
}

pub enum IoRequest {
    Save(Character),
    LoadChar,
    LoadOpponent,
}

pub enum IoResponse {
    CharLoaded(Character),
    OpponentLoaded(Character),
}
