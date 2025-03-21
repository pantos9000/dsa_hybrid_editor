use strum::IntoEnumIterator;

use crate::simulator::Simulator;

use super::{Character, Drawable};

// use crate::simulator::Gradient;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Weapon {
    damage: Damage,
    bonus_damage: BonusDamage,
}

impl Drawable for Weapon {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("Waffe");

        ui.heading("Waffe");
        grid.show(ui, |ui| {
            let mod_schaden = |c: &mut Character| c.weapon.damage.increment();
            let mod_bonus = |c: &mut Character| c.weapon.bonus_damage.increment();

            ui.label("Schaden");
            self.damage.draw(ui);
            sim.gradient(Box::new(mod_schaden)).draw(ui);
            ui.end_row();

            ui.label("Schadensbonus");
            self.bonus_damage.draw(ui);
            sim.gradient(Box::new(mod_bonus)).draw(ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("OpponentWeapon");

        ui.heading("Waffe");
        grid.show(ui, |ui| {
            ui.label("Schaden");
            self.damage.draw_as_opponent(ui);
            ui.end_row();

            ui.label("Schadensbonus");
            self.bonus_damage.draw_as_opponent(ui);
            ui.end_row();
        });
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Damage {
    #[default]
    W4,
    W6,
    W8,
    W10,
    W12,
}

impl Damage {
    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for val in Self::iter() {
                ui.selectable_value(self, val, val.as_str());
            }
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        // TODO small button?
        let _ = ui.button(self.as_str());
    }

    fn as_str(&self) -> &'static str {
        match self {
            Damage::W4 => "W4",
            Damage::W6 => "W6",
            Damage::W8 => "W8",
            Damage::W10 => "W10",
            Damage::W12 => "W12",
        }
    }

    #[allow(dead_code)]
    fn decrement(&mut self) {
        let new = match self {
            Self::W4 => Self::W4,
            Self::W6 => Self::W4,
            Self::W8 => Self::W6,
            Self::W10 => Self::W8,
            Self::W12 => Self::W10,
        };
        *self = new;
    }

    #[allow(dead_code)]
    fn increment(&mut self) {
        let new = match self {
            Self::W4 => Self::W6,
            Self::W6 => Self::W8,
            Self::W8 => Self::W10,
            Self::W10 => Self::W12,
            Self::W12 => Self::W12,
        };
        *self = new;
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct BonusDamage(i32);

impl From<i32> for BonusDamage {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<BonusDamage> for i32 {
    fn from(value: BonusDamage) -> Self {
        value.0
    }
}

impl BonusDamage {
    const MIN: i32 = -3;
    const MAX: i32 = 3;

    fn draw(&mut self, ui: &mut egui::Ui) {
        let slider = egui::Slider::new(&mut self.0, Self::MIN..=Self::MAX);
        ui.add(slider);
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let _ = ui.button(format!("{}", self.0));
    }

    #[allow(dead_code)]
    fn decrement(&mut self) {
        let new = match self.0 {
            val if val < Self::MIN => unreachable!(),
            val if val > Self::MAX => unreachable!(),
            Self::MIN => Self::MIN,
            val => val - 1,
        };
        self.0 = new;
    }

    #[allow(dead_code)]
    fn increment(&mut self) {
        let new = match self.0 {
            val if val < Self::MIN => unreachable!(),
            val if val > Self::MAX => unreachable!(),
            Self::MAX => Self::MAX,
            val => val + 1,
        };
        self.0 = new;
    }
}
