use crate::{simulator::Simulator, util};

use super::{Character, Drawable};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Armor {
    pub(crate) torso: ArmorPiece,
}

impl Drawable for Armor {
    fn draw(&mut self, sim: &crate::simulator::Simulator, ui: &mut egui::Ui) {
        let grid = util::create_grid("Rüstung");

        ui.heading("Rüstung");
        grid.show(ui, |ui| {
            self.torso.draw(ArmorName::Torso, sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = util::create_grid("GegnerRüstung");

        ui.heading("Rüstung");
        grid.show(ui, |ui| {
            self.torso.draw_as_opponent(ArmorName::Torso, ui);
            ui.end_row();
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display)]
enum ArmorName {
    Torso,
}

impl ArmorName {
    fn as_str(&self) -> &'static str {
        match self {
            ArmorName::Torso => "Torso",
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct ArmorPiece(u8);

impl From<ArmorPiece> for u8 {
    fn from(armor: ArmorPiece) -> Self {
        armor.0
    }
}

impl ArmorPiece {
    const MIN: u8 = 0;
    const MAX: u8 = 5;

    pub(crate) fn increment(&mut self) {
        let new = self.0.saturating_add(1);
        self.0 = new.clamp(Self::MIN, Self::MAX);
    }

    pub(crate) fn decrement(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }

    fn as_str(&self) -> &'static str {
        match self.0 {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            _ => unreachable!(),
        }
    }

    fn draw(&mut self, name: ArmorName, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label(name.as_str());

        let slider = egui::Slider::new(&mut self.0, Self::MIN..=Self::MAX);
        ui.add(slider);

        let mod_dec = Box::new(|c: &mut Character| c.armor.torso.decrement());
        let mod_inc = Box::new(|c: &mut Character| c.armor.torso.increment());
        ui.horizontal(|ui| {
            sim.gradient(mod_dec).draw(ui);
            sim.gradient(mod_inc).draw(ui);
        });
    }

    fn draw_as_opponent(&mut self, name: ArmorName, ui: &mut egui::Ui) {
        ui.label(format!("{name}"));
        let _ = ui.button(self.as_str());
    }
}
