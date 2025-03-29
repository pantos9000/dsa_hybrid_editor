use crate::gradient::Total;

use super::cards::{Card, CardDeck};
use super::fighter::Fighter;
use super::CharData;

const COUNT_FIGHTS: u32 = 1000;
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
    let _probability_draw = calc_prob(count_draws);

    let total: i8 = probability_win.try_into().unwrap();
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

        let fighter_card = self.cards.draw();
        let opponent_card = self.cards.draw();
        self.fighter.new_round(fighter_card.is_joker());
        self.opponent.new_round(opponent_card.is_joker());

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

enum FightIsOver {
    FighterWon,
    OpponentWon,
    Draw,
}

enum Initiative {
    FighterFirst,
    OpponentFirst,
}
