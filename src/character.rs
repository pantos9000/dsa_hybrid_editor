mod armor;
mod attributes;
mod bennies;
mod edges;
mod name;
mod passive_stats;
mod skills;
mod weapon;

use passive_stats::PassiveModifiers;

pub use armor::Armor;
pub use attributes::{Attribute, Attributes};
pub use bennies::Bennies;
pub use edges::{Edge3, Edges};
pub use name::Name;
pub use passive_stats::PassiveStats;
pub use skills::{Skill, Skills};
pub use weapon::{Damage, Weapon};

use crate::io::{IoRequest, IoThread};
use crate::simulator::Simulator;
use crate::util;

/// Represents a drawable element of a char
trait Drawable {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui);
    fn draw_as_opponent(&mut self, ui: &mut egui::Ui);
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Character {
    pub(crate) name: Name,
    pub(crate) passive_modifiers: PassiveModifiers,
    pub(crate) attributes: Attributes,
    pub(crate) skills: Skills,
    pub(crate) weapon: Weapon<false>,
    pub(crate) secondary_weapon: Weapon<true>,
    pub(crate) armor: Armor,
    pub(crate) edges: Edges,
    pub(crate) bennies: Bennies,
}

impl Character {
    fn drawable_iter(&mut self) -> impl Iterator<Item = &mut dyn Drawable> {
        [
            &mut self.attributes as _,
            &mut self.skills as _,
            &mut self.passive_modifiers as _,
            &mut self.weapon as _,
            &mut self.secondary_weapon as _,
            &mut self.armor as _,
            &mut self.edges as _,
            &mut self.bennies as _,
        ]
        .into_iter()
    }

    pub fn draw(&mut self, sim: &Simulator, io: &IoThread, ui: &mut egui::Ui) {
        util::create_frame(ui).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    self.draw_buttons(io, ui, false);
                });

                let mut draw = |drawable: &mut dyn Drawable| {
                    util::create_frame(ui).show(ui, |ui| {
                        drawable.draw(sim, ui);
                    });
                };

                let mut passive_stats = PassiveStats::new(self);

                draw(&mut self.name);
                draw(&mut passive_stats);

                for drawable in self.drawable_iter() {
                    draw(drawable);
                }
            });
        });
    }

    pub fn draw_as_opponent(&mut self, io: &IoThread, ui: &mut egui::Ui) {
        util::create_frame(ui).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    self.draw_buttons(io, ui, true);
                });

                let mut draw = |drawable: &mut dyn Drawable| {
                    util::create_frame(ui).show(ui, |ui| {
                        drawable.draw_as_opponent(ui);
                    });
                };

                let mut passive_stats = PassiveStats::new(self);

                draw(&mut self.name);
                draw(&mut passive_stats);

                for drawable in self.drawable_iter() {
                    draw(drawable);
                }
            });
        });
    }

    fn draw_buttons(&mut self, io: &IoThread, ui: &mut egui::Ui, is_opponent: bool) {
        let button_size = 40.0;
        let save = util::create_menu_button("💾", "Save", button_size, ui);
        let open = util::create_menu_button("🗁", "Open", button_size, ui);
        let reset = util::create_menu_button("❌", "Reset", button_size, ui);
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
