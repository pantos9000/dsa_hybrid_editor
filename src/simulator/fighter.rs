use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::app::character::{Character, Edge3, PassiveStats};
use crate::simulator::fight_report::FightStats;

use super::{
    cards::{Card, CardDeck, Suit},
    roller::{Roll, RollResult, roller},
};

#[derive(Debug, Default, Clone)]
pub struct Distance {
    base_contact: Cell<bool>,
}

impl Distance {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn close_in(&self) {
        self.base_contact.set(true);
    }

    pub fn back_off(&self) {
        self.base_contact.set(false);
    }

    pub fn base_contact(&self) -> bool {
        self.base_contact.get()
    }
}

#[allow(clippy::struct_excessive_bools)] // lots of yes/no state
#[derive(Debug, Clone)]
pub struct Fighter {
    fight_stats: RefCell<FightStats>, // change stats even when self is immutable
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
    attacked_wild: bool,
    distance: Rc<Distance>,
}

impl Fighter {
    pub fn new(character: Character, distance: Rc<Distance>) -> Self {
        let passive_stats = PassiveStats::new(&character);
        let berserker = character.edges.berserker == Edge3::Improved;
        let bennies = i8::from(character.bennies.count).try_into().unwrap();
        Self {
            fight_stats: RefCell::new(FightStats::new()),
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
            attacked_wild: false,
            distance,
        }
    }

    pub fn finish(self) -> FightStats {
        self.fight_stats.into_inner()
    }

    fn draw_card(&self, cards: &mut CardDeck) -> Card {
        let num_cards = match self.character.edges.kuhler_kopf {
            Edge3::None => 1,
            Edge3::Normal => 2,
            Edge3::Improved => 3,
        };

        let mut card = (0..num_cards).map(|_| cards.draw()).max().unwrap();

        if self.character.edges.schnell.is_set() {
            while card.suit() < Suit::Seven {
                card = cards.draw();
            }
        }

        card
    }

    fn weapon_has_reach(&self) -> bool {
        i8::from(self.character.weapon.reach) > 0
    }

    pub fn new_round(&mut self, cards: &mut CardDeck) -> Card {
        self.fight_stats.borrow_mut().add_round();
        let card = self.draw_card(cards);
        self.joker = card.is_joker();
        self.riposte_done = false;
        card
    }

    pub fn is_dead(&self) -> bool {
        let threshold = if self.berserker { 0 } else { 5 };
        self.passive_stats.life <= threshold
    }

    fn step_forward(&mut self, opponent: &mut Fighter) {
        if self.weapon_has_reach() || self.distance.base_contact() {
            // don't step forward if not needed
            return;
        }
        self.distance.close_in();
        opponent.trigger_erstschlag(self);
    }

    fn step_back(&mut self, opponent: &mut Fighter) {
        if !self.character.edges.erstschlag.is_set() {
            // if we don't have erstschlag, don't step back
            return;
        }
        if self.weapon_has_reach() {
            // always step back if opponent can't hit us
            self.distance.back_off();
            return;
        }
        opponent.unshake_against_erstschlag();
        if !opponent.shaken {
            // don't step back if opponent could hit us
            return;
        }
        self.distance.back_off();
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
            let mut attacks = attacks.into_iter();
            if let Some(attack) = attacks.next() {
                self.do_damage(false, opponent, attack);
            }
            debug_assert!(
                attacks.next().is_none(),
                "attacks should only contain single attack"
            );
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
        let mut attacks = attacks.into_iter();
        if let Some(attack) = attacks.next() {
            self.do_damage(true, opponent, attack);
        }
        debug_assert!(
            attacks.next().is_none(),
            "attacks should only contain single attack"
        );
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

        self.step_forward(opponent);
        if self.shaken {
            // we were interrupted by first strike, return without doing anything
            return;
        }

        self.do_full_attack(opponent);

        // take a step back to ready erstschlag
        self.step_back(opponent);
    }

    fn apply_wound_penalty(&self, roll: &mut Roll) {
        if self.berserker {
            return;
        }
        let wound_penalty: i8 = match self.passive_stats.life {
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
        if self.character.edges.berserker != Edge3::None {
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
            || !self.distance.base_contact()
        {
            return;
        }

        if self.character.bennies.use_for_erstschlag.is_set() {
            // if we would be able to do erstschlag, try to unshake if necessary
            self.unshake_with_bennie();
        }

        if self.shaken {
            // if still shaken, don't attack
            return;
        }

        // do erstschlag
        self.do_special_attack(opponent);
    }

    pub fn dex_roll(&self) -> Option<Roll> {
        let mut roll = roller().roll_attribute(self.character.attributes.ges)?;
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
        if self.character.bennies.use_for_unshake.is_set() {
            return self.unshake_with_bennie();
        }
        false
    }

    /// return `true` if bennie was used
    fn unshake_with_bennie(&mut self) -> bool {
        if !self.shaken || self.bennies == 0 {
            return false;
        }
        self.bennies -= 1;
        self.shaken = false;
        true
    }

    fn unshake_without_bennie(&mut self) -> bool {
        if self.interrupted {
            self.interrupted = false;
            return false;
        }
        if !self.shaken {
            return true;
        }
        let Some(mut roll) = roller().roll_attribute(self.character.attributes.wil) else {
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
            RollResult::Raise => {
                self.shaken = false;
                true
            }
        }
    }

    fn unshake_against_erstschlag(&mut self) {
        if !self.character.bennies.use_against_erstschlag.is_set() {
            return;
        }
        self.unshake_with_bennie();
    }

    fn unarmed(&self, opponent: &Self) -> bool {
        if self.character.edges.kampfkunstler.is_set() {
            return false;
        }
        if self.weapon_lost {
            return true;
        }
        self.character.weapon.unarmed()
            && !opponent.character.weapon.unarmed()
            && !opponent.weapon_lost
    }

    fn try_to_hit(
        &mut self,
        opponent: &Fighter,
        num_skill_dice: usize,
        modifier: i8,
    ) -> Option<Vec<AttackResult>> {
        let opponent_fell_modifier: u8 = if opponent.fell { 2 } else { 0 };
        let opponent_berserker_modifier: u8 = if opponent.berserker { 2 } else { 0 };
        let opponent_wild_modifier: u8 = if opponent.attacked_wild { 2 } else { 0 };
        let opponent_weapon_lost_modifier: u8 = if opponent.unarmed(self) { 2 } else { 0 };
        let mut opponent_parry = opponent.passive_stats.parry;
        opponent_parry = opponent_parry.saturating_sub(opponent_fell_modifier);
        opponent_parry = opponent_parry.saturating_sub(opponent_berserker_modifier);
        opponent_parry = opponent_parry.saturating_sub(opponent_wild_modifier);
        opponent_parry = opponent_parry.saturating_sub(opponent_weapon_lost_modifier);
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
        let apply_attack_head = |roll| {
            if self.character.passive_modifiers.attack_head.is_set() {
                roll - 4_u8
            } else {
                roll
            }
        };
        let apply_tuchfühlung = |mut roll| {
            self.apply_tuchfühlung_to_attack(opponent, &mut roll);
            roll
        };
        let check_hit = |mut roll: Roll| -> AttackResult {
            roll -= opponent_parry;
            match roll.as_i8() {
                ..0 => AttackResult::Miss,
                0..4 => AttackResult::Hit,
                4.. => AttackResult::Raise,
            }
        };
        let mut num_hits: u8 = 0;
        let check_fails = |hit| -> AttackResult {
            if hit != AttackResult::Miss {
                num_hits += 1;
            }
            hit
        };
        let rolls = roller().roll_skill_with_n_dice(
            self.character.skills.kampfen,
            num_skill_dice,
            self.berserker,
        )?;
        let hits = rolls
            .into_iter()
            .map(apply_modifier)
            .map(apply_passive_modifiers)
            .map(apply_wound_penalty)
            .map(apply_joker)
            .map(apply_berserker_attack)
            .map(apply_wild_attack)
            .map(apply_attack_head)
            .map(apply_tuchfühlung)
            .map(check_hit)
            .map(check_fails)
            .collect();

        if num_hits == 0 && self.character.bennies.use_for_attack.is_set() && self.bennies > 0 {
            self.bennies -= 1;
            self.try_to_hit(opponent, num_skill_dice, modifier)
        } else {
            self.fight_stats.borrow_mut().add_hits_dealt(num_hits);
            opponent
                .fight_stats
                .borrow_mut()
                .add_hits_received(num_hits);
            Some(hits)
        }
    }

    fn apply_opponents_armor(&self, opponent: &Self, damage: &mut Roll) {
        let armor = if self.character.passive_modifiers.attack_head.is_set() {
            opponent.character.armor.head
        } else {
            opponent.character.armor.torso
        };
        *damage -= i8::from(armor);
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

        let mut damage = if primary_weapon {
            roller().roll_weapon_damage(&self.character.weapon, self.character.attributes.sta)
        } else {
            roller().roll_weapon_damage(
                &self.character.secondary_weapon,
                self.character.attributes.sta,
            )
        };

        damage += roller().roll_attribute_without_wild_die(self.character.attributes.sta);
        if raise {
            damage += roller().roll_raise();
        }
        self.apply_piercing(opponent, &mut damage);
        self.apply_berserker_damage(&mut damage);
        if self.character.edges.ubertolpeln.is_set() && opponent.shaken {
            damage += 4_u8;
        }
        self.apply_wild(&mut damage);
        if self.character.passive_modifiers.attack_head.is_set() {
            damage += 6_u8;
        }
        self.apply_joker_to_damage(&mut damage);
        if u8::from(damage) < opponent.passive_stats.robustness {
            if self.character.bennies.use_for_damage.is_set() && self.bennies > 0 {
                self.bennies -= 1;
                self.do_damage(primary_weapon, opponent, attack_result);
            }
            return;
        }

        damage -= opponent.passive_stats.robustness;
        self.apply_opponents_armor(opponent, &mut damage);
        opponent.passive_stats.life -= damage;
        opponent.shaken = true;
        opponent.enable_berserker();

        self.fight_stats
            .borrow_mut()
            .add_damage_dealt(damage.into());
        opponent
            .fight_stats
            .borrow_mut()
            .add_damage_received(damage.into());

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
                // kampfkünstler not affected
                if self.character.edges.kampfkunstler.is_set() {
                    return;
                }
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
            CriticalFailResult::WeaponLost => {
                self.weapon_lost = !self.character.edges.kampfkunstler.is_set();
            }
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
        match roller().roll_critical_fail_result().as_u8() {
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
        match roller().roll_critical_fail_result().as_u8() {
            0..=1 => unreachable!(),
            13.. => unreachable!(),
            2..=3 => Self::WeaponDestroyed,
            4..=10 => Self::WeaponLost, // actually it blocks or something, but for the sake of simplicity...
            11..=12 => Self::Injured,
        }
    }
}
