use crate::character::{Character, PassiveStats};

use super::roller::{roller, Roll, RollResult};

#[derive(Debug, Clone)]
pub struct Fighter {
    pub(super) character: Character,
    passive_stats: PassiveStats,
    shaken: bool,
    interrupted: bool,
    fell: bool,
    joker: bool,
    weapon_lost: bool,
}

impl Default for Fighter {
    fn default() -> Self {
        Self::new(Character::default())
    }
}

impl Fighter {
    pub fn new(character: Character) -> Self {
        let passive_stats = PassiveStats::new(&character);
        Self {
            character,
            passive_stats,
            shaken: false,
            interrupted: false,
            fell: false,
            joker: false,
            weapon_lost: false,
        }
    }

    pub fn new_round(&mut self, joker: bool) {
        self.joker = joker;
    }

    pub fn is_dead(&self) -> bool {
        self.passive_stats.life <= 5
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

        let (hit, raise) = match self.try_to_hit(opponent) {
            AttackResult::CriticalFail => {
                self.critical_fail();
                (false, false)
            }
            AttackResult::Miss => (false, false),
            AttackResult::Hit => (true, false),
            AttackResult::Raise => (true, true),
        };
        if !hit {
            return;
        }

        self.do_damage(opponent, raise);
    }

    fn apply_wound_penalty(&self, roll: &mut Roll) {
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

    /// call from the other fighter!
    fn apply_fell_state(&self, roll: &mut Roll) {
        if self.fell {
            *roll += 2_u8;
        }
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
        match roll.eval() {
            RollResult::Fail => false,
            RollResult::Success => {
                self.shaken = false;
                false
            }
            RollResult::Raise => true,
        }
    }

    fn try_to_hit(&self, opponent: &Fighter) -> AttackResult {
        let Some(mut roll) = roller().roll_skill(&self.character.skills.kampfen) else {
            return AttackResult::CriticalFail;
        };
        self.apply_wound_penalty(&mut roll);
        self.apply_joker(&mut roll);
        opponent.apply_fell_state(&mut roll);
        roll += 1_u8; // add 1 to be able to check against 0
        roll -= opponent.passive_stats.parry;
        match roll.as_u8() {
            0 => AttackResult::Miss,
            1..=4 => AttackResult::Hit,
            5.. => AttackResult::Raise,
        }
    }

    fn do_damage(&self, opponent: &mut Fighter, raise: bool) {
        let mut damage = roller().roll_attribute_without_wild_die(&self.character.attributes.sta);
        damage += roller().roll_weapon_damage(&self.character.weapon);
        if raise {
            damage += roller().roll_raise();
        }
        self.apply_joker(&mut damage);
        if u8::from(damage) < opponent.passive_stats.robustness {
            return;
        }

        damage -= opponent.passive_stats.robustness;
        opponent.passive_stats.life -= damage;
        opponent.shaken = true;

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
                self.do_damage(&mut tmp, false);
                *self = tmp;
            }
            CriticalFailResult::HeavilyInjured => {
                let mut tmp = self.clone();
                self.do_damage(&mut tmp, true);
                *self = tmp;
            }
        }
    }
}

enum AttackResult {
    CriticalFail,
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
