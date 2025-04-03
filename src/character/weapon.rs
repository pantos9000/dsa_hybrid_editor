use strum::IntoEnumIterator;

use crate::simulator::Simulator;

use super::{Character, Drawable};

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Weapon {
    pub(crate) damage: Damage,
    pub(crate) bonus_damage: BonusDamage,
}

impl Drawable for Weapon {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("Waffe");

        ui.heading("Waffe");
        grid.show(ui, |ui| {
            self.damage.draw(sim, ui);
            ui.end_row();
            self.bonus_damage.draw(sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("OpponentWeapon");

        ui.heading("Waffe");
        grid.show(ui, |ui| {
            self.damage.draw_as_opponent(ui);
            ui.end_row();
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
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label("Schaden");
        ui.horizontal(|ui| {
            for val in Self::iter() {
                ui.selectable_value(self, val, val.as_str())
                    .on_hover_ui(|ui| {
                        ui.horizontal(|ui| {
                            let mod_set = Box::new(move |c: &mut Character| c.weapon.damage = val);
                            sim.gradient(mod_set).draw(ui);
                        });
                    });
            }
        });
        let mod_dec = Box::new(|c: &mut Character| c.weapon.damage.decrement());
        let mod_inc = Box::new(|c: &mut Character| c.weapon.damage.increment());
        ui.horizontal(|ui| {
            sim.gradient(mod_dec).draw(ui);
            sim.gradient(mod_inc).draw(ui);
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        ui.label("Schaden");
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
pub struct BonusDamage(i8);

impl From<i8> for BonusDamage {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl From<BonusDamage> for i8 {
    fn from(value: BonusDamage) -> Self {
        value.0
    }
}

impl BonusDamage {
    const MIN: i8 = -3;
    const MAX: i8 = 3;

    fn as_str(&self) -> &'static str {
        match self.0 {
            ..-3 => unreachable!(),
            4.. => unreachable!(),
            -3 => "-3",
            -2 => "-2",
            -1 => "-1",
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
        }
    }

    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label("Schadensbonus");

        let slider = egui::Slider::new(&mut self.0, Self::MIN..=Self::MAX);
        ui.add(slider);

        let mod_dec = Box::new(|c: &mut Character| c.weapon.bonus_damage.decrement());
        let mod_inc = Box::new(|c: &mut Character| c.weapon.bonus_damage.increment());
        ui.horizontal(|ui| {
            sim.gradient(mod_dec).draw(ui);
            sim.gradient(mod_inc).draw(ui);
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        ui.label("Schadensbonus");
        let _ = ui.button(self.as_str());
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
