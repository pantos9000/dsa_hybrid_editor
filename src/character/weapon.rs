use strum::IntoEnumIterator;

use crate::simulator::Gradient;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Weapon {
    damage: Damage,
    #[serde(skip)]
    damage_gradient: Gradient,
    bonus_damage: BonusDamage,
    #[serde(skip)]
    bonus_gradient: Gradient,
}

impl crate::app::Drawable for Weapon {
    fn draw(&mut self, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("Waffe");

        ui.heading("Waffe");
        grid.show(ui, |ui| {
            ui.label("Schaden");
            self.damage.draw(ui);
            self.damage_gradient.draw(ui);
            ui.end_row();

            ui.label("Schadensbonus");
            self.bonus_damage.draw(ui);
            self.bonus_gradient.draw(ui);
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

impl crate::app::Drawable for Damage {
    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for val in Self::iter() {
                ui.selectable_value(self, val, val.as_str());
            }
        });
    }
}

impl Damage {
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

impl crate::app::Drawable for BonusDamage {
    fn draw(&mut self, ui: &mut egui::Ui) {
        let slider = egui::Slider::new(&mut self.0, Self::MIN..=Self::MAX);
        ui.add(slider);
    }
}

impl BonusDamage {
    const MIN: i32 = -3;
    const MAX: i32 = 3;

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
