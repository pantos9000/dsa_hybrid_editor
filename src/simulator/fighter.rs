use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use crate::app::character::{Character, Edge3, PassiveStats};
use crate::simulator::fight_report::FightStats;
use crate::simulator::roller::RollError;

use super::{
    cards::{Card, CardDeck, Suit},
    roller::{Roll, RollResult, roller},
};

struct NoOpponentLeft;
type ActionResult<T> = Result<T, NoOpponentLeft>;

#[derive(Debug, Clone, Copy)]
struct CriticalMiss;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Left,
    Right,
}

#[allow(clippy::struct_excessive_bools, reason = "lots of yes/no state")]
#[derive(Debug, Clone)]
pub struct Fighter {
    group: Group,
    fight_stats: Option<Rc<RefCell<FightStats>>>,
    drawn_card: Option<Card>,
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
    distance_map: Rc<RefCell<DistanceMap>>,
    distance_id: u16,
}

impl Fighter {
    pub fn new(
        character: Character,
        group: Group,
        distance_map: Rc<RefCell<DistanceMap>>,
        stats: Option<Rc<RefCell<FightStats>>>,
    ) -> Self {
        let passive_stats = PassiveStats::new(&character);
        let berserker = character.edges.berserker == Edge3::Improved;
        let bennies = i8::from(character.bennies.count).try_into().unwrap();
        let distance_id = distance_map.borrow_mut().register_fighter(group);
        Self {
            group,
            fight_stats: stats,
            drawn_card: None,
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
            distance_map,
            distance_id,
        }
    }

    pub fn group(&self) -> Group {
        self.group
    }

    fn draw_card(&mut self, cards: &mut CardDeck) {
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

        self.drawn_card = Some(card);
    }

    pub fn drawn_card(&self) -> Card {
        self.drawn_card.unwrap()
    }

    fn weapon_has_reach(&self) -> bool {
        i8::from(self.character.weapon.reach) > 0
    }

    pub fn new_round(&mut self, cards: &mut CardDeck) {
        self.draw_card(cards);
        self.joker = self.drawn_card.unwrap().is_joker();
        self.riposte_done = false;
    }

    pub fn is_dead(&self) -> bool {
        let threshold = if self.berserker { 0 } else { 5 };
        self.passive_stats.life <= threshold
    }

    /// take a step forward, but only toward our target
    fn step_forward(&mut self, opponents: &[Rc<RefCell<Fighter>>]) -> ActionResult<()> {
        let mut opponent = self.pick_opponent(opponents)?;
        let mut distance_map = self.distance_map.borrow_mut();
        let base_contact_to_target = distance_map.base_contact_mut(self, &opponent);
        if self.weapon_has_reach() || *base_contact_to_target {
            // don't step forward if not needed
            return Ok(());
        }
        *base_contact_to_target = true;
        let base_contact = *base_contact_to_target;
        drop(distance_map);
        opponent.trigger_erstschlag(self, base_contact);
        Ok(())
    }

    /// take a step back from everybody
    fn step_back(&mut self, opponents: &[Rc<RefCell<Fighter>>]) {
        if !self.character.edges.erstschlag.is_set() {
            // if we don't have erstschlag, don't step back
            return;
        }

        let opponents_cant_attack = self.weapon_has_reach()
            || opponents
                .iter()
                .filter(|opponent| !opponent.borrow().is_dead())
                .map(|opponent| opponent.borrow_mut())
                .map(|mut opponent| {
                    opponent.unshake_against_step_back();
                    opponent
                })
                .all(|opponent| opponent.shaken);

        if !opponents_cant_attack {
            // don't step back if an opponent could hit us
            return;
        }

        // step back from all opponents
        for opponent in opponents
            .iter()
            .filter(|opponent| !opponent.borrow().is_dead())
            .map(|opponent| opponent.borrow_mut())
        {
            let mut distance_map = self.distance_map.borrow_mut();
            let base_contact_to_opponent = distance_map.base_contact_mut(self, &opponent);
            *base_contact_to_opponent = false;
        }
    }

    fn attack_with_primary_weapon(&mut self, opponent: &mut Fighter) {
        let mut dmg_modifier = 0;
        let (num_rolls, mut attack_modifier) = match self.character.edges.blitzhieb {
            Edge3::None => (1, 0),
            Edge3::Normal => (2, -2),
            Edge3::Improved => (2, 0),
        };
        if self.character.passive_modifiers.attack_wild.is_set() {
            self.attacked_wild = true;
            attack_modifier += 2;
            dmg_modifier += 2;
        }
        if self.character.secondary_weapon.active
            && !self.character.edges.beidhandiger_kampf.is_set()
        {
            attack_modifier -= 2;
        }
        #[allow(clippy::single_match_else, reason = "better readability")]
        let attacks = match self.try_to_hit_with_bennie(opponent, num_rolls, attack_modifier) {
            Ok(results) => results,
            Err(CriticalMiss) => {
                self.critical_fail(true);
                return;
            }
        };
        for attack in attacks {
            self.do_damage(true, opponent, attack, dmg_modifier, false);
        }
    }

    fn attack_with_second_weapon(&mut self, opponent: &mut Fighter) {
        let mut attack_modifier = 0;
        let mut dmg_modifier = 0;
        if self.character.passive_modifiers.attack_wild.is_set() {
            self.attacked_wild = true;
            attack_modifier += 2;
            dmg_modifier += 2;
        }
        if !self.character.edges.beidhandig.is_set() {
            attack_modifier -= 2;
        }
        if self.character.weapon.active && !self.character.edges.beidhandiger_kampf.is_set() {
            attack_modifier -= 2;
        }

        #[allow(clippy::single_match_else, reason = "better readability")]
        let attacks = match self.try_to_hit_with_bennie(opponent, 1, attack_modifier) {
            Ok(results) => results,
            Err(CriticalMiss) => {
                self.critical_fail(false);
                return;
            }
        };
        let mut attacks = attacks.into_iter();
        if let Some(attack) = attacks.next() {
            self.do_damage(false, opponent, attack, dmg_modifier, false);
        }
        debug_assert!(
            attacks.next().is_none(),
            "attacks should only contain single attack"
        );
    }

    fn wanna_do_rundumschlag(&self, opponents: &[Rc<RefCell<Fighter>>]) -> bool {
        // don't do rundumschlag if we don't have it, duh
        if !self.character.edges.rundumschlag.is_set() {
            return false;
        }

        // only use rundumschlag if we can at least hit two opponents
        let distance_map = self.distance_map.borrow();
        let count_attackable = opponents
            .iter()
            .map(|opponent| opponent.borrow())
            .filter(|opponent| distance_map.base_contact(self, opponent))
            .count();
        count_attackable >= 2
    }

    fn do_rundumschlag(&mut self, opponents: &[Rc<RefCell<Fighter>>]) {
        let mut attack_modifier = 0;
        let mut dmg_modifier = 0;
        if self.character.passive_modifiers.attack_wild.is_set() {
            self.attacked_wild = true;
            attack_modifier += 2;
            dmg_modifier += 2;
        }
        // only attack with primary weapon
        if self.character.secondary_weapon.active
            && !self.character.edges.beidhandiger_kampf.is_set()
        {
            attack_modifier -= 2;
        }

        let attack_rolls = loop {
            let roll = match self.roll_attack_dice(1) {
                Ok(mut rolls) => rolls.pop(), // we rolled with 1
                Err(RollError::CriticalFail) => {
                    self.critical_fail(true);
                    return;
                }
                Err(RollError::Fail) => None,
            };

            let attacks = if let Some(roll) = roll {
                opponents
                    .iter()
                    .map(|opponent| opponent.borrow_mut())
                    .map(|opponent| {
                        let result =
                            self.try_to_hit_without_bennie(&opponent, roll, attack_modifier);
                        (opponent, result)
                    })
                    .collect()
            } else {
                Vec::new()
            };

            let count_hits = attacks
                .iter()
                .filter(|(_opponent, attack)| attack != &AttackResult::Miss)
                .count()
                .try_into()
                .unwrap_or(u8::MAX);
            if count_hits == 0 && self.character.bennies.use_for_attack.is_set() && self.bennies > 0
            {
                // use a benny and reroll if we can...
                self.bennies -= 1;
                continue;
            }

            // ...else take the result and adjust the stats
            if let Some(stats) = self.fight_stats.as_ref() {
                stats.borrow_mut().add_hits_dealt(count_hits);
            }
            for (opponent, attack) in &attacks {
                if attack != &AttackResult::Miss
                    && let Some(stats) = opponent.fight_stats.as_ref()
                {
                    stats.borrow_mut().add_hits_received(1);
                }
            }
            break attacks;
        };

        for (mut opponent, attack_result) in attack_rolls {
            self.do_damage(false, &mut opponent, attack_result, dmg_modifier, false);
        }
    }

    fn do_full_attack(&mut self, opponents: &[Rc<RefCell<Fighter>>]) -> ActionResult<()> {
        if self.character.weapon.active {
            if self.wanna_do_rundumschlag(opponents) {
                self.do_rundumschlag(opponents);
            } else {
                let mut opponent = self.pick_opponent(opponents)?;
                self.attack_with_primary_weapon(&mut opponent);
            }
        }
        if self.character.bennies.use_for_unshake.is_set() {
            self.unshake_with_bennie();
        }
        if self.shaken {
            return Ok(());
        }
        if self.character.secondary_weapon.active {
            let mut opponent = self.pick_opponent(opponents)?;
            self.attack_with_second_weapon(&mut opponent);
        }

        Ok(())
    }

    fn do_special_attack(&mut self, opponent: &mut Fighter) {
        if !self.character.weapon.active {
            return;
        }
        let mut attack_modifier = 0;
        let mut dmg_modifier = 0;
        if self.character.edges.erbarmungslos.is_set()
            && self.character.passive_modifiers.attack_wild.is_set()
        {
            self.attacked_wild = true;
            attack_modifier += 2;
            dmg_modifier += 2;
        }

        #[allow(clippy::single_match_else, reason = "better readability")]
        let attacks = match self.try_to_hit_with_bennie(opponent, 1, attack_modifier) {
            Ok(results) => results,
            Err(CriticalMiss) => {
                self.critical_fail(true);
                return;
            }
        };
        let mut attacks = attacks.into_iter();
        if let Some(attack) = attacks.next() {
            self.do_damage(true, opponent, attack, dmg_modifier, false);
        }
        debug_assert!(
            attacks.next().is_none(),
            "attacks should only contain single attack"
        );
    }

    fn pick_opponent<'o>(
        &self,
        opponents: &'o [Rc<RefCell<Fighter>>],
    ) -> ActionResult<RefMut<'o, Fighter>> {
        let opponent = opponents
            .first()
            .expect("fight should be over if opponent list is empty")
            .borrow_mut();
        if opponent.is_dead() {
            return Err(NoOpponentLeft);
        }
        Ok(opponent)
    }

    pub fn action(&mut self, opponents: &[Rc<RefCell<Fighter>>]) {
        self.fell = false;
        self.attacked_wild = false;
        if !self.unshake() {
            return;
        }

        if self.weapon_lost {
            self.weapon_lost = false;
            return;
        }

        // take a step forward
        if let Err(NoOpponentLeft) = self.step_forward(opponents) {
            return;
        }
        if self.character.bennies.use_against_step_back.is_set() {
            self.unshake_with_bennie();
        }
        if self.shaken {
            // we were interrupted by first strike, return without doing anything
            return;
        }

        if let Err(NoOpponentLeft) = self.do_full_attack(opponents) {
            return;
        }

        // take a step back from all opponents to ready erstschlag
        self.step_back(opponents);
    }

    fn apply_wound_penalty(&self, roll: &mut Roll) {
        if self.berserker || self.character.passive_modifiers.no_wound_penalty.is_set() {
            return;
        }
        let wound_penalty: i8 = match self.passive_stats.life {
            0..=10 => 3,
            11..=20 => 1,
            21.. => 0,
        };
        *roll -= wound_penalty;
    }

    fn apply_gangup(opponent: &Self, roll: &mut Roll) {
        let opponent_has_two_weapons = opponent.character.edges.kampfkunstler.is_set()
            || (opponent.character.weapon.active && opponent.character.secondary_weapon.active);
        if opponent.character.edges.fechten_m2w.is_set() && opponent_has_two_weapons {
            return;
        }

        let distance_map = opponent.distance_map.borrow();
        let count_attackers = distance_map.all_base_contacts(opponent).count();
        let gangup: u8 = (count_attackers - 1).min(4).try_into().unwrap();
        *roll += gangup;
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
        if self.riposte_done || self.character.edges.riposte != Edge3::Improved {
            return;
        }

        if self.character.bennies.use_for_special_attacks.is_set() {
            // if we would be able to do riposte, try to unshake if necessary
            self.unshake_with_bennie();
        }

        if self.shaken {
            return;
        }

        self.riposte_done = true;

        self.do_special_attack(opponent);
    }

    fn trigger_erstschlag(&mut self, opponent: &mut Self, base_contact_to_target: bool) {
        if !self.character.edges.erstschlag.is_set() {
            return;
        }

        if i8::from(opponent.character.weapon.reach) == 0 && !base_contact_to_target {
            return;
        }

        if self.character.bennies.use_for_special_attacks.is_set() {
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

    pub fn dex_roll(&self) -> Result<Roll, RollError> {
        let mut roll = roller().roll_attribute(self.character.attributes.ges)?;
        self.apply_joker(&mut roll);
        self.apply_wound_penalty(&mut roll);
        Ok(roll)
    }

    /// returns `true` if char still has an action this round
    fn unshake(&mut self) -> bool {
        let has_action = self.unshake_without_bennie();
        if has_action {
            return true;
        }
        if self.character.bennies.use_for_unshake.is_set() {
            self.unshake_with_bennie();
            return !self.shaken;
        }
        false
    }

    fn unshake_with_bennie(&mut self) {
        if !self.shaken || self.bennies == 0 {
            return;
        }
        self.bennies -= 1;
        self.shaken = false;
    }

    /// return true if action is still available
    #[must_use]
    fn unshake_without_bennie(&mut self) -> bool {
        if self.interrupted {
            self.interrupted = false;
            return false;
        }
        if !self.shaken {
            return true;
        }
        let mut roll = match roller().roll_attribute(self.character.attributes.wil) {
            Ok(roll) => roll,
            Err(RollError::CriticalFail) => return false,
            Err(RollError::Fail) => return false,
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

    fn unshake_against_step_back(&mut self) {
        if !self.character.bennies.use_against_step_back.is_set() {
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

    fn roll_attack_dice(&self, num_skill_dice: usize) -> Result<Vec<Roll>, RollError> {
        roller().roll_skill_with_n_dice(
            self.character.skills.kampfen,
            num_skill_dice,
            self.berserker,
        )
    }

    fn try_to_hit_without_bennie(&self, opponent: &Self, roll: Roll, modifier: i8) -> AttackResult {
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

        let mut roll = roll;
        roll += modifier;
        roll += i8::from(self.character.passive_modifiers.attack);
        self.apply_wound_penalty(&mut roll);
        Self::apply_gangup(opponent, &mut roll);
        self.apply_joker(&mut roll);
        self.apply_berserker_attack(&mut roll);
        if self.character.passive_modifiers.attack_head.is_set() {
            roll -= 4_u8;
        }
        self.apply_tuchfühlung_to_attack(opponent, &mut roll);
        roll -= opponent_parry;

        match roll.as_i8() {
            ..0 => AttackResult::Miss,
            0..4 => AttackResult::Hit,
            4.. => AttackResult::Raise,
        }
    }

    fn try_to_hit_with_bennie(
        &mut self,
        opponent: &Fighter,
        num_skill_dice: usize,
        modifier: i8,
    ) -> Result<Vec<AttackResult>, CriticalMiss> {
        let all_fail = (0..num_skill_dice).map(|_| AttackResult::Miss).collect();
        let rolls = match self.roll_attack_dice(num_skill_dice) {
            Ok(rolls) => rolls,
            Err(RollError::CriticalFail) => return Err(CriticalMiss),
            Err(RollError::Fail) => return Ok(all_fail),
        };
        let attacks: Vec<_> = rolls
            .into_iter()
            .map(|roll| self.try_to_hit_without_bennie(opponent, roll, modifier))
            .collect();

        let count_hits = attacks
            .iter()
            .filter(|attack| attack != &&AttackResult::Miss)
            .count()
            .try_into()
            .unwrap_or(u8::MAX);
        if count_hits == 0 && self.character.bennies.use_for_attack.is_set() && self.bennies > 0 {
            self.bennies -= 1;
            self.try_to_hit_with_bennie(opponent, num_skill_dice, modifier)
        } else {
            if let Some(stats) = self.fight_stats.as_ref() {
                stats.borrow_mut().add_hits_dealt(count_hits);
            }
            if let Some(stats) = opponent.fight_stats.as_ref() {
                stats.borrow_mut().add_hits_received(count_hits);
            }
            Ok(attacks)
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

    fn apply_opponent_berserker_rob(&self, opponent: &Self, damage: &mut Roll) {
        if !opponent.berserker {
            return;
        }
        *damage -= 2i8;
    }

    fn do_damage(
        &mut self,
        primary_weapon: bool,
        opponent: &mut Self,
        attack_result: AttackResult,
        modifier: i8,
        self_damage: bool,
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
            let more_crit = primary_weapon && self.character.weapon.more_crit.is_set()
                || !primary_weapon && self.character.secondary_weapon.more_crit.is_set();
            damage += if more_crit {
                roller().roll_raise_d10()
            } else {
                roller().roll_raise()
            };
        }
        self.apply_piercing(opponent, &mut damage);
        self.apply_berserker_damage(&mut damage);
        if self.character.edges.ubertolpeln.is_set() && opponent.shaken {
            damage += 4_u8;
        }
        damage += modifier;
        if self.character.passive_modifiers.attack_head.is_set() {
            damage += 6_u8;
        }
        self.apply_joker_to_damage(&mut damage);
        if u8::from(damage) < opponent.passive_stats.robustness {
            if !self_damage && self.character.bennies.use_for_damage.is_set() && self.bennies > 0 {
                self.bennies -= 1;
                self.do_damage(
                    primary_weapon,
                    opponent,
                    attack_result,
                    modifier,
                    self_damage,
                );
            }
            return;
        }

        damage -= opponent.passive_stats.robustness;
        self.apply_opponents_armor(opponent, &mut damage);
        self.apply_opponent_berserker_rob(opponent, &mut damage);
        opponent.passive_stats.life -= damage;
        opponent.shaken = true;
        opponent.enable_berserker();

        if let Some(stats) = self.fight_stats.as_ref() {
            stats.borrow_mut().add_damage_dealt(damage.into());
        }
        if let Some(stats) = opponent.fight_stats.as_ref() {
            stats.borrow_mut().add_damage_received(damage.into());
        }

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
                let modifier = if self.attacked_wild { 2 } else { 0 };
                tmp.do_damage(primary_weapon, self, AttackResult::Hit, modifier, true);
            }
            CriticalFailResult::HeavilyInjured => {
                let mut tmp = self.clone();
                let modifier = if self.attacked_wild { 2 } else { 0 };
                tmp.do_damage(primary_weapon, self, AttackResult::Raise, modifier, true);
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

#[derive(Debug, Default)]
pub struct DistanceMap {
    ids_left: Vec<u16>,
    ids_right: Vec<u16>,
    next_id: u16,
    map: HashMap<(u16, u16), bool>,
}

impl DistanceMap {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_key(fighter_group: Group, fighter_id: u16, opponent_id: u16) -> (u16, u16) {
        match fighter_group {
            Group::Left => (fighter_id, opponent_id),
            Group::Right => (opponent_id, fighter_id),
        }
    }

    fn register_fighter(&mut self, group: Group) -> u16 {
        let fighter_id = self.next_id;
        self.next_id += 1;

        let (own_group, other_group) = match group {
            Group::Left => (&mut self.ids_left, &mut self.ids_right),
            Group::Right => (&mut self.ids_right, &mut self.ids_left),
        };

        own_group.push(fighter_id);
        for other_id in other_group {
            let key = Self::get_key(group, fighter_id, *other_id);
            self.map.insert(key, false);
        }

        fighter_id
    }

    fn base_contact_mut(&mut self, fighter: &Fighter, opponent: &Fighter) -> &mut bool {
        let key = Self::get_key(fighter.group(), fighter.distance_id, opponent.distance_id);
        self.map
            .get_mut(&key)
            .expect("distance map entry should exist if fighter was registered")
    }

    fn base_contact(&self, fighter: &Fighter, opponent: &Fighter) -> bool {
        let key = Self::get_key(fighter.group(), fighter.distance_id, opponent.distance_id);
        *self
            .map
            .get(&key)
            .expect("distance map entry should exist if fighter was registered")
    }

    fn all_base_contacts(&self, fighter: &Fighter) -> impl Iterator<Item = bool> {
        self.map
            .iter()
            .filter(|(key, _val)| key.0 == fighter.distance_id || key.1 == fighter.distance_id)
            .map(|(_key, val)| *val)
    }
}
