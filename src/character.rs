mod attributes;
mod edges;
mod name;
mod passive_stats;
mod skills;
mod weapon;

pub use attributes::{Attribute, Attributes};
pub use edges::{Blitzhieb, Edges};
pub use name::Name;
pub use passive_stats::PassiveStats;
pub use skills::{Skill, Skills};
pub use weapon::{Damage, Weapon};

use crate::io::{IoRequest, IoThread};
use crate::simulator::Simulator;
use crate::util;

use egui::Layout;

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
    pub(crate) edges: Edges,
}

impl Character {
    const BUTTON_SIZE: f32 = 40.0;

    pub fn draw(&mut self, sim: &Simulator, io: &IoThread, ui: &mut egui::Ui) {
        util::create_frame(ui).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    self.draw_buttons(io, ui, false);
                    ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                        sim.total().draw([Self::BUTTON_SIZE, Self::BUTTON_SIZE], ui);
                    });
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.name.draw(sim, ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    PassiveStats::new(self).draw(sim, ui);
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
                util::create_frame(ui).show(ui, |ui| {
                    self.edges.draw(sim, ui);
                });
            });
        });
    }

    pub fn draw_as_opponent(&mut self, io: &IoThread, ui: &mut egui::Ui) {
        util::create_frame(ui).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    self.draw_buttons(io, ui, true);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.name.draw_as_opponent(ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    PassiveStats::new(self).draw_as_opponent(ui);
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
                util::create_frame(ui).show(ui, |ui| {
                    self.edges.draw_as_opponent(ui);
                });
            });
        });
    }

    fn draw_buttons(&mut self, io: &IoThread, ui: &mut egui::Ui, is_opponent: bool) {
        let save = util::create_menu_button("💾", "Save", Self::BUTTON_SIZE, ui);
        let open = util::create_menu_button("🗁", "Open", Self::BUTTON_SIZE, ui);
        let reset = util::create_menu_button("❌", "Reset", Self::BUTTON_SIZE, ui);
        if save.clicked() {
            io.request(crate::io::IoRequest::Save(self.clone()));
        }
        if open.clicked() {
            let request = if is_opponent {
                IoRequest::LoadOpponent
            } else {
                IoRequest::LoadChar
            };
            io.request(request);
        }
        if reset.clicked() {
            *self = Default::default();
        }
    }
}
