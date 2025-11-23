use crate::{
    app::{
        self,
        widgets::{self, DrawInfo, IntStat, ValueSlider as _},
    },
    simulator::{self, Simulator},
};

use super::Drawable;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Armor {
    pub(crate) torso: IntStat<0, 5>,
    pub(crate) head: IntStat<0, 5>,
}

impl Drawable for Armor {
    fn draw(&mut self, selection: app::CharSelection, sim: &mut Simulator, ui: &mut egui::Ui) {
        let grid = widgets::create_grid("Rüstung");

        ui.heading("Rüstung");
        grid.show(ui, |ui| {
            self.torso.draw(ArmorInfo::Torso, selection, sim, ui);
            ui.end_row();
            self.head.draw(ArmorInfo::Head, selection, sim, ui);
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

    fn mod_dec(&self, selection: app::CharSelection) -> simulator::CharModification {
        let modification: simulator::CharModFunc = match self {
            Self::Torso => Box::new(|c| c.armor.torso.decrement()),
            Self::Head => Box::new(|c| c.armor.head.decrement()),
        };
        simulator::CharModification::new(selection, modification)
    }

    fn mod_inc(&self, selection: app::CharSelection) -> simulator::CharModification {
        let modification: simulator::CharModFunc = match self {
            Self::Torso => Box::new(|c| c.armor.torso.increment()),
            Self::Head => Box::new(|c| c.armor.head.increment()),
        };
        simulator::CharModification::new(selection, modification)
    }

    fn mod_set(
        &self,
        selection: app::CharSelection,
        value: IntStat<0, 5>,
    ) -> crate::simulator::CharModification {
        let modification: simulator::CharModFunc = match self {
            Self::Torso => Box::new(move |c| c.armor.torso.set(value.into())),
            Self::Head => Box::new(move |c| c.armor.head.set(value.into())),
        };
        simulator::CharModification::new(selection, modification)
    }
}
