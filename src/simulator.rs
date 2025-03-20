use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam::channel::unbounded as channel;
use crossbeam::channel::{self as mpsc, RecvTimeoutError};
use dashmap::DashMap;
use fxhash::FxBuildHasher;

use crate::character::Character;
use crate::gradient::Gradient;

pub type CharacterModification = Box<dyn FnOnce(&mut Character)>;

/// Holds all gradient results
type GradientMap = DashMap<CharData, Gradient, FxBuildHasher>;

pub struct Simulator {
    thread: Option<thread::JoinHandle<()>>,
    gradient_map: Arc<GradientMap>,
    send: Option<mpsc::Sender<CharData>>,
    progress: Arc<AtomicU8>,
    char_data: CharData,
}

impl Drop for Simulator {
    fn drop(&mut self) {
        drop(self.send.take());
        if let Some(thread) = self.thread.take() {
            thread.join().expect("worker thread panicked");
        }
    }
}

impl Simulator {
    #[allow(dead_code)] // TODO
    fn new(character: Character, opponent: Character) -> Self {
        let char_data = CharData {
            character,
            opponent,
        };
        let (send, recv) = channel();
        let progress = Arc::new(AtomicU8::new(100));
        let gradient_map = Arc::new(GradientMap::default());
        let thread = Some(Self::spawn_simulator_thread(&gradient_map, recv, &progress));
        Self {
            thread,
            gradient_map,
            send: Some(send),
            progress,
            char_data,
        }
    }

    fn spawn_simulator_thread(
        gradient_map: &Arc<GradientMap>,
        recv: mpsc::Receiver<CharData>,
        progress: &Arc<AtomicU8>,
    ) -> thread::JoinHandle<()> {
        let progress = Arc::clone(progress);
        let gradient_map = Arc::clone(gradient_map);
        thread::Builder::new()
            .name("simulator_thread".to_owned())
            .spawn(|| {
                log::info!("starting simulator thread");
                Self::simulator_thread(gradient_map, recv, progress);
                log::info!("stopping simulator thread");
            })
            .expect("failed to spawn thread")
    }

    fn simulator_thread(
        gradient_map: Arc<GradientMap>,
        recv: mpsc::Receiver<CharData>,
        progress: Arc<AtomicU8>,
    ) {
        fn dummy_calculation(_char_data: &CharData) -> Gradient {
            std::thread::sleep(std::time::Duration::from_millis(100));
            42.try_into().unwrap()
        }

        let mut count_done = 0;

        'thread_loop: loop {
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
            if !gradient_map.contains_key(&char_data) {
                let gradient = dummy_calculation(&char_data);
                gradient_map.insert(char_data, gradient);
            }

            // update progress
            count_done += 1;
            let all = count_done + recv.len();
            let new_progress = 100 * count_done / all;
            let new_progress = new_progress.try_into().unwrap_or(100);
            progress.store(new_progress, Ordering::Relaxed);
        }
    }

    #[allow(dead_code)] // TODO
    pub fn update_characters(&mut self, character: Character, opponent: Character) {
        self.char_data = CharData {
            character,
            opponent,
        };
    }

    /// Return the progress of current calculations
    #[allow(dead_code)] // TODO
    pub fn progress(&self) -> u8 {
        self.progress.load(Ordering::Relaxed)
    }

    #[allow(dead_code)] // TODO
    pub fn gradient(&self, modification: CharacterModification) -> Option<Gradient> {
        let mut char_data = self.char_data.clone();
        modification(&mut char_data.character);

        // return early if already in map
        if let Some(gradient) = self.gradient_map.get(&char_data) {
            return Some(*gradient);
        }

        self.send
            .as_ref()
            .expect("sender is gone")
            .send(char_data)
            .expect("simulator thread is gone");
        None
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct CharData {
    character: Character,
    opponent: Character,
}

#[cfg(test)]
mod tests {
    use crate::character::AttributeName;
    use crate::character::SkillName;

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
    fn test_simulator_progress() {
        let mod1 = |c: &mut Character| c.skills[SkillName::Kämpfen].increment();
        let mod2 = |c: &mut Character| c.attributes[AttributeName::Stä].increment();
        let mod1 = Box::new(mod1);
        let mod2 = Box::new(mod2);
        let simulator = create_simulator();

        let _ = simulator.gradient(mod1.clone());
        let _ = simulator.gradient(mod2.clone());
        std::thread::sleep(std::time::Duration::from_millis(140));
        assert_eq!(simulator.progress(), 50);
        std::thread::sleep(std::time::Duration::from_millis(140));
        assert_eq!(simulator.progress(), 100);

        assert!(simulator.gradient(mod1).is_some());
        assert!(simulator.gradient(mod2).is_some());
    }
}
