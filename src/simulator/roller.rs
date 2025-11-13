use std::{cell::RefCell, rc::Rc};

use biski64::Biski64Rng as Rng;

use crate::app::character::{Attribute, Skill, Weapon};

std::thread_local! {
    static ROLLER: Rc<Roller> = Rc::new(Roller::new());
}

pub fn roller() -> Rc<Roller> {
    ROLLER.with(Rc::clone)
}

pub struct Roller {
    rng: RefCell<Rng>,
}

impl Default for Roller {
    fn default() -> Self {
        Self::new()
    }
}

impl Roller {
    pub fn new() -> Self {
        use rand::SeedableRng as _;
        let rng = Rng::seed_from_u64(rand::random());
        Self {
            rng: RefCell::new(rng),
        }
    }

    fn roll_die_nonexploding(&self, sides: u8) -> Roll {
        use rand::Rng as _; // for random_range()
        if sides == 0 {
            return Roll(0);
        }
        let result = self.rng.borrow_mut().random_range(1..=sides);
        let result = result.try_into().unwrap_or(i8::MAX);
        Roll(result)
    }

    fn roll_die(&self, sides: u8, modifier: i8) -> Roll {
        assert!(sides != 1, "side can't be 1, the result would be infinite");
        if sides == 0 {
            return Roll(0);
        }
        let mut result = Roll(0);
        loop {
            let roll = self.roll_die_nonexploding(sides);
            result += roll;
            if roll.as_u8() != sides {
                break;
            }
        }
        result + modifier
    }

    /// return `None` on crit fail
    fn roll_additional_wild_die(&self, old_result: Roll, sides: u8) -> Result<Roll, RollError> {
        let wild_die = self.roll_die(sides, 0);
        if old_result.as_u8() < 2 && wild_die.as_u8() < 2 {
            return Err(RollError::CriticalFail);
        }
        Ok(wild_die.max(old_result))
    }

    pub fn roll_raise(&self) -> Roll {
        self.roll_die(6, 0)
    }

    pub fn roll_raise_d10(&self) -> Roll {
        self.roll_die(10, 0)
    }

    /// non-exploding 2d6
    pub fn roll_critical_fail_result(&self) -> Roll {
        self.roll_die_nonexploding(6) + self.roll_die_nonexploding(6)
    }

    pub fn roll_attribute_without_wild_die(&self, attribute: Attribute) -> Roll {
        let (sides, modifier) = match attribute {
            Attribute::W4 => (4, 0),
            Attribute::W6 => (6, 0),
            Attribute::W8 => (8, 0),
            Attribute::W10 => (10, 0),
            Attribute::W12 => (12, 0),
            Attribute::W12p1 => (12, 1),
            Attribute::W12p2 => (12, 2),
            Attribute::Master => (12, 2),
        };
        self.roll_die(sides, modifier)
    }

    pub fn roll_attribute(&self, attribute: Attribute) -> Result<Roll, RollError> {
        let roll = self.roll_attribute_without_wild_die(attribute);
        self.roll_additional_wild_die(roll, attribute.wild_die_sides())
    }

    pub fn roll_skill_with_n_dice(
        &self,
        skill: Skill,
        n: usize,
        fail_on_one: bool,
    ) -> Result<Vec<Roll>, RollError> {
        assert!(n > 0);
        let (sides, modifier) = match skill {
            Skill::W4m2 => (4, -2),
            Skill::W4 => (4, 0),
            Skill::W6 => (6, 0),
            Skill::W8 => (8, 0),
            Skill::W10 => (10, 0),
            Skill::W12 => (12, 0),
            Skill::W12p1 => (12, 1),
            Skill::W12p2 => (12, 2),
            Skill::Master => (12, 2),
        };
        let mut rolls: Vec<_> = (0..n).map(|_| self.roll_die(sides, modifier)).collect();
        if fail_on_one && rolls.iter().any(|roll| roll.as_u8() == 1) {
            return Err(RollError::Fail);
        }
        let minimum = rolls.iter_mut().min().unwrap();
        *minimum = self.roll_additional_wild_die(*minimum, skill.wild_die_sides())?;
        Ok(rolls)
    }

    /// roll weapon damage, but cap die sides by strength die
    pub fn roll_weapon_damage<const SECONDARY: bool>(
        &self,
        weapon: &Weapon<SECONDARY>,
        strength: Attribute,
    ) -> Roll {
        let damage_sides: u8 = weapon.damage.into();
        let strength_sides = strength.into();
        let sides = damage_sides.min(strength_sides);
        let modifier = weapon.bonus_damage.into();
        self.roll_die(sides, modifier)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Roll(i8);

impl Roll {
    /// compares against 0/4/8
    pub fn eval(self) -> RollResult {
        self.eval_against(4)
    }

    pub fn eval_against(self, value: u8) -> RollResult {
        let mut roll = self + 1_i8; // add 1 to be able to check against 0
        roll -= value;
        match roll.0 {
            ..=0 => RollResult::Fail,
            1..=4 => RollResult::Success,
            5.. => RollResult::Raise,
        }
    }

    pub fn as_u8(self) -> u8 {
        self.into()
    }

    pub fn as_i8(self) -> i8 {
        self.into()
    }
}

impl From<Roll> for i8 {
    fn from(value: Roll) -> Self {
        value.0
    }
}

impl From<Roll> for u8 {
    fn from(value: Roll) -> Self {
        // internally, a roll can be negative, but not externally
        value.0.try_into().unwrap_or(0)
    }
}

impl std::ops::Add<Roll> for Roll {
    type Output = Self;

    fn add(self, rhs: Roll) -> Self::Output {
        Roll(self.0.saturating_add(rhs.0))
    }
}

impl std::ops::AddAssign<Roll> for Roll {
    fn add_assign(&mut self, rhs: Roll) {
        *self = *self + rhs;
    }
}

impl std::ops::Add<u8> for Roll {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        let rhs: i8 = rhs.try_into().unwrap_or(i8::MAX);
        Roll(self.0.saturating_add(rhs))
    }
}

impl std::ops::AddAssign<u8> for Roll {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<u8> for Roll {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        let rhs: i8 = rhs.try_into().unwrap_or(i8::MAX);
        Roll(self.0.saturating_sub(rhs))
    }
}

impl std::ops::SubAssign<u8> for Roll {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<u8> for Roll {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        let rhs: i8 = rhs.try_into().unwrap_or(i8::MAX);
        Roll(self.0.saturating_mul(rhs))
    }
}

impl std::ops::MulAssign<u8> for Roll {
    fn mul_assign(&mut self, rhs: u8) {
        *self = *self * rhs;
    }
}

impl std::ops::Add<i8> for Roll {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        Roll(self.0.saturating_add(rhs))
    }
}

impl std::ops::AddAssign<i8> for Roll {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<i8> for Roll {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        Roll(self.0.saturating_sub(rhs))
    }
}

impl std::ops::SubAssign<i8> for Roll {
    fn sub_assign(&mut self, rhs: i8) {
        *self = *self - rhs;
    }
}

impl std::ops::Add<Roll> for u8 {
    type Output = Self;

    fn add(self, rhs: Roll) -> Self::Output {
        let roll = u8::from(rhs);
        self + roll
    }
}

impl std::ops::AddAssign<Roll> for u8 {
    fn add_assign(&mut self, rhs: Roll) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Roll> for u8 {
    type Output = Self;

    fn sub(self, rhs: Roll) -> Self::Output {
        let roll = u8::from(rhs);
        self.saturating_sub(roll)
    }
}

impl std::ops::SubAssign<Roll> for u8 {
    fn sub_assign(&mut self, rhs: Roll) {
        *self = *self - rhs;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RollError {
    CriticalFail,
    Fail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollResult {
    Fail,
    Success,
    Raise,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_eval() {
        assert_eq!(Roll(-1).eval(), RollResult::Fail);
        assert_eq!(Roll(0).eval(), RollResult::Fail);
        assert_eq!(Roll(1).eval(), RollResult::Fail);
        assert_eq!(Roll(2).eval(), RollResult::Fail);
        assert_eq!(Roll(3).eval(), RollResult::Fail);
        assert_eq!(Roll(4).eval(), RollResult::Success);
        assert_eq!(Roll(5).eval(), RollResult::Success);
        assert_eq!(Roll(6).eval(), RollResult::Success);
        assert_eq!(Roll(7).eval(), RollResult::Success);
        assert_eq!(Roll(8).eval(), RollResult::Raise);
        assert_eq!(Roll(9).eval(), RollResult::Raise);
        assert_eq!(Roll(i8::MAX).eval(), RollResult::Raise);
    }

    #[test]
    fn test_roll_eval_against() {
        assert_eq!(Roll(7).eval_against(8), RollResult::Fail);
        assert_eq!(Roll(8).eval_against(8), RollResult::Success);
        assert_eq!(Roll(9).eval_against(8), RollResult::Success);
        assert_eq!(Roll(10).eval_against(8), RollResult::Success);
        assert_eq!(Roll(11).eval_against(8), RollResult::Success);
        assert_eq!(Roll(12).eval_against(8), RollResult::Raise);
        assert_eq!(Roll(13).eval_against(8), RollResult::Raise);
    }

    #[test]
    fn test_roll_arithmetic_assign() {
        let mut roll = Roll(2);
        roll += 1_u8;
        assert_eq!(roll.0, 3);
        roll += 1_i8;
        assert_eq!(roll.0, 4);
        roll -= 1_u8;
        assert_eq!(roll.0, 3);
        roll -= 1_i8;
        assert_eq!(roll.0, 2);
    }

    #[test]
    fn test_roll_arithmetic() {
        let roll = Roll(3);
        assert_eq!(roll + 2_u8, Roll(5));
        assert_eq!(roll + 2_i8, Roll(5));
        assert_eq!(roll - 2_u8, Roll(1));
        assert_eq!(roll - 2_i8, Roll(1));
    }

    #[test]
    fn test_roll_arithmetic_around_zero() {
        assert_eq!(Roll(1) - 2_u8, Roll(-1));
        assert_eq!(Roll(1) - 2_i8, Roll(-1));
        assert_eq!(Roll(1) + (-2_i8), Roll(-1));
        assert_eq!(Roll(-1) + (2_u8), Roll(1));
        assert_eq!(Roll(-1) + (2_i8), Roll(1));
        assert_eq!(Roll(-1) - (-2_i8), Roll(1));
    }

    #[test]
    fn test_roll_arithmetic_assign_around_zero() {
        let mut roll = Roll(1);
        roll -= 2_u8;
        assert_eq!(roll.0, -1);
        roll += 2_u8;
        assert_eq!(roll.0, 1);
        roll -= 2_i8;
        assert_eq!(roll.0, -1);
        roll += 2_i8;
        assert_eq!(roll.0, 1);
    }
}
