use strum::IntoEnumIterator;

use crate::simulator::{CharModification, Simulator};

use super::{Character, Drawable};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Attributes {
    pub(crate) ges: Attribute,
    pub(crate) stä: Attribute,
    pub(crate) kon: Attribute,
    pub(crate) int: Attribute,
    pub(crate) wil: Attribute,
}

impl Drawable for Attributes {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("Attribute");

        ui.heading("Attribute");
        grid.show(ui, |ui| {
            let mut draw =
                |attr: &mut Attribute, name, mod1: CharModification, mod2: CharModification| {
                    ui.label(name);
                    attr.draw(ui);
                    ui.horizontal(|ui| {
                        sim.gradient(mod1).draw(ui);
                        sim.gradient(mod2).draw(ui);
                    });
                    ui.end_row();
                };

            let mod_ges_dec = Box::new(|c: &mut Character| c.attributes.ges.decrement());
            let mod_stä_dec = Box::new(|c: &mut Character| c.attributes.stä.decrement());
            let mod_kon_dec = Box::new(|c: &mut Character| c.attributes.kon.decrement());
            let mod_int_dec = Box::new(|c: &mut Character| c.attributes.int.decrement());
            let mod_wil_dec = Box::new(|c: &mut Character| c.attributes.wil.decrement());
            let mod_ges_inc = Box::new(|c: &mut Character| c.attributes.ges.increment());
            let mod_stä_inc = Box::new(|c: &mut Character| c.attributes.stä.increment());
            let mod_kon_inc = Box::new(|c: &mut Character| c.attributes.kon.increment());
            let mod_int_inc = Box::new(|c: &mut Character| c.attributes.int.increment());
            let mod_wil_inc = Box::new(|c: &mut Character| c.attributes.wil.increment());

            draw(&mut self.ges, "Ges", mod_ges_dec, mod_ges_inc);
            draw(&mut self.stä, "Stä", mod_stä_dec, mod_stä_inc);
            draw(&mut self.kon, "Kon", mod_kon_dec, mod_kon_inc);
            draw(&mut self.int, "Int", mod_int_dec, mod_int_inc);
            draw(&mut self.wil, "Wil", mod_wil_dec, mod_wil_inc);
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let draw = |attr: &mut Attribute, name, ui: &mut egui::Ui| {
            ui.label(name);
            attr.draw_as_opponent(ui);
            ui.end_row();
        };

        let grid = crate::util::create_grid("OpponentAttributes");

        ui.heading("Attribute");
        grid.show(ui, |ui| {
            draw(&mut self.ges, "Ges", ui);
            draw(&mut self.stä, "Stä", ui);
            draw(&mut self.kon, "Kon", ui);
            draw(&mut self.int, "Int", ui);
            draw(&mut self.wil, "Wil", ui);
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

impl Attribute {
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
