use std::rc::Rc;

use super::GroupData;
use super::cards::{Card, CardDeck};
use super::fight_report::{FightOutcome, FightReport, ReportBuilder};
use super::fighter::{Distance, Fighter};

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
    group_left: Vec<Fighter>,
    group_right: Vec<Fighter>,
}

impl Arena {
    fn new(group_data: &GroupData) -> Self {
        let cards = CardDeck::new();
        let distance_from_left = Rc::new(Distance::new());
        let distance_from_right = Rc::new(Distance::new());
        let group_left = group_data
            .group_left
            .iter()
            .cloned()
            .map(|char| Fighter::new(char, Rc::clone(&distance_from_right)))
            .collect();
        let group_right = group_data
            .group_right
            .iter()
            .cloned()
            .map(|char| Fighter::new(char, Rc::clone(&distance_from_left)))
            .collect();
        let fighter = Fighter::new(char_data.character.clone(), Rc::clone(&distance));
        let opponent = Fighter::new(char_data.opponent.clone(), distance);
        Self {
            cards,
            fighter,
            opponent,
        }
    }

    fn round(&mut self) -> FightResult {
        self.cards.new_round();

        let fighter_card = self.fighter.new_round(&mut self.cards);
        let opponent_card = self.opponent.new_round(&mut self.cards);

        match self.initiative(fighter_card, opponent_card) {
            Initiative::FighterFirst => {
                self.fighter.action(&mut self.opponent);
                self.check_end()?;
                self.opponent.action(&mut self.fighter);
                self.check_end()?;
            }
            Initiative::OpponentFirst => {
                self.opponent.action(&mut self.fighter);
                self.check_end()?;
                self.fighter.action(&mut self.opponent);
                self.check_end()?;
            }
        }

        Ok(())
    }

    fn initiative(&mut self, fighter_card: Card, opponent_card: Card) -> Initiative {
        match fighter_card.cmp(&opponent_card) {
            std::cmp::Ordering::Less => return Initiative::OpponentFirst,
            std::cmp::Ordering::Greater => return Initiative::FighterFirst,
            std::cmp::Ordering::Equal => { /* contine */ }
        }

        // both have joker, do a dex comparison until one wins
        'dex_roll_loop: loop {
            match self.fighter.dex_roll().cmp(&self.opponent.dex_roll()) {
                std::cmp::Ordering::Less => return Initiative::OpponentFirst,
                std::cmp::Ordering::Greater => return Initiative::FighterFirst,
                std::cmp::Ordering::Equal => continue 'dex_roll_loop,
            }
        }
    }

    fn check_end(&self) -> FightResult {
        match (self.fighter.is_dead(), self.opponent.is_dead()) {
            (false, false) => Ok(()),
            _ => Err(FightIsOver),
        }
    }

    fn finish(self) -> FightOutcome {
        let fighter_dead = self.fighter.is_dead();
        let opponent_dead = self.opponent.is_dead();
        let stats = self.fighter.finish();
        match (fighter_dead, opponent_dead) {
            (true, true) => FightOutcome::Draw(stats),
            (true, false) => FightOutcome::OpponentWon(stats),
            (false, true) => FightOutcome::FighterWon(stats),
            (false, false) => FightOutcome::Draw(stats),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Initiative {
    FighterFirst,
    OpponentFirst,
}

#[cfg(test)]
mod tests {
    use crate::app::character::Character;

    use super::*;

    #[test]
    fn test_probability_for_same_chars_is_around_50() {
        let count_fights = 10000;
        let max_rounds = 100;

        let char1 = Character::default();
        let char2 = Character::default();
        let data1 = GroupData {
            character: char1.clone(),
            opponent: char2.clone(),
        };
        let data2 = GroupData {
            character: char2,
            opponent: char1,
        };

        let prob1: i8 = simulate_fights(&data1, count_fights, max_rounds)
            .total()
            .try_into()
            .unwrap();
        let prob2: i8 = simulate_fights(&data2, count_fights, max_rounds)
            .total()
            .try_into()
            .unwrap();

        eprintln!("prob1 = {prob1}");
        eprintln!("prob2 = {prob1}");
        assert!(
            (48..=52).contains(&prob1),
            "{prob1} is too far away from 50"
        );
        assert!(
            (48..=52).contains(&prob2),
            "{prob2} is too far away from 50"
        );
    }
}
