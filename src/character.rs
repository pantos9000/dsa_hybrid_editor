mod attributes;
mod name;
mod skills;
mod weapon;

pub use attributes::Attributes;
pub use name::Name;
pub use skills::Skills;
pub use weapon::Weapon;

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
}

impl Character {
    const BUTTON_SIZE: [f32; 2] = [40.0, 40.0];

    pub fn draw(&mut self, sim: &Simulator, io: &IoThread, ui: &mut egui::Ui) {
        util::create_frame(ui).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    self.draw_buttons(io, ui, false);
                    ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                        let no_mod = Box::new(|_: &mut Character| ());
                        sim.gradient(no_mod).draw_sized(Self::BUTTON_SIZE, ui);
                    });
                });
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

    fn draw_buttons(&mut self, io: &IoThread, ui: &mut egui::Ui, is_opponent: bool) {
        let mut add_button = |text, help| -> egui::Response {
            let text = egui::RichText::new(text).size(24.0);
            let button = egui::Button::new(text).corner_radius(10.0);
            ui.add_sized(Self::BUTTON_SIZE, button).on_hover_ui(|ui| {
                egui::show_tooltip(ui.ctx(), ui.layer_id(), egui::Id::new("my_tooltip"), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(help);
                    });
                });
            })
        };

        let save = add_button("💾", "Save");
        let open = add_button("🗁", "Open");
        let reset = add_button("❌", "Reset");
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
