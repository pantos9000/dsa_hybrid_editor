use crate::app::widgets::{DrawInfo, IntStat, ValueSlider as _};
use crate::util;

use super::Drawable;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Armor {
    pub(crate) torso: IntStat<0, 5>,
}

impl Drawable for Armor {
    fn draw(&mut self, sim: &crate::simulator::Simulator, ui: &mut egui::Ui) {
        let grid = util::create_grid("Rüstung");

        ui.heading("Rüstung");
        grid.show(ui, |ui| {
            self.torso.draw(ArmorInfo::Torso, sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = util::create_grid("GegnerRüstung");

        ui.heading("Rüstung");
        grid.show(ui, |ui| {
            self.torso.draw_as_opponent(ArmorInfo::Torso, ui);
            ui.end_row();
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display)]
enum ArmorInfo {
    Torso,
}

impl DrawInfo<IntStat<0, 5>> for ArmorInfo {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Torso => "Torso",
        }
    }

    fn mod_dec(&self) -> crate::simulator::CharModification {
        match self {
            Self::Torso => Box::new(|c| c.armor.torso.decrement()),
        }
    }

    fn mod_inc(&self) -> crate::simulator::CharModification {
        match self {
            Self::Torso => Box::new(|c| c.armor.torso.increment()),
        }
    }

    fn mod_set(&self, value: IntStat<0, 5>) -> crate::simulator::CharModification {
        match self {
            Self::Torso => Box::new(move |c| c.armor.torso.set(value.into())),
        }
    }
}
