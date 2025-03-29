mod arena;
mod cards;
mod fighter;
mod roller;

use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam::channel::unbounded as channel;
use crossbeam::channel::{self as mpsc, RecvTimeoutError};
use dashmap::DashMap;
use fxhash::FxBuildHasher;

use crate::character::Character;
use crate::gradient::{Gradient, Total};

pub type CharModification = Box<dyn FnOnce(&mut Character)>;

/// Holds all total results
type DataMap = DashMap<CharData, Total, FxBuildHasher>;

pub struct Simulator {
    thread: Option<thread::JoinHandle<()>>,
    total_map: Arc<DataMap>,
    send: Option<mpsc::Sender<CharData>>,
    stop: Arc<AtomicBool>,
    progress: Arc<AtomicU8>,
    char_data: CharData,
}

impl Drop for Simulator {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        drop(self.send.take());
        if let Some(thread) = self.thread.take() {
            thread.join().expect("worker thread panicked");
        }
    }
}

impl Default for Simulator {
    fn default() -> Self {
        let character = Character::default();
        let opponent = Character::default();
        Self::new(character, opponent)
    }
}

impl Simulator {
    fn new(mut character: Character, mut opponent: Character) -> Self {
        character.name.clear();
        opponent.name.clear();
        let char_data = CharData {
            character,
            opponent,
        };
        let (send, recv) = channel();
        let stop = Arc::new(AtomicBool::new(false));
        let progress = Arc::new(AtomicU8::new(100));
        let gradient_map = Arc::new(DataMap::default());
        let thread = Some(Self::spawn_simulator_thread(
            &gradient_map,
            recv,
            &stop,
            &progress,
        ));
        Self {
            thread,
            total_map: gradient_map,
            send: Some(send),
            stop,
            progress,
            char_data,
        }
    }

    fn spawn_simulator_thread(
        gradient_map: &Arc<DataMap>,
        recv: mpsc::Receiver<CharData>,
        stop: &Arc<AtomicBool>,
        progress: &Arc<AtomicU8>,
    ) -> thread::JoinHandle<()> {
        let progress = Arc::clone(progress);
        let stop = Arc::clone(stop);
        let gradient_map = Arc::clone(gradient_map);
        thread::Builder::new()
            .name("simulator_thread".to_owned())
            .spawn(|| {
                log::info!("starting simulator thread");
                Self::simulator_thread(gradient_map, recv, stop, progress);
                log::info!("stopping simulator thread");
            })
            .expect("failed to spawn thread")
    }

    fn simulator_thread(
        gradient_map: Arc<DataMap>,
        recv: mpsc::Receiver<CharData>,
        stop: Arc<AtomicBool>,
        progress: Arc<AtomicU8>,
    ) {
        let mut count_done = 0;

        'thread_loop: loop {
            // stop if we are dropped
            if stop.load(Ordering::Relaxed) {
                break 'thread_loop;
            }

            // get more work
            let char_data = match recv.recv_timeout(Duration::from_secs(1)) {
                Ok(char_data) => char_data,
                Err(RecvTimeoutError::Disconnected) => break 'thread_loop,
                Err(RecvTimeoutError::Timeout) => {
                    count_done = 0;
                    continue 'thread_loop;
                }
            };

            // do the calculation and store the result if needed
            let total = arena::calculate_probability(&char_data);
            gradient_map.insert(char_data, total);

            // update progress
            count_done += 1;
            let all = count_done + recv.len();
            let new_progress = 100 * count_done / all;
            let new_progress = new_progress.try_into().unwrap_or(100);
            progress.store(new_progress, Ordering::Relaxed);
        }
    }

    pub fn update_characters(&mut self, mut character: Character, mut opponent: Character) {
        character.name.clear();
        opponent.name.clear();
        self.char_data = CharData {
            character,
            opponent,
        };
    }

    /// Return the progress of current calculations
    pub fn progress(&self) -> u8 {
        self.progress.load(Ordering::Relaxed)
    }

    pub fn total(&self) -> Total {
        self.modified_total(self.char_data.clone())
    }

    pub fn gradient(&self, modification: CharModification) -> Gradient {
        let mut char_data = self.char_data.clone();
        modification(&mut char_data.character);

        let old_total = self.total();
        let new_total = self.modified_total(char_data);

        new_total - old_total
    }

    fn modified_total(&self, char_data: CharData) -> Total {
        // return early if already in map
        if let Some(total) = self.total_map.get(&char_data) {
            return *total;
        }

        // otherwise insert a placeholder...
        self.total_map.insert(char_data.clone(), Total::default());

        // ... and request that a result is calculated
        self.send
            .as_ref()
            .expect("sender is gone")
            .send(char_data)
            .expect("simulator thread is gone");

        Total::NONE
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct CharData {
    character: Character,
    opponent: Character,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_simulator() -> Simulator {
        let char = Character::default();
        Simulator::new(char.clone(), char)
    }

    #[test]
    fn test_simulator_starts_with_100_percent() {
        let simulator = create_simulator();
        assert_eq!(simulator.progress(), 100);
    }

    #[test]
    #[ignore] // TODO fix this test or remove
    fn test_simulator_progress() {
        let mod1 = |c: &mut Character| c.skills.kampfen.increment();
        let mod2 = |c: &mut Character| c.attributes.sta.increment();
        let mod1 = Box::new(mod1);
        let mod2 = Box::new(mod2);
        let simulator = create_simulator();

        let _ = simulator.gradient(mod1.clone());
        let _ = simulator.gradient(mod2.clone());
        std::thread::sleep(std::time::Duration::from_millis(140));
        assert_eq!(simulator.progress(), 50);
        std::thread::sleep(std::time::Duration::from_millis(140));
        assert_eq!(simulator.progress(), 100);

        assert_ne!(simulator.gradient(mod1), Gradient::NONE);
        assert_ne!(simulator.gradient(mod2), Gradient::NONE);
    }
}
