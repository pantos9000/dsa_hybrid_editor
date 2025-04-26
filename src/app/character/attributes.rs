use crate::app::widgets::{DrawInfo, ValueSelector};
use crate::simulator::{CharModification, Simulator};

use super::Drawable;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Attributes {
    pub(crate) ges: Attribute,
    pub(crate) sta: Attribute,
    pub(crate) kon: Attribute,
    pub(crate) int: Attribute,
    pub(crate) wil: Attribute,
}

impl Drawable for Attributes {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("Attribute");

        ui.heading("Attribute");
        grid.show(ui, |ui| {
            self.ges.draw(AttrName::Ges, sim, ui);
            ui.end_row();
            self.sta.draw(AttrName::Stä, sim, ui);
            ui.end_row();
            self.kon.draw(AttrName::Kon, sim, ui);
            ui.end_row();
            self.int.draw(AttrName::Int, sim, ui);
            ui.end_row();
            self.wil.draw(AttrName::Wil, sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("GegnerAttribute");

        ui.heading("Attribute");
        grid.show(ui, |ui| {
            self.ges.draw_as_opponent(AttrName::Ges, ui);
            ui.end_row();
            self.sta.draw_as_opponent(AttrName::Stä, ui);
            ui.end_row();
            self.kon.draw_as_opponent(AttrName::Kon, ui);
            ui.end_row();
            self.int.draw_as_opponent(AttrName::Int, ui);
            ui.end_row();
            self.wil.draw_as_opponent(AttrName::Wil, ui);
            ui.end_row();
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrName {
    Ges,
    Stä,
    Kon,
    Int,
    Wil,
}

impl DrawInfo<Attribute> for AttrName {
    fn as_str(&self) -> &'static str {
        match self {
            AttrName::Ges => "Ges",
            AttrName::Stä => "Stä",
            AttrName::Kon => "Kon",
            AttrName::Int => "Int",
            AttrName::Wil => "Wil",
        }
    }

    fn mod_dec(&self) -> CharModification {
        match self {
            AttrName::Ges => Box::new(|c| c.attributes.ges.decrement()),
            AttrName::Stä => Box::new(|c| c.attributes.sta.decrement()),
            AttrName::Kon => Box::new(|c| c.attributes.kon.decrement()),
            AttrName::Int => Box::new(|c| c.attributes.int.decrement()),
            AttrName::Wil => Box::new(|c| c.attributes.wil.decrement()),
        }
    }

    fn mod_inc(&self) -> CharModification {
        match self {
            AttrName::Ges => Box::new(|c| c.attributes.ges.increment()),
            AttrName::Stä => Box::new(|c| c.attributes.sta.increment()),
            AttrName::Kon => Box::new(|c| c.attributes.kon.increment()),
            AttrName::Int => Box::new(|c| c.attributes.int.increment()),
            AttrName::Wil => Box::new(|c| c.attributes.wil.increment()),
        }
    }

    fn mod_set(&self, value: Attribute) -> CharModification {
        match self {
            AttrName::Ges => Box::new(move |c| c.attributes.ges = value),
            AttrName::Stä => Box::new(move |c| c.attributes.sta = value),
            AttrName::Kon => Box::new(move |c| c.attributes.kon = value),
            AttrName::Int => Box::new(move |c| c.attributes.int = value),
            AttrName::Wil => Box::new(move |c| c.attributes.wil = value),
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
pub enum Attribute {
    #[default]
    W4,
    W6,
    W8,
    W10,
    W12,
    W12p1,
    W12p2,
    Master,
}

impl From<Attribute> for u8 {
    fn from(attribute: Attribute) -> Self {
        match attribute {
            Attribute::W4 => 4,
            Attribute::W6 => 6,
            Attribute::W8 => 8,
            Attribute::W10 => 10,
            Attribute::W12 => 12,
            Attribute::W12p1 => 13,
            Attribute::W12p2 => 14,
            Attribute::Master => 14,
        }
    }
}

impl ValueSelector for Attribute {
    type Info = AttrName;

    fn possible_values() -> impl Iterator<Item = Self> {
        use strum::IntoEnumIterator as _;

        Self::iter()
    }

    fn as_str(&self, _info: &Self::Info) -> &'static str {
        match self {
            Self::W4 => "W4",
            Self::W6 => "W6",
            Self::W8 => "W8",
            Self::W10 => "W10",
            Self::W12 => "W12",
            Self::W12p1 => "W12+1",
            Self::W12p2 => "W12+2",
            Self::Master => "Meister",
        }
    }
}

impl Attribute {
    pub fn wild_die_sides(self) -> u8 {
        match self {
            Self::Master => 10,
            _ => 6,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn increment(&mut self) {
        let new = match self {
            Self::W4 => Self::W6,
            Self::W6 => Self::W8,
            Self::W8 => Self::W10,
            Self::W10 => Self::W12,
            Self::W12 => Self::W12p1,
            Self::W12p1 => Self::W12p2,
            Self::W12p2 => Self::Master,
            Self::Master => Self::Master,
        };
        *self = new;
    }

    #[allow(dead_code)]
    pub(crate) fn decrement(&mut self) {
        let new = match self {
            Self::W4 => Self::W4,
            Self::W6 => Self::W4,
            Self::W8 => Self::W6,
            Self::W10 => Self::W8,
            Self::W12 => Self::W10,
            Self::W12p1 => Self::W12,
            Self::W12p2 => Self::W12p1,
            Self::Master => Self::W12p2,
        };
        *self = new;
    }
}
