use std::cell::RefCell;
use std::rc::Rc;

use crate::simulator::fight_report::FightStats;
use crate::simulator::fighter::DistanceMap;

use super::GroupData;
use super::cards::CardDeck;
use super::fight_report::{FightOutcome, FightReport, ReportBuilder};
use super::fighter::{Fighter, Group};

pub fn simulate_fights(char_data: &GroupData, count_fights: u32, max_rounds: u32) -> FightReport {
    let mut report = ReportBuilder::new();
    for _ in 0..count_fights {
        let outcome = calc_fight(char_data, max_rounds);
        report.add_fight(outcome);
    }
    report.build()
}

fn calc_fight(char_data: &GroupData, max_rounds: u32) -> FightOutcome {
    let mut arena = Arena::new(char_data);
    'fight: for _ in 0..max_rounds {
        if matches!(arena.round(), Err(FightIsOver)) {
            break 'fight;
        }
    }
    arena.finish()
}

struct FightIsOver;
type FightResult = Result<(), FightIsOver>;

#[derive(Debug)]
struct Arena {
    cards: CardDeck,
    stats: Rc<RefCell<FightStats>>,
    group_left: Vec<Rc<RefCell<Fighter>>>,
    group_right: Vec<Rc<RefCell<Fighter>>>,
}

impl Arena {
    fn new(group_data: &GroupData) -> Self {
        let cards = CardDeck::new();
        let stats = Rc::new(RefCell::new(FightStats::new()));
        let distance_map = Rc::new(RefCell::new(DistanceMap::new()));
        let group_left = group_data
            .group_left
            .iter()
            .cloned()
            .map(|char| {
                Fighter::new(
                    char,
                    Group::Left,
                    Rc::clone(&distance_map),
                    Some(Rc::clone(&stats)),
                )
            })
            .map(|fighter| Rc::new(RefCell::new(fighter)))
            .collect();
        let group_right = group_data
            .group_right
            .iter()
            .cloned()
            .map(|char| Fighter::new(char, Group::Right, Rc::clone(&distance_map), None))
            .map(|fighter| Rc::new(RefCell::new(fighter)))
            .collect();
        Self {
            cards,
            stats,
            group_left,
            group_right,
        }
    }

    fn do_fighter_action(&mut self, fighter: &Rc<RefCell<Fighter>>) -> FightResult {
        let mut fighter = fighter.borrow_mut();
        let opponents = match fighter.group() {
            Group::Left => &mut self.group_right,
            Group::Right => &mut self.group_left,
        };
        if opponents.is_empty() {
            return Err(FightIsOver);
        }

        fighter.action(opponents);

        Ok(())
    }

    fn round(&mut self) -> FightResult {
        self.stats.borrow_mut().add_round();
        self.cards.new_round();
        self.group_left
            .iter_mut()
            .chain(self.group_right.iter_mut())
            .for_each(|f| f.borrow_mut().new_round(&mut self.cards));

        let initiative_list = self.initiative();

        for fighter in initiative_list {
            // groups don't contain dead fighters, but initiative list is not updated
            if fighter.borrow().is_dead() {
                continue;
            }

            self.do_fighter_action(&fighter)?;
            self.filter_out_dead_fighters();
        }

        Ok(())
    }

    fn filter_out_dead_fighters(&mut self) {
        self.group_left
            .retain(|fighter| !fighter.borrow().is_dead());
        self.group_right
            .retain(|fighter| !fighter.borrow().is_dead());
    }

    fn initiative(&mut self) -> Vec<Rc<RefCell<Fighter>>> {
        let mut initiative_list: Vec<_> = self
            .group_left
            .iter()
            .chain(self.group_right.iter())
            .cloned()
            .collect();
        initiative_list.sort_by_key(|a| a.borrow().drawn_card());

        Self::initiative_jokers(&mut initiative_list);

        initiative_list
    }

    /// extra case if 2 have joker
    fn initiative_jokers(initiative_list: &mut [Rc<RefCell<Fighter>>]) {
        let Some(first) = initiative_list.first() else {
            return;
        };
        let Some(second) = initiative_list.get(1) else {
            return;
        };
        if !first.borrow().drawn_card().is_joker() {
            return;
        }
        if !second.borrow().drawn_card().is_joker() {
            return;
        }
        let swap = 'dex_roll_loop: loop {
            match first.borrow().dex_roll().cmp(&second.borrow().dex_roll()) {
                std::cmp::Ordering::Less => break 'dex_roll_loop true,
                std::cmp::Ordering::Greater => break 'dex_roll_loop false,
                std::cmp::Ordering::Equal => continue 'dex_roll_loop,
            }
        };
        if swap {
            initiative_list.swap(0, 1);
        }
    }

    fn finish(mut self) -> FightOutcome {
        self.filter_out_dead_fighters();
        let left_dead = self.group_left.is_empty();
        let right_dead = self.group_right.is_empty();
        drop(self.group_left);
        drop(self.group_right);
        let stats = Rc::into_inner(self.stats)
            .expect("other Rcs should be gone")
            .into_inner();
        match (left_dead, right_dead) {
            (true, true) => FightOutcome::Draw(stats),
            (true, false) => FightOutcome::RightWon(stats),
            (false, true) => FightOutcome::LeftWon(stats),
            (false, false) => FightOutcome::Draw(stats),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::character::Character;

    use super::*;

    #[test]
    fn test_probability_for_same_chars_is_around_50() {
        let count_fights = 10000;
        let max_rounds = 100;

        let character = Character::default();
        let data = GroupData {
            group_left: vec![character.clone()],
            group_right: vec![character],
        };

        let prob: i8 = simulate_fights(&data, count_fights, max_rounds)
            .total()
            .try_into()
            .unwrap();

        eprintln!("prob = {prob}");
        assert!((45..=55).contains(&prob), "{prob} is too far away from 50");
    }
}
