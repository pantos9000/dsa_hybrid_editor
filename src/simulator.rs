use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

use crate::character::Character;

struct CharData {
    character: Character,
    enemy: Character,
}

pub struct Simulator {
    thread: thread::JoinHandle<()>,
    sender: mpsc::Sender<CharData>,
    gradients: Arc<Gradients>, // TODO use fxhash
    progress: Arc<AtomicU8>,
}

// TODO: drop

impl Simulator {
    fn spawn_simulator_thread(
        receiver: mpsc::Receiver<CharData>,
        gradients: &Arc<Gradients>,
        progress: &Arc<AtomicU8>,
    ) -> thread::JoinHandle<()> {
        let gradients = Arc::clone(gradients);
        let progress = Arc::clone(progress);
        thread::Builder::new()
            .name("simulator_thread".to_owned())
            .spawn(|| {
                log::info!("starting simulator thread");
                Self::simulator_thread(receiver, gradients, progress);
                log::info!("stopping simulator thread");
            })
            .expect("failed to spawn thread")
    }

    fn simulator_thread(
        receiver: mpsc::Receiver<CharData>,
        gradients: Arc<Gradients>,
        progress: Arc<AtomicU8>,
    ) {
        let Ok(CharData {
            mut character,
            mut enemy,
        }) = receiver.recv()
        else {
            log::debug!("channel is closed, exiting thread loop");
            return;
        };

        // we got a new configuration, so void all current results
        gradients.clear();
        progress.store(0, Ordering::Relaxed);
    }

    fn all_possible_changes() -> Vec<CharacterChange> {
        todo!()
    }

    #[allow(dead_code)] // TODO
    pub fn gradient(&self, change: CharacterChange) -> Option<Gradient> {
        self.gradients.get(&change).map(|item| *item)
    }

    #[allow(dead_code)] // TODO
    pub fn update_characters(&self, character: Character, enemy: Character) {
        let char_data = CharData { character, enemy };
        self.sender.send(char_data).expect("worker thread is gone");
    }

    /// Return the progress of current calculations
    #[allow(dead_code)] // TODO
    pub fn progress(&self) -> u8 {
        self.progress.load(Ordering::Relaxed)
    }
}
