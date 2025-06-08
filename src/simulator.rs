mod arena;
mod cards;
mod fight_report;
mod fighter;
mod roller;

use std::time;

use crossbeam::channel as mpsc;
use crossbeam::channel::unbounded as channel;
use fxhash::FxBuildHasher;
use threadpool::ThreadPool;

use fight_report::FightReport;

use crate::app::character::Character;
use crate::app::gradient::{Gradient, Total};

pub type CharModification = Box<dyn FnOnce(&mut Character)>;

/// Holds all total results
type DataMap = std::collections::HashMap<CharData, FightReport, FxBuildHasher>;

const COUNT_FIGHTS: u32 = 5000;
const MAX_ROUNDS: u32 = 100;

pub struct Simulator {
    report_map: DataMap,
    workers: ThreadPool,
    report_send: mpsc::Sender<(CharData, FightReport)>,
    report_recv: mpsc::Receiver<(CharData, FightReport)>,
    char_data: CharData,
    progress: ProgressTracker,
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
        let report_map = DataMap::default();
        let workers = ThreadPool::with_name("simulator_worker".to_owned(), 4);
        let (report_send, report_recv) = channel();
        Self {
            report_map,
            workers,
            report_send,
            report_recv,
            char_data,
            progress: ProgressTracker::new(),
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

    fn update_report_map(&mut self) {
        while let Ok((char_data, report)) = self.report_recv.try_recv() {
            let old_val = self.report_map.insert(char_data, report);
            assert!(old_val.is_some());
            self.progress.add_resolved();
        }
    }

    /// Return the progress of current calculations
    pub fn progress(&mut self) -> u8 {
        self.update_report_map();
        self.progress.get_progress()
    }

    pub fn report(&mut self) -> FightReport {
        self.request_report(self.char_data.clone())
    }

    fn request_report(&mut self, char_data: CharData) -> FightReport {
        // update the map (should be done regularly, so this should come first)
        self.update_report_map();

        // return early if already in map
        if let Some(report) = self.report_map.get(&char_data) {
            return report.clone();
        }

        // otherwise insert a placeholder...
        // (so we don't enqueue jobs multiple times)
        self.report_map.insert(char_data.clone(), FightReport::NONE);

        // ... and request that a result is calculated
        let report_send = self.report_send.clone();
        self.workers.execute(move || {
            let report = arena::simulate_fights(&char_data, COUNT_FIGHTS, MAX_ROUNDS);
            report_send
                .send((char_data, report))
                .expect("simulator is gone");
        });
        self.progress.add_request();

        FightReport::NONE
    }

    pub fn gradient(&mut self, modification: CharModification) -> Gradient {
        let mut char_data = self.char_data.clone();
        modification(&mut char_data.character);

        let old_total = self.total(self.char_data.clone());
        let new_total = self.total(char_data);

        new_total - old_total
    }

    fn total(&mut self, char_data: CharData) -> Total {
        self.request_report(char_data).total()
    }
}

#[derive(Debug)]
struct ProgressTracker {
    requested: usize,
    resolved: usize,
    last_request: time::Instant,
}

impl ProgressTracker {
    const RESET_TIMEOUT: time::Duration = time::Duration::from_secs(1);

    fn new() -> Self {
        Self {
            requested: 0,
            resolved: 0,
            last_request: time::Instant::now(),
        }
    }

    fn add_request(&mut self) {
        self.requested += 1;
        self.last_request = time::Instant::now();
    }

    fn add_resolved(&mut self) {
        self.resolved += 1;
    }

    fn get_progress(&mut self) -> u8 {
        if self.requested == 0 {
            return 100;
        }

        let progress = 100 * self.resolved / self.requested;
        assert!(progress <= 100);
        let progress: u8 = progress.try_into().unwrap();

        let reset_timeout_reached = self.last_request.elapsed() > Self::RESET_TIMEOUT;
        if progress == 100 && reset_timeout_reached {
            self.requested = 0;
            self.resolved = 0;
        }

        progress
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

    use std::thread;

    fn create_simulator() -> Simulator {
        let char = Character::default();
        Simulator::new(char.clone(), char)
    }

    fn wait_until_done(simulator: &mut Simulator) {
        let max_tries = 100;
        let sleep_time = time::Duration::from_millis(20);

        for _ in 0..max_tries {
            if simulator.progress() == 100 {
                return;
            }

            thread::sleep(sleep_time);
        }

        panic!("simulator still not done: progress != 100 after {max_tries}x {sleep_time:?}");
    }

    #[test]
    fn test_simulator_starts_with_100_percent() {
        let mut simulator = create_simulator();
        assert_eq!(simulator.progress(), 100);
    }

    #[test]
    fn test_simulator_progress() {
        let mod1 = |c: &mut Character| c.skills.kampfen.increment();
        let mod2 = |c: &mut Character| c.attributes.sta.increment();
        let mod1 = Box::new(mod1);
        let mod2 = Box::new(mod2);
        let mut simulator = create_simulator();

        assert_eq!(simulator.progress(), 100);

        let gradient_1 = simulator.gradient(mod1.clone());
        let gradient_2 = simulator.gradient(mod2.clone());
        assert_eq!(gradient_1, Gradient::NONE);
        assert_eq!(gradient_2, Gradient::NONE);
        assert_ne!(simulator.progress(), 100);

        wait_until_done(&mut simulator);

        let gradient_1 = simulator.gradient(mod1.clone());
        let gradient_2 = simulator.gradient(mod2.clone());
        assert_ne!(gradient_1, Gradient::NONE);
        assert_ne!(gradient_2, Gradient::NONE);
        assert_eq!(simulator.progress(), 100);
    }
}
