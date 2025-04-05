use crate::character::{Character, Edge3, PassiveStats};

use super::{
    cards::{Card, CardDeck},
    roller::{roller, Roll, RollResult},
};

#[derive(Debug, Clone)]
pub struct Fighter {
    pub(super) character: Character,
    passive_stats: PassiveStats,
    shaken: bool,
    interrupted: bool,
    fell: bool,
    joker: bool,
    weapon_lost: bool,
    berserker: bool,
    riposte_done: bool,
    erstschlag_done: bool,
}

impl Default for Fighter {
    fn default() -> Self {
        Self::new(Character::default())
    }
}

impl Fighter {
    pub fn new(character: Character) -> Self {
        let passive_stats = PassiveStats::new(&character);
        let berserker = character.edges.berserker == Edge3::Improved;
        Self {
            character,
            passive_stats,
            shaken: false,
            interrupted: false,
            fell: false,
            joker: false,
            weapon_lost: false,
            berserker,
            riposte_done: false,
            erstschlag_done: false,
        }
    }

    pub fn new_round(&mut self, cards: &mut CardDeck) -> Card {
        let card = cards.draw();
        self.joker = card.is_joker();
        self.riposte_done = false;
        if i8::from(self.character.weapon.reach) > 0 {
            self.erstschlag_done = false;
        }
        card
    }

    pub fn is_dead(&self) -> bool {
        let threshold = match self.berserker {
            true => 0,
            false => 5,
        };
        self.passive_stats.life <= threshold
    }

    pub fn action(&mut self, opponent: &mut Fighter) {
        self.fell = false;
        if !self.unshake() {
            return;
        }

        if self.weapon_lost {
            self.weapon_lost = false;
            return;
        }

        opponent.trigger_erstschlag(self);
        if self.shaken {
            // we were interrupted by first strike, return without doing anything
            return;
        }

        let (num_rolls, modifier) = match self.character.edges.blitzhieb {
            Edge3::None => (1, 0),
            Edge3::Normal => (2, -2),
            Edge3::Improved => (2, 0),
        };
        let Some(attacks) = self.try_to_hit(opponent, num_rolls, modifier) else {
            self.critical_fail();
            return;
        };
        let attacks: Vec<_> = attacks.collect();
        for attack in attacks {
            self.do_damage(opponent, attack);
        }
    }

    fn apply_wound_penalty(&self, roll: &mut Roll) {
        if self.berserker {
            return;
        }
        let wound_penalty = match self.passive_stats.life {
            0..=10 => 3,
            11..=20 => 1,
            21.. => 0,
        };
        *roll -= wound_penalty;
    }

    fn apply_joker(&self, roll: &mut Roll) {
        if self.joker {
            *roll += 2_u8;
        }
    }

    fn enable_berserker(&mut self) {
        if self.character.edges.berserker == Edge3::Normal {
            self.berserker = true;
        }
    }

    fn apply_berserker_attack(&self, roll: &mut Roll) {
        if self.berserker {
            *roll += 2_u8;
        }
    }

    fn apply_berserker_damage(&self, roll: &mut Roll) {
        if self.berserker {
            *roll += 2_u8;
        }
    }

    fn apply_kampfreflexe(&self, roll: &mut Roll) {
        if self.character.edges.kampfreflexe.is_set() {
            *roll += 2_u8;
        }
    }

    fn apply_piercing(&self, opponent: &Self, roll: &mut Roll) {
        let piercing: u8 = i8::from(self.character.weapon.piercing)
            .try_into()
            .unwrap_or(0);
        let armor = u8::from(opponent.character.armor.torso);
        *roll += piercing.min(armor);
    }

    fn apply_tuchfühlung_to_attack(&self, opponent: &Self, roll: &mut Roll) {
        if self.character.edges.tuchfuhlung == Edge3::Improved {
            let reach = i8::from(opponent.character.weapon.reach);
            *roll += 1 + reach;
        }
    }

    fn apply_tuchfühlung_to_parry(&self, opponent: &Self, parry: &mut u8) {
        if self.character.edges.tuchfuhlung != Edge3::None {
            let reach = i8::from(opponent.character.weapon.reach)
                .try_into()
                .unwrap_or(0);
            *parry += 1 + reach;
        }
    }

    fn trigger_riposte(&mut self, opponent: &mut Self) {
        if self.shaken || self.riposte_done {
            return;
        }
        match self.character.edges.riposte {
            Edge3::None => return,
            Edge3::Normal => self.riposte_done = true,
            Edge3::Improved => (),
        }

        let Some(mut attacks) = self.try_to_hit(opponent, 1, 0) else {
            self.critical_fail();
            return;
        };
        let attack = attacks.next().unwrap(); // can only be a single attack
        drop(attacks);
        self.do_damage(opponent, attack);
    }

    fn trigger_erstschlag(&mut self, opponent: &mut Self) {
        if !self.character.edges.erstschlag.is_set()
            || i8::from(opponent.character.weapon.reach) > 0
            || self.shaken
            || self.erstschlag_done
        {
            return;
        }

        self.erstschlag_done = true;

        let Some(mut attacks) = self.try_to_hit(opponent, 1, 0) else {
            self.critical_fail();
            return;
        };
        let attack = attacks.next().unwrap();
        drop(attacks);
        self.do_damage(opponent, attack);
    }

    pub fn dex_roll(&self) -> Option<Roll> {
        let mut roll = roller().roll_attribute(&self.character.attributes.ges)?;
        self.apply_joker(&mut roll);
        self.apply_wound_penalty(&mut roll);
        Some(roll)
    }

    /// returns `true` if char still has an action this round
    fn unshake(&mut self) -> bool {
        if self.interrupted {
            self.interrupted = false;
            return false;
        }
        if !self.shaken {
            return true;
        }
        let Some(mut roll) = roller().roll_attribute(&self.character.attributes.wil) else {
            return false;
        };
        self.apply_wound_penalty(&mut roll);
        self.apply_joker(&mut roll);
        self.apply_kampfreflexe(&mut roll);
        match roll.eval() {
            RollResult::Fail => false,
            RollResult::Success => {
                self.shaken = false;
                false
            }
            RollResult::Raise => true,
        }
    }

    fn try_to_hit<'a>(
        &'a self,
        opponent: &'a Fighter,
        num_skill_dice: usize,
        modifier: i8,
    ) -> Option<impl Iterator<Item = AttackResult> + use<'a>> {
        let fell_state_modifier: u8 = match opponent.fell {
            true => 2,
            false => 0,
        };
        let berserker_state_modifier: u8 = match opponent.berserker {
            true => 2,
            false => 0,
        };
        let mut opponent_parry = opponent.passive_stats.parry;
        opponent_parry = opponent_parry.saturating_sub(fell_state_modifier);
        opponent_parry = opponent_parry.saturating_sub(berserker_state_modifier);
        opponent.apply_tuchfühlung_to_parry(self, &mut opponent_parry);

        let apply_modifier = move |roll| roll + modifier;
        let apply_wound_penalty = move |mut roll| {
            self.apply_wound_penalty(&mut roll);
            roll
        };
        let apply_joker = move |mut roll| {
            self.apply_joker(&mut roll);
            roll
        };
        let apply_berserker_attack = move |mut roll| {
            self.apply_berserker_attack(&mut roll);
            roll
        };
        let apply_tuchfühlung = move |mut roll| {
            self.apply_tuchfühlung_to_attack(opponent, &mut roll);
            roll
        };
        let check_hit = move |mut roll: Roll| -> AttackResult {
            roll += 1_u8; // add 1 to be able to check against 0 later
            roll -= opponent_parry;
            match roll.as_u8() {
                0 => AttackResult::Miss,
                1..=4 => AttackResult::Hit,
                5.. => AttackResult::Raise,
            }
        };
        let rolls =
            roller().roll_skill_with_n_dice(&self.character.skills.kampfen, num_skill_dice)?;
        let hit_iter = rolls
            .into_iter()
            .map(apply_modifier)
            .map(apply_wound_penalty)
            .map(apply_joker)
            .map(apply_berserker_attack)
            .map(apply_tuchfühlung)
            .map(check_hit);
        Some(hit_iter)
    }

    fn do_damage(&mut self, opponent: &mut Self, attack_result: AttackResult) {
        // might have gone shaken in between due to riposte
        if self.shaken {
            return;
        }

        let raise = match attack_result {
            AttackResult::Miss => {
                opponent.trigger_riposte(self);
                return;
            }
            AttackResult::Hit => false,
            AttackResult::Raise => true,
        };

        let mut damage = roller().roll_attribute_without_wild_die(&self.character.attributes.sta);
        damage += roller().roll_weapon_damage(&self.character.weapon);
        if raise {
            damage += roller().roll_raise();
        }
        self.apply_piercing(opponent, &mut damage);
        self.apply_joker(&mut damage);
        self.apply_berserker_damage(&mut damage);
        if self.character.edges.ubertolpeln.is_set() && opponent.shaken {
            damage += 4_u8;
        }
        if u8::from(damage) < opponent.passive_stats.robustness {
            return;
        }

        damage -= opponent.passive_stats.robustness;
        opponent.passive_stats.life -= damage;
        opponent.shaken = true;
        opponent.enable_berserker();

        // instead of implementing interrupting logic, we can just assume that
        // damage done while holding a joker just interrupts the opponent.
        if self.joker && !opponent.joker {
            opponent.interrupted = true;
        }
    }

    fn critical_fail(&mut self) {
        let fail_result = CriticalFailResult::short_range();

        match fail_result {
            CriticalFailResult::WeaponDestroyed => {
                // handle this as defeat for now
                self.passive_stats.life = 0;
            }
            CriticalFailResult::Fell => {
                // actually also requires 2 pace, but treat it the same for now
                self.shaken = true;
                self.fell = true;
            }
            CriticalFailResult::Tripped => {
                self.shaken = true;
                self.fell = true;
            }
            CriticalFailResult::WeaponLost => self.weapon_lost = true,
            CriticalFailResult::Injured => {
                let mut tmp = self.clone();
                self.do_damage(&mut tmp, AttackResult::Hit);
                *self = tmp;
            }
            CriticalFailResult::HeavilyInjured => {
                let mut tmp = self.clone();
                self.do_damage(&mut tmp, AttackResult::Raise);
                *self = tmp;
            }
        }
    }
}

enum AttackResult {
    Miss,
    Hit,
    Raise,
}

enum CriticalFailResult {
    WeaponDestroyed,
    Fell,
    Tripped,
    WeaponLost,
    Injured,
    HeavilyInjured,
}

impl CriticalFailResult {
    fn short_range() -> Self {
        match roller().roll_critical_fail_result().into() {
            0..=1 => unreachable!(),
            13.. => unreachable!(),
            2 => Self::WeaponDestroyed,
            3..=5 => Self::Fell,
            6..=8 => Self::Tripped,
            9..=10 => Self::WeaponLost,
            11 => Self::Injured,
            12 => Self::HeavilyInjured,
        }
    }

    #[allow(dead_code)]
    fn long_range() -> Self {
        match roller().roll_critical_fail_result().into() {
            0..=1 => unreachable!(),
            13.. => unreachable!(),
            2..=3 => Self::WeaponDestroyed,
            4..=10 => Self::WeaponLost, // actually it blocks or something, but for the sake of simplicity...
            11..=12 => Self::Injured,
        }
    }
}
