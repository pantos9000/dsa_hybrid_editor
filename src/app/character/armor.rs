use crate::app::widgets::{self, DrawInfo, IntStat, ValueSlider as _};

use super::Drawable;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Armor {
    pub(crate) torso: IntStat<0, 5>,
    #[serde(default)]
    pub(crate) head: IntStat<0, 5>,
}

impl Drawable for Armor {
    fn draw(&mut self, sim: &crate::simulator::Simulator, ui: &mut egui::Ui) {
        let grid = widgets::create_grid("Rüstung");

        ui.heading("Rüstung");
        grid.show(ui, |ui| {
            self.torso.draw(ArmorInfo::Torso, sim, ui);
            ui.end_row();
            self.head.draw(ArmorInfo::Head, sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = widgets::create_grid("GegnerRüstung");

        ui.heading("Rüstung");
        grid.show(ui, |ui| {
            self.torso.draw_as_opponent(ArmorInfo::Torso, ui);
            ui.end_row();
            self.head.draw_as_opponent(ArmorInfo::Head, ui);
            ui.end_row();
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display)]
enum ArmorInfo {
    Torso,
    Head,
}

impl DrawInfo<IntStat<0, 5>> for ArmorInfo {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Torso => "Torso",
            Self::Head => "Kopf",
        }
    }

    fn mod_dec(&self) -> crate::simulator::CharModification {
        match self {
            Self::Torso => Box::new(|c| c.armor.torso.decrement()),
            Self::Head => Box::new(|c| c.armor.head.decrement()),
        }
    }

    fn mod_inc(&self) -> crate::simulator::CharModification {
        match self {
            Self::Torso => Box::new(|c| c.armor.torso.increment()),
            Self::Head => Box::new(|c| c.armor.head.increment()),
        }
    }

    fn mod_set(&self, value: IntStat<0, 5>) -> crate::simulator::CharModification {
        match self {
            Self::Torso => Box::new(move |c| c.armor.torso.set(value.into())),
            Self::Head => Box::new(move |c| c.armor.head.set(value.into())),
        }
    }
}
