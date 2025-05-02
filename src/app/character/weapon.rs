use strum::IntoEnumIterator;

use crate::app::widgets::{self, DrawInfo, IntStat, ValueSelector, ValueSlider};
use crate::simulator::{CharModification, Simulator};

use super::Drawable;

#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Weapon<const SECONDARY: bool> {
    pub(crate) active: bool,
    pub(crate) damage: Damage,
    pub(crate) bonus_damage: IntStat<-2, 2>,
    pub(crate) piercing: IntStat<0, 3>,
    pub(crate) reach: IntStat<0, 2>,
}

impl<const SECONDARY: bool> Weapon<SECONDARY> {
    pub fn unarmed(&self) -> bool {
        self.damage == Damage::None
    }
}

impl<const SECONDARY: bool> Default for Weapon<SECONDARY> {
    fn default() -> Self {
        Self {
            active: !SECONDARY,
            damage: Damage::default(),
            bonus_damage: IntStat::default(),
            piercing: IntStat::default(),
            reach: IntStat::default(),
        }
    }
}

impl<const SECONDARY: bool> Drawable for Weapon<SECONDARY> {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let heading = self.heading(false);
        let grid = widgets::create_grid(heading);
        ui.heading(heading);
        grid.show(ui, |ui| {
            self.draw_active(sim, ui);
            ui.end_row();
            self.damage.draw(self.damage_name(), sim, ui);
            ui.end_row();
            self.bonus_damage
                .draw(ModifierName::<SECONDARY>::BonusDamage, sim, ui);
            ui.end_row();
            self.piercing
                .draw(ModifierName::<SECONDARY>::Piercing, sim, ui);
            ui.end_row();
            self.reach.draw(ModifierName::<SECONDARY>::Reach, sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let heading = self.heading(true);
        let grid = widgets::create_grid(heading);
        ui.heading(heading);
        grid.show(ui, |ui| {
            self.draw_active_as_opponent(ui);
            ui.end_row();
            self.damage.draw_as_opponent(self.damage_name(), ui);
            ui.end_row();
            self.bonus_damage
                .draw_as_opponent(ModifierName::<SECONDARY>::BonusDamage, ui);
            ui.end_row();
            self.piercing
                .draw_as_opponent(ModifierName::<SECONDARY>::Piercing, ui);
            ui.end_row();
            self.reach
                .draw_as_opponent(ModifierName::<SECONDARY>::Reach, ui);
            ui.end_row();
        });
    }
}

impl<const SECONDARY: bool> Weapon<SECONDARY> {
    fn heading(&self, opponent: bool) -> &'static str {
        match (SECONDARY, opponent) {
            (false, false) => "Hauptwaffe",
            (true, false) => "Zweitwaffe",
            (false, true) => "Gegner Hauptwaffe",
            (true, true) => "Gegner Zweitwaffe",
        }
    }

    fn damage_name(&self) -> DamageName {
        if SECONDARY {
            DamageName::Secondary
        } else {
            DamageName::Primary
        }
    }

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
        let active = if self.active { "Ja" } else { "Nein" };
        let _ = ui.button(active);
    }
}

pub enum DamageName {
    Primary,
    Secondary,
}

impl DrawInfo<Damage> for DamageName {
    fn as_str(&self) -> &'static str {
        "Schaden"
    }

    fn mod_dec(&self) -> CharModification {
        match self {
            DamageName::Primary => Box::new(|c| c.weapon.damage.decrement()),
            DamageName::Secondary => Box::new(|c| c.secondary_weapon.damage.decrement()),
        }
    }

    fn mod_inc(&self) -> CharModification {
        match self {
            DamageName::Primary => Box::new(|c| c.weapon.damage.increment()),
            DamageName::Secondary => Box::new(|c| c.secondary_weapon.damage.increment()),
        }
    }

    fn mod_set(&self, value: Damage) -> CharModification {
        match self {
            DamageName::Primary => Box::new(move |c| c.weapon.damage = value),
            DamageName::Secondary => Box::new(move |c| c.secondary_weapon.damage = value),
        }
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
    None,
    W4,
    W6,
    W8,
    W10,
    W12,
}

impl From<Damage> for u8 {
    fn from(value: Damage) -> Self {
        match value {
            Damage::None => 0,
            Damage::W4 => 4,
            Damage::W6 => 6,
            Damage::W8 => 8,
            Damage::W10 => 10,
            Damage::W12 => 12,
        }
    }
}

impl ValueSelector for Damage {
    type Info = DamageName;

    fn possible_values() -> impl Iterator<Item = Self> {
        Self::iter()
    }

    fn as_str(&self, _info: &Self::Info) -> &'static str {
        match self {
            Damage::None => "---",
            Damage::W4 => "W4",
            Damage::W6 => "W6",
            Damage::W8 => "W8",
            Damage::W10 => "W10",
            Damage::W12 => "W12",
        }
    }
}

impl Damage {
    fn decrement(&mut self) {
        let new = match self {
            Self::None => Self::None,
            Self::W4 => Self::None,
            Self::W6 => Self::W4,
            Self::W8 => Self::W6,
            Self::W10 => Self::W8,
            Self::W12 => Self::W10,
        };
        *self = new;
    }

    fn increment(&mut self) {
        let new = match self {
            Self::None => Self::W4,
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
enum ModifierName<const SECONDARY: bool> {
    BonusDamage,
    Piercing,
    Reach,
}

impl<const SECONDARY: bool, const MIN: i8, const MAX: i8> DrawInfo<IntStat<MIN, MAX>>
    for ModifierName<SECONDARY>
{
    fn as_str(&self) -> &'static str {
        match self {
            ModifierName::BonusDamage => "Bonusschaden",
            ModifierName::Piercing => "Panzerbrechend",
            ModifierName::Reach => "Reichweite",
        }
    }

    fn mod_dec(&self) -> CharModification {
        match (SECONDARY, self) {
            (false, Self::BonusDamage) => Box::new(|c| c.weapon.bonus_damage.decrement()),
            (false, Self::Piercing) => Box::new(|c| c.weapon.piercing.decrement()),
            (false, Self::Reach) => Box::new(|c| c.weapon.reach.decrement()),
            (true, Self::BonusDamage) => Box::new(|c| c.secondary_weapon.bonus_damage.decrement()),
            (true, Self::Piercing) => Box::new(|c| c.secondary_weapon.piercing.decrement()),
            (true, Self::Reach) => Box::new(|c| c.secondary_weapon.reach.decrement()),
        }
    }

    fn mod_inc(&self) -> CharModification {
        match (SECONDARY, self) {
            (false, Self::BonusDamage) => Box::new(|c| c.weapon.bonus_damage.increment()),
            (false, Self::Piercing) => Box::new(|c| c.weapon.piercing.increment()),
            (false, Self::Reach) => Box::new(|c| c.weapon.reach.increment()),
            (true, Self::BonusDamage) => Box::new(|c| c.secondary_weapon.bonus_damage.increment()),
            (true, Self::Piercing) => Box::new(|c| c.secondary_weapon.piercing.increment()),
            (true, Self::Reach) => Box::new(|c| c.secondary_weapon.reach.increment()),
        }
    }

    fn mod_set(&self, value: IntStat<MIN, MAX>) -> CharModification {
        let value = value.into();
        match (SECONDARY, self) {
            (false, Self::BonusDamage) => Box::new(move |c| c.weapon.bonus_damage.set(value)),
            (false, Self::Piercing) => Box::new(move |c| c.weapon.piercing.set(value)),
            (false, Self::Reach) => Box::new(move |c| c.weapon.reach.set(value)),
            (true, Self::BonusDamage) => {
                Box::new(move |c| c.secondary_weapon.bonus_damage.set(value))
            }
            (true, Self::Piercing) => Box::new(move |c| c.secondary_weapon.piercing.set(value)),
            (true, Self::Reach) => Box::new(move |c| c.secondary_weapon.reach.set(value)),
        }
    }
}
