use crate::gradient::Total;

use super::cards::{Card, CardDeck};
use super::fighter::Fighter;
use super::CharData;

const COUNT_FIGHTS: u32 = 5000;
const MAX_ROUNDS: u32 = 100;

pub fn calculate_probability(char_data: &CharData) -> Total {
    fn calc_prob(wins: u32) -> u32 {
        100 * wins / COUNT_FIGHTS
    }

    let mut count_character_wins = 0;
    let mut count_opponent_wins = 0;
    let mut count_draws = 0;

    for _ in 0..COUNT_FIGHTS {
        match calc_fight(char_data) {
            FightIsOver::FighterWon => count_character_wins += 1,
            FightIsOver::OpponentWon => count_opponent_wins += 1,
            FightIsOver::Draw => count_draws += 1,
        }
    }
    let probability_win = calc_prob(count_character_wins);
    let _probability_loss = calc_prob(count_opponent_wins);
    let probability_draw = calc_prob(count_draws);

    let total = probability_win + probability_draw / 2;
    let total: i8 = total.try_into().unwrap();
    Total::try_from(total).unwrap()
}

fn calc_fight(char_data: &CharData) -> FightIsOver {
    let mut arena = Arena::new(char_data);
    for _ in 0..MAX_ROUNDS {
        match arena.round() {
            Ok(()) => (),
            Err(report) => return report,
        }
    }
    FightIsOver::Draw
}

#[derive(Debug, Default)]
struct Arena {
    cards: CardDeck,
    fighter: Fighter,
    opponent: Fighter,
}

impl Arena {
    fn new(char_data: &CharData) -> Self {
        let cards = CardDeck::new();
        let fighter = Fighter::new(char_data.character.clone());
        let opponent = Fighter::new(char_data.opponent.clone());
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
            (true, true) => Err(FightIsOver::Draw),
            (true, false) => Err(FightIsOver::OpponentWon),
            (false, true) => Err(FightIsOver::FighterWon),
            (false, false) => Ok(()),
        }
    }
}

type FightResult = Result<(), FightIsOver>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FightIsOver {
    FighterWon,
    OpponentWon,
    Draw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Initiative {
    FighterFirst,
    OpponentFirst,
}

#[cfg(test)]
mod tests {
    use crate::character::Character;

    use super::*;

    #[test]
    fn test_probability_for_same_chars_is_around_50() {
        for _ in 0..20 {
            let char1 = Character::default();
            let char2 = Character::default();
            let data1 = CharData {
                character: char1.clone(),
                opponent: char2.clone(),
            };
            let data2 = CharData {
                character: char2,
                opponent: char1,
            };

            let prob1: Option<i8> = calculate_probability(&data1).into();
            let prob2: Option<i8> = calculate_probability(&data2).into();
            let prob1 = prob1.unwrap();
            let prob2 = prob2.unwrap();

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
}
