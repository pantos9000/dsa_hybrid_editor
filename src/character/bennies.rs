use crate::{
    simulator::{CharModification, Simulator},
    util,
};

use super::Drawable;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Bennies {
    pub(crate) num_bennies: util::Modifier<0, 10>,
    pub(crate) use_for_unshake: BennieUsage,
    pub(crate) use_for_attack: BennieUsage,
    pub(crate) use_for_damage: BennieUsage,
}

impl Drawable for Bennies {
    fn draw(&mut self, sim: &crate::simulator::Simulator, ui: &mut egui::Ui) {
        let grid = util::create_grid("Bennies");

        ui.heading("Bennies");
        grid.show(ui, |ui| {
            self.num_bennies.draw(NumBennies, sim, ui);
            ui.end_row();
            self.use_for_unshake.draw(OptionName::Unshake, sim, ui);
            ui.end_row();
            self.use_for_attack.draw(OptionName::Attack, sim, ui);
            ui.end_row();
            self.use_for_damage.draw(OptionName::Damage, sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = util::create_grid("GegnerBennies");

        ui.heading("Bennies");
        grid.show(ui, |ui| {
            self.num_bennies.draw_as_opponent(NumBennies, ui);
            ui.end_row();
            self.use_for_unshake
                .draw_as_opponent(OptionName::Unshake, ui);
            ui.end_row();
            self.use_for_attack.draw_as_opponent(OptionName::Attack, ui);
            ui.end_row();
            self.use_for_damage.draw_as_opponent(OptionName::Damage, ui);
            ui.end_row();
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumBennies;

impl util::ModifierName for NumBennies {
    fn as_str(&self) -> &str {
        "Anzahl Bennies"
    }

    fn modification_dec(&self) -> CharModification {
        Box::new(|c| c.bennies.num_bennies.decrement())
    }

    fn modification_inc(&self) -> CharModification {
        Box::new(|c| c.bennies.num_bennies.increment())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OptionName {
    Unshake,
    Attack,
    Damage,
}

impl OptionName {
    fn as_str(&self) -> &'static str {
        match self {
            OptionName::Unshake => "Nutzen für Entschütteln",
            OptionName::Attack => "Nutzen für Angriffe",
            OptionName::Damage => "Nutzen für Schaden",
        }
    }

    fn modification_dec(&self) -> CharModification {
        match self {
            OptionName::Unshake => Box::new(|c| c.bennies.use_for_unshake.decrement()),
            OptionName::Attack => Box::new(|c| c.bennies.use_for_attack.decrement()),
            OptionName::Damage => Box::new(|c| c.bennies.use_for_damage.decrement()),
        }
    }

    fn modification_inc(&self) -> CharModification {
        match self {
            OptionName::Unshake => Box::new(|c| c.bennies.use_for_unshake.increment()),
            OptionName::Attack => Box::new(|c| c.bennies.use_for_attack.increment()),
            OptionName::Damage => Box::new(|c| c.bennies.use_for_damage.increment()),
        }
    }

    fn modification_toggle(&self) -> CharModification {
        match self {
            OptionName::Unshake => Box::new(|c| c.bennies.use_for_unshake.toggle()),
            OptionName::Attack => Box::new(|c| c.bennies.use_for_attack.toggle()),
            OptionName::Damage => Box::new(|c| c.bennies.use_for_damage.toggle()),
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct BennieUsage(bool);

impl BennieUsage {
    pub fn is_set(&self) -> bool {
        self.0
    }

    fn as_str(&self) -> &'static str {
        match self.0 {
            true => "Ja",
            false => "Nein",
        }
    }

    fn decrement(&mut self) {
        self.0 = false;
    }

    fn increment(&mut self) {
        self.0 = true;
    }

    fn toggle(&mut self) {
        self.0 = !self.0;
    }

    fn draw(&mut self, name: OptionName, sim: &Simulator, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.0, name.as_str()).on_hover_ui(|ui| {
            ui.horizontal(|ui| {
                sim.gradient(name.modification_toggle()).draw(ui);
            });
        });

        ui.horizontal(|ui| {
            sim.gradient(name.modification_dec()).draw(ui);
            sim.gradient(name.modification_inc()).draw(ui);
        });
    }

    fn draw_as_opponent(&self, name: OptionName, ui: &mut egui::Ui) {
        ui.label(name.as_str());
        let _ = ui.button(self.as_str());
    }
}
