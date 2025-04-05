use strum::IntoEnumIterator;

use crate::simulator::{CharModification, Simulator};

use super::{Character, Drawable};

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Weapon {
    pub(crate) damage: Damage,
    pub(crate) bonus_damage: Modifier<-2, 2>,
    pub(crate) bonus_parry: Modifier<-2, 2>,
    pub(crate) reach: Modifier<0, 2>,
}

impl Drawable for Weapon {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("Waffe");

        ui.heading("Waffe");
        grid.show(ui, |ui| {
            self.damage.draw(sim, ui);
            ui.end_row();
            self.bonus_damage.draw(ModifierName::BonusDamage, sim, ui);
            ui.end_row();
            self.bonus_parry.draw(ModifierName::BonusParry, sim, ui);
            ui.end_row();
            self.reach.draw(ModifierName::Reach, sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("OpponentWeapon");

        ui.heading("Waffe");
        grid.show(ui, |ui| {
            self.damage.draw_as_opponent(ui);
            ui.end_row();
            self.bonus_damage
                .draw_as_opponent(ModifierName::BonusDamage, ui);
            ui.end_row();
            self.bonus_parry
                .draw_as_opponent(ModifierName::BonusParry, ui);
            ui.end_row();
            self.reach.draw_as_opponent(ModifierName::Reach, ui);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModifierName {
    BonusDamage,
    BonusParry,
    Reach,
}

impl ModifierName {
    fn as_str(&self) -> &'static str {
        match self {
            ModifierName::BonusDamage => "Schadensbonus",
            ModifierName::BonusParry => "Paradebonus",
            ModifierName::Reach => "Reichweite",
        }
    }

    fn modification_dec(&self) -> CharModification {
        match self {
            ModifierName::BonusDamage => Box::new(|c| c.weapon.bonus_damage.decrement()),
            ModifierName::BonusParry => Box::new(|c| c.weapon.bonus_parry.decrement()),
            ModifierName::Reach => Box::new(|c| c.weapon.reach.decrement()),
        }
    }

    fn modification_inc(&self) -> CharModification {
        match self {
            ModifierName::BonusDamage => Box::new(|c| c.weapon.bonus_damage.increment()),
            ModifierName::BonusParry => Box::new(|c| c.weapon.bonus_parry.increment()),
            ModifierName::Reach => Box::new(|c| c.weapon.reach.increment()),
        }
    }

    #[allow(dead_code)]
    fn modification_set<const MIN: i8, const MAX: i8>(
        &self,
        value: Modifier<MIN, MAX>,
    ) -> CharModification {
        match self {
            ModifierName::BonusDamage => Box::new(move |c| c.weapon.bonus_damage.0 = value.0),
            ModifierName::BonusParry => Box::new(move |c| c.weapon.bonus_parry.0 = value.0),
            ModifierName::Reach => Box::new(move |c| c.weapon.reach.0 = value.0),
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Modifier<const MIN: i8, const MAX: i8>(i8);

impl<const MIN: i8, const MAX: i8> From<i8> for Modifier<MIN, MAX> {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl<const MIN: i8, const MAX: i8> From<Modifier<MIN, MAX>> for i8 {
    fn from(value: Modifier<MIN, MAX>) -> Self {
        value.0
    }
}

impl<const MIN: i8, const MAX: i8> Modifier<MIN, MAX> {
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

    fn draw(&mut self, name: ModifierName, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label(name.as_str());

        let slider = egui::Slider::new(&mut self.0, MIN..=MAX);
        ui.add(slider);

        ui.horizontal(|ui| {
            sim.gradient(name.modification_dec()).draw(ui);
            sim.gradient(name.modification_inc()).draw(ui);
        });
    }

    fn draw_as_opponent(&mut self, name: ModifierName, ui: &mut egui::Ui) {
        ui.label(name.as_str());
        let _ = ui.button(self.as_str());
    }

    fn decrement(&mut self) {
        let new = match self.0 {
            val if val < MIN => unreachable!(),
            val if val > MAX => unreachable!(),
            val if val == MIN => MIN,
            val => val - 1,
        };
        self.0 = new;
    }

    fn increment(&mut self) {
        let new = match self.0 {
            val if val < MIN => unreachable!(),
            val if val > MAX => unreachable!(),
            val if val == MAX => MAX,
            val => val + 1,
        };
        self.0 = new;
    }
}
