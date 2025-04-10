use strum::IntoEnumIterator;

use crate::{
    simulator::{CharModification, Simulator},
    util::{Modifier, ModifierName},
};

use super::{Character, Drawable};

#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Weapon<const SECONDARY: bool> {
    pub(crate) active: bool,
    pub(crate) damage: Damage,
    pub(crate) bonus_damage: Modifier<-2, 2>,
    pub(crate) piercing: Modifier<0, 3>,
    pub(crate) reach: Modifier<0, 2>,
}

impl<const SECONDARY: bool> Default for Weapon<SECONDARY> {
    fn default() -> Self {
        Self {
            active: !SECONDARY,
            damage: Default::default(),
            bonus_damage: Default::default(),
            piercing: Default::default(),
            reach: Default::default(),
        }
    }
}

impl<const SECONDARY: bool> Drawable for Weapon<SECONDARY> {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let name = if SECONDARY {
            "Zweitwaffe"
        } else {
            "Hauptwaffe"
        };
        let grid = crate::util::create_grid(name);

        ui.heading(name);
        grid.show(ui, |ui| {
            self.draw_active(sim, ui);
            ui.end_row();
            self.damage.draw(sim, ui);
            ui.end_row();
            self.bonus_damage.draw(WeaponModifier::BonusDamage, sim, ui);
            ui.end_row();
            self.piercing.draw(WeaponModifier::Piercing, sim, ui);
            ui.end_row();
            self.reach.draw(WeaponModifier::Reach, sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let name = if SECONDARY {
            "Gegner Zweitwaffe"
        } else {
            "Gegner Hauptwaffe"
        };
        let grid = crate::util::create_grid(name);

        ui.heading(name);
        grid.show(ui, |ui| {
            self.draw_active_as_opponent(ui);
            ui.end_row();
            self.damage.draw_as_opponent(ui);
            ui.end_row();
            self.bonus_damage
                .draw_as_opponent(WeaponModifier::BonusDamage, ui);
            ui.end_row();
            self.piercing.draw_as_opponent(WeaponModifier::Piercing, ui);
            ui.end_row();
            self.reach.draw_as_opponent(WeaponModifier::Reach, ui);
            ui.end_row();
        });
    }
}

impl<const SECONDARY: bool> Weapon<SECONDARY> {
    fn draw_active(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let mod_dec: CharModification;
        let mod_inc: CharModification;
        let mod_toggle: CharModification;

        if SECONDARY {
            mod_dec = Box::new(|c| c.secondary_weapon.active = false);
            mod_inc = Box::new(|c| c.secondary_weapon.active = true);
            mod_toggle = Box::new(|c| c.secondary_weapon.active = !c.weapon.active);
        } else {
            mod_dec = Box::new(|c| c.weapon.active = false);
            mod_inc = Box::new(|c| c.weapon.active = true);
            mod_toggle = Box::new(|c| c.weapon.active = !c.weapon.active);
        }
        ui.checkbox(&mut self.active, "Aktiv").on_hover_ui(|ui| {
            ui.horizontal(|ui| {
                sim.gradient(mod_toggle).draw(ui);
            });
        });

        ui.horizontal(|ui| {
            sim.gradient(mod_dec).draw(ui);
            sim.gradient(mod_inc).draw(ui);
        });
    }

    fn draw_active_as_opponent(&mut self, ui: &mut egui::Ui) {
        ui.label("Aktiv");
        let active = match self.active {
            true => "Ja",
            false => "Nein",
        };
        let _ = ui.button(active);
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
enum WeaponModifier {
    BonusDamage,
    Piercing,
    Reach,
}

impl ModifierName for WeaponModifier {
    fn as_str(&self) -> &str {
        match self {
            WeaponModifier::BonusDamage => "Schadensbonus",
            WeaponModifier::Piercing => "Panzerbrechend",
            WeaponModifier::Reach => "Reichweite",
        }
    }

    fn modification_dec(&self) -> CharModification {
        match self {
            WeaponModifier::BonusDamage => Box::new(|c| c.weapon.bonus_damage.decrement()),
            WeaponModifier::Piercing => Box::new(|c| c.weapon.piercing.decrement()),
            WeaponModifier::Reach => Box::new(|c| c.weapon.reach.decrement()),
        }
    }

    fn modification_inc(&self) -> CharModification {
        match self {
            WeaponModifier::BonusDamage => Box::new(|c| c.weapon.bonus_damage.increment()),
            WeaponModifier::Piercing => Box::new(|c| c.weapon.piercing.increment()),
            WeaponModifier::Reach => Box::new(|c| c.weapon.reach.increment()),
        }
    }
}
