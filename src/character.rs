mod attributes;
mod name;
mod skills;
mod weapon;

pub use attributes::Attributes;
pub use name::Name;
pub use skills::Skills;
pub use weapon::Weapon;

use crate::{simulator::Simulator, util};

/// Represents a drawable element of a char
trait Drawable {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui);
    fn draw_as_opponent(&mut self, ui: &mut egui::Ui);
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Character {
    pub(crate) name: Name,
    pub(crate) attributes: Attributes,
    pub(crate) skills: Skills,
    pub(crate) weapon: Weapon,
}

impl Character {
    pub fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        util::create_frame(ui).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                util::create_frame(ui).show(ui, |ui| {
                    self.name.draw(sim, ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.attributes.draw(sim, ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.skills.draw(sim, ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.weapon.draw(sim, ui);
                });
            });
        });
    }

    pub fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        util::create_frame(ui).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                util::create_frame(ui).show(ui, |ui| {
                    self.name.draw_as_opponent(ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.attributes.draw_as_opponent(ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.skills.draw_as_opponent(ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.weapon.draw_as_opponent(ui);
                });
            });
        });
    }
}
