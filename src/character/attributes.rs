use strum::{EnumCount, IntoEnumIterator};

use crate::util::{Named, Set};

use super::Drawable;

pub type Attributes = Set<Attribute>;

impl Named for Attribute {
    type Name = AttributeName;

    const NAME_COUNT: usize = AttributeName::COUNT;

    fn name_to_index(name: Self::Name) -> usize {
        match name {
            AttributeName::Ges => 0,
            AttributeName::Stä => 1,
            AttributeName::Kon => 2,
            AttributeName::Int => 3,
            AttributeName::Wil => 4,
        }
    }
}

impl Drawable for Attributes {
    fn draw(&mut self, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("Attribute");

        ui.heading("Attribute");
        grid.show(ui, |ui| {
            for aname in AttributeName::iter() {
                ui.label(aname.as_ref());
                self[aname].draw(ui);
                ui.end_row();
            }
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("OpponentAttributes");

        ui.heading("Attribute");
        grid.show(ui, |ui| {
            for aname in AttributeName::iter() {
                ui.label(aname.as_ref());
                self[aname].draw_as_opponent(ui);
                ui.end_row();
            }
        });
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumIter,
    strum_macros::AsRefStr,
    strum_macros::EnumCount,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum AttributeName {
    Ges,
    Stä,
    Kon,
    Int,
    Wil,
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
pub enum Attribute {
    #[default]
    W4,
    W6,
    W8,
    W10,
    W12,
    W12p1,
    W12p2,
}

impl Drawable for Attribute {
    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for val in Self::iter() {
                ui.selectable_value(self, val, val.as_str());
            }
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let _ = ui.button(self.as_str());
    }
}

impl Attribute {
    fn as_str(&self) -> &'static str {
        match self {
            Self::W4 => "W4",
            Self::W6 => "W6",
            Self::W8 => "W8",
            Self::W10 => "W10",
            Self::W12 => "W12",
            Self::W12p1 => "W12+1",
            Self::W12p2 => "W12+2",
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
            Self::W12p2 => Self::W12p2,
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
        };
        *self = new;
    }
}
