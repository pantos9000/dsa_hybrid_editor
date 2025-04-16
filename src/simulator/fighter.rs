use crate::character::{Character, Edge3, PassiveStats};

use super::{
    cards::{Card, CardDeck, Suit},
    roller::{roller, Roll, RollResult},
};

#[derive(Debug, Clone)]
pub struct Fighter {
    character: Character,
    passive_stats: PassiveStats,
    bennies: u8,
    shaken: bool,
    interrupted: bool,
    fell: bool,
    joker: bool,
    weapon_lost: bool,
    berserker: bool,
    riposte_done: bool,
    erstschlag_done: bool,
    attacked_wild: bool,
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
        let bennies = i8::from(character.bennies.num_bennies).try_into().unwrap();
        Self {
            character,
            passive_stats,
            bennies,
            shaken: false,
            interrupted: false,
            fell: false,
            joker: false,
            weapon_lost: false,
            berserker,
            riposte_done: false,
            erstschlag_done: false,
            attacked_wild: false,
        }
    }

    fn draw_card(&self, cards: &mut CardDeck) -> Card {
        let num_cards = match self.character.edges.kuhler_kopf {
            Edge3::None => 1,
            Edge3::Normal => 2,
            Edge3::Improved => 3,
        };

        let mut card = (0..num_cards).map(|_| cards.draw()).max().unwrap();

        while card.suit() < Suit::Seven {
            card = cards.draw();
        }

        card
    }

    pub fn new_round(&mut self, cards: &mut CardDeck) -> Card {
        let card = self.draw_card(cards);
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

    fn do_full_attack(&mut self, opponent: &mut Fighter) {
        if self.character.weapon.active {
            if self.character.passive_modifiers.attack_wild.is_set() {
                self.attacked_wild = true;
            }

            let (num_rolls, mut modifier) = match self.character.edges.blitzhieb {
                Edge3::None => (1, 0),
                Edge3::Normal => (2, -2),
                Edge3::Improved => (2, 0),
            };
            if self.character.secondary_weapon.active
                && !self.character.edges.beidhandiger_kampf.is_set()
            {
                modifier -= 2;
            }
            let Some(attacks) = self.try_to_hit(opponent, num_rolls, modifier) else {
                self.critical_fail(true);
                return;
            };
            for attack in attacks {
                self.do_damage(true, opponent, attack);
            }
        }

        if self.character.secondary_weapon.active {
            if self.character.passive_modifiers.attack_wild.is_set() {
                self.attacked_wild = true;
            }

            let mut modifier = 0;
            if !self.character.edges.beidhandig.is_set() {
                modifier -= 2;
            }
            if self.character.weapon.active && !self.character.edges.beidhandiger_kampf.is_set() {
                modifier -= 2;
            }
            let Some(attacks) = self.try_to_hit(opponent, 1, modifier) else {
                self.critical_fail(false);
                return;
            };
            let attack = attacks[0];
            debug_assert_eq!(
                attacks.len(),
                1,
                "attacks should only contain single attack"
            );
            self.do_damage(false, opponent, attack);
        }
    }

    fn do_special_attack(&mut self, opponent: &mut Fighter) {
        if !self.character.weapon.active {
            return;
        }
        if self.character.edges.erbarmungslos.is_set()
            && self.character.passive_modifiers.attack_wild.is_set()
        {
            self.attacked_wild = true;
        }

        let Some(attacks) = self.try_to_hit(opponent, 1, 0) else {
            self.critical_fail(true);
            return;
        };
        let attack = attacks[0];
        debug_assert_eq!(
            attacks.len(),
            1,
            "attacks should only contain single attack"
        );
        self.do_damage(true, opponent, attack);
    }

    pub fn action(&mut self, opponent: &mut Fighter) {
        self.fell = false;
        self.attacked_wild = false;
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

        self.do_full_attack(opponent);
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

    fn apply_joker_to_damage(&self, roll: &mut Roll) {
        if !self.joker {
            return;
        }
        *roll += 2_u8;
        if !self.character.edges.machtiger_hieb.is_set() {
            return;
        }
        *roll *= 2_u8;
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

    fn apply_wild(&self, roll: &mut Roll) {
        if self.attacked_wild {
            *roll += 2_u8;
        }
    }

    fn apply_kampfreflexe(&self, roll: &mut Roll) {
        if self.character.edges.kampfreflexe.is_set() {
            *roll += 2_u8;
        }
    }

    fn apply_piercing(&self, opponent: &Self, roll: &mut Roll) {
        let piercing = i8::from(self.character.weapon.piercing);
        let armor = i8::from(opponent.character.armor.torso);
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

        self.do_special_attack(opponent);
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

        self.do_special_attack(opponent);
    }

    pub fn dex_roll(&self) -> Option<Roll> {
        let mut roll = roller().roll_attribute(&self.character.attributes.ges)?;
        self.apply_joker(&mut roll);
        self.apply_wound_penalty(&mut roll);
        Some(roll)
    }

    /// returns `true` if char still has an action this round
    fn unshake(&mut self) -> bool {
        let has_action = self.unshake_without_bennie();
        if has_action {
            return true;
        }
        if self.character.bennies.use_for_unshake.is_set() && self.bennies > 0 {
            self.bennies -= 1;
            return true;
        }
        false
    }

    fn unshake_without_bennie(&mut self) -> bool {
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

    fn try_to_hit(
        &mut self,
        opponent: &Fighter,
        num_skill_dice: usize,
        modifier: i8,
    ) -> Option<Vec<AttackResult>> {
        let opponent_fell_modifier: u8 = match opponent.fell {
            true => 2,
            false => 0,
        };
        let opponent_berserker_modifier: u8 = match opponent.berserker {
            true => 2,
            false => 0,
        };
        let opponent_wild_modifier: u8 = match opponent.attacked_wild {
            true => 2,
            false => 0,
        };
        let mut opponent_parry = opponent.passive_stats.parry;
        opponent_parry = opponent_parry.saturating_sub(opponent_fell_modifier);
        opponent_parry = opponent_parry.saturating_sub(opponent_berserker_modifier);
        opponent_parry = opponent_parry.saturating_sub(opponent_wild_modifier);
        opponent.apply_tuchfühlung_to_parry(self, &mut opponent_parry);

        let apply_modifier = |roll| roll + modifier;
        let apply_passive_modifiers =
            |roll| roll + i8::from(self.character.passive_modifiers.attack);
        let apply_wound_penalty = |mut roll| {
            self.apply_wound_penalty(&mut roll);
            roll
        };
        let apply_joker = |mut roll| {
            self.apply_joker(&mut roll);
            roll
        };
        let apply_berserker_attack = |mut roll| {
            self.apply_berserker_attack(&mut roll);
            roll
        };
        let apply_wild_attack = |mut roll| {
            self.apply_wild(&mut roll);
            roll
        };
        let apply_tuchfühlung = |mut roll| {
            self.apply_tuchfühlung_to_attack(opponent, &mut roll);
            roll
        };
        let check_hit = |mut roll: Roll| -> AttackResult {
            roll += 1_u8; // add 1 to be able to check against 0 later
            roll -= opponent_parry;
            match roll.as_u8() {
                0 => AttackResult::Miss,
                1..=4 => AttackResult::Hit,
                5.. => AttackResult::Raise,
            }
        };
        let mut all_failed = true;
        let check_fails = |hit| -> AttackResult {
            if hit != AttackResult::Miss {
                all_failed = false;
            }
            hit
        };
        let rolls =
            roller().roll_skill_with_n_dice(&self.character.skills.kampfen, num_skill_dice)?;
        let hits = rolls
            .into_iter()
            .map(apply_modifier)
            .map(apply_passive_modifiers)
            .map(apply_wound_penalty)
            .map(apply_joker)
            .map(apply_berserker_attack)
            .map(apply_wild_attack)
            .map(apply_tuchfühlung)
            .map(check_hit)
            .map(check_fails)
            .collect();

        if all_failed && self.character.bennies.use_for_attack.is_set() && self.bennies > 0 {
            self.bennies -= 1;
            self.try_to_hit(opponent, num_skill_dice, modifier)
        } else {
            Some(hits)
        }
    }

    fn do_damage(
        &mut self,
        primary_weapon: bool,
        opponent: &mut Self,
        attack_result: AttackResult,
    ) {
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

        let mut damage = match primary_weapon {
            true => roller().roll_weapon_damage(&self.character.weapon),
            false => roller().roll_weapon_damage(&self.character.secondary_weapon),
        };

        damage += roller().roll_attribute_without_wild_die(&self.character.attributes.sta);
        if raise {
            damage += roller().roll_raise();
        }
        self.apply_piercing(opponent, &mut damage);
        self.apply_berserker_damage(&mut damage);
        if self.character.edges.ubertolpeln.is_set() && opponent.shaken {
            damage += 4_u8;
        }
        self.apply_wild(&mut damage);
        self.apply_joker_to_damage(&mut damage);
        if u8::from(damage) < opponent.passive_stats.robustness {
            if self.character.bennies.use_for_damage.is_set() && self.bennies > 0 {
                self.bennies -= 1;
                self.do_damage(primary_weapon, opponent, attack_result);
            }
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

    fn critical_fail(&mut self, primary_weapon: bool) {
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
                tmp.do_damage(primary_weapon, self, AttackResult::Hit);
            }
            CriticalFailResult::HeavilyInjured => {
                let mut tmp = self.clone();
                tmp.do_damage(primary_weapon, self, AttackResult::Raise);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
