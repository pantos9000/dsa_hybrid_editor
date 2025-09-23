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
use crate::app::gradient::Gradient;
use crate::app::group::CharIndex;
use crate::app::{CharSelection, GroupId};

pub type CharModFunc = Box<dyn FnOnce(&mut Character)>;

pub struct CharModification {
    group_id: GroupId,
    index: CharIndex,
    modification: CharModFunc,
}

impl CharModification {
    pub fn new(selection: CharSelection, modification: CharModFunc) -> Self {
        let (group_id, index) = match selection {
            CharSelection::Left(char_index) => (GroupId::Left, char_index),
            CharSelection::Right(char_index) => (GroupId::Right, char_index),
        };
        Self {
            group_id,
            index,
            modification,
        }
    }
}

/// Holds all total results
type DataMap = std::collections::HashMap<GroupData, FightReport, FxBuildHasher>;

const COUNT_FIGHTS: u32 = 5000;
const MAX_ROUNDS: u32 = 100;

pub struct Simulator {
    report_map: DataMap,
    workers: ThreadPool,
    report_send: mpsc::Sender<(GroupData, FightReport)>,
    report_recv: mpsc::Receiver<(GroupData, FightReport)>,
    group_data: GroupData,
    progress: ProgressTracker,
}

impl Default for Simulator {
    fn default() -> Self {
        let group_left = Vec::default();
        let group_right = Vec::default();
        Self::new(group_left, group_right)
    }
}

impl Simulator {
    fn new(mut group_left: Vec<Character>, mut group_right: Vec<Character>) -> Self {
        for character in &mut group_left {
            character.name.clear();
        }
        for character in &mut group_right {
            character.name.clear();
        }

        let group_data = GroupData {
            group_left,
            group_right,
        };
        let report_map = DataMap::default();
        let workers = ThreadPool::with_name("simulator_worker".to_owned(), 4);
        let (report_send, report_recv) = channel();
        Self {
            report_map,
            workers,
            report_send,
            report_recv,
            group_data,
            progress: ProgressTracker::new(),
        }
    }

    pub fn update(&mut self, mut group_left: Vec<Character>, mut group_right: Vec<Character>) {
        for c in &mut group_left {
            c.name.clear();
        }
        for c in &mut group_right {
            c.name.clear();
        }
        self.group_data = GroupData {
            group_left,
            group_right,
        };
    }

    fn update_report_map(&mut self) {
        while let Ok((group_data, report)) = self.report_recv.try_recv() {
            let old_val = self.report_map.insert(group_data, report);
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
        self.request_report(self.group_data.clone())
    }

    fn request_report(&mut self, group_data: GroupData) -> FightReport {
        // update the map (should be done regularly, so this should come first)
        self.update_report_map();

        // return early if already in map
        if let Some(report) = self.report_map.get(&group_data) {
            return report.clone();
        }

        // otherwise insert a placeholder...
        // (so we don't enqueue jobs multiple times)
        self.report_map
            .insert(group_data.clone(), FightReport::NONE);

        // ... and request that a result is calculated
        let report_send = self.report_send.clone();
        self.workers.execute(move || {
            let report = arena::simulate_fights(&group_data, COUNT_FIGHTS, MAX_ROUNDS);
            report_send
                .send((group_data, report))
                .expect("simulator is gone");
        });
        self.progress.add_request();

        FightReport::NONE
    }

    pub fn gradient(&mut self, modification: CharModification) -> Gradient {
        let mut modified_data = self.group_data.clone();
        modified_data.apply_mod(modification);

        let old_total = self.request_report(self.group_data.clone()).total();
        let new_total = self.request_report(modified_data).total();

        new_total - old_total
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
struct GroupData {
    group_left: Vec<Character>,
    group_right: Vec<Character>,
}

impl GroupData {
    fn apply_mod(&mut self, modification: CharModification) {
        let CharModification {
            group_id,
            index,
            modification,
        } = modification;
        let group = match group_id {
            GroupId::Left => &mut self.group_left,
            GroupId::Right => &mut self.group_right,
        };
        let char = group
            .get_mut(index.into_usize())
            .expect("index does not exist!");
        modification(char);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::thread;

    fn create_simulator() -> Simulator {
        let group = vec![Character::default()];
        Simulator::new(group.clone(), group)
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
        let mod1 = || CharModification {
            group_id: GroupId::Left,
            index: CharIndex::default(),
            modification: Box::new(|c: &mut Character| c.skills.kampfen.increment()),
        };
        let mod2 = || CharModification {
            group_id: GroupId::Right,
            index: CharIndex::default(),
            modification: Box::new(|c: &mut Character| c.attributes.sta.increment()),
        };
        let mut simulator = create_simulator();

        assert_eq!(simulator.progress(), 100);

        let gradient_1 = simulator.gradient(mod1());
        let gradient_2 = simulator.gradient(mod2());
        assert_eq!(gradient_1, Gradient::NONE);
        assert_eq!(gradient_2, Gradient::NONE);
        assert_ne!(simulator.progress(), 100);

        wait_until_done(&mut simulator);

        let gradient_1 = simulator.gradient(mod1());
        let gradient_2 = simulator.gradient(mod2());
        assert_ne!(gradient_1, Gradient::NONE);
        assert_ne!(gradient_2, Gradient::NONE);
        assert_eq!(simulator.progress(), 100);
    }
}
