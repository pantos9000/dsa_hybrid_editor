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
pub use weapon::Weapon;

use super::{
    io::{IoRequest, IoThread},
    widgets,
};

use crate::{app, simulator::Simulator};

/// Represents a drawable element of a char
trait Drawable {
    fn draw(&mut self, sim: &mut Simulator, ui: &mut egui::Ui);
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Character {
    pub(crate) name: Name,
    pub(crate) passive_modifiers: PassiveModifiers,
    pub(crate) attributes: Attributes,
    pub(crate) skills: Skills,
    pub(crate) armor: Armor,
    pub(crate) weapon: Weapon<false>,
    pub(crate) secondary_weapon: Weapon<true>,
    pub(crate) edges: Edges,
    pub(crate) bennies: Bennies,
}

impl Character {
    fn drawable_iter(&mut self) -> impl Iterator<Item = &mut dyn Drawable> {
        [
            &mut self.attributes as _,
            &mut self.skills as _,
            &mut self.passive_modifiers as _,
            &mut self.armor as _,
            &mut self.weapon as _,
            &mut self.secondary_weapon as _,
            &mut self.edges as _,
            &mut self.bennies as _,
        ]
        .into_iter()
    }

    pub fn draw_editor(&mut self, sim: &mut Simulator, io: &IoThread, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_width(app::EDITOR_WIDTH);
            ui.set_height(ui.available_height());
            ui.vertical(|ui| {
                egui::containers::ScrollArea::both().show(ui, |ui| {
                    self.draw_buttons(io, ui);

                    let mut draw = |drawable: &mut dyn Drawable| {
                        widgets::create_frame(ui).show(ui, |ui| {
                            ui.set_width(app::EDITOR_WIDTH * 0.9);
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
        });
    }

    pub fn draw_help(ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_width(app::EDITOR_WIDTH);
            ui.set_height(ui.available_height());

            let text = |ui: &mut egui::Ui, s| {
                ui.add_space(10.0);
                ui.label(s);
            };

            let header = |ui: &mut egui::Ui, s| {
                ui.add_space(40.0);
                ui.heading(s);
            };

            ui.vertical_centered(|ui| {
                egui::containers::ScrollArea::both().show(ui, |ui| {
                    header(ui, "Bedienung Gruppen");
                    text(ui, "Links und rechts sind die beiden gegeneinander antretenden Gruppen.");
                    text(ui, "Charakter zu Gruppe hinzufügen: Neu/Laden Button klicken in der entsprechenden Gruppe.");
                    text(ui, "Charakter editieren: Auf Charakter der linken oder rechten Gruppe klicken.");
                    text(ui, "Charakter aus Gruppe löschen: Kleines 'x'.");
                    text(ui, "Alle Charaktere einer Gruppe löschen: Mülleimer.");

                    header(ui, "Bedienung Editor");
                    text(ui, "Charakter speichern: Save Button.");
                    text(ui, "Charakter auf Default (alles W4) zurücksetzen: 'X' Button.");
                    text(ui, "Setzen der Werte mit Slidern und Buttons.");
                    text(ui, "Rechts daneben: Änderung der Gewinnchance für die linke Gruppe bei inkrement/dekrement des Werts.");
                    text(ui, "Tooltip bei Buttons/Checkboxes zeigen Änderung der Gewinnchance für diesen Wert an.");

                    header(ui, "Kampf");
                    text(ui, "Jede Gruppe greift immer den ersten der jeweils anderen Gruppe an.");
                });
            });
        });
    }

    pub fn draw_as_button(
        &mut self,
        simulator: &mut Simulator,
        ui: &mut egui::Ui,
    ) -> egui::Response {
        // TODO: use sim
        let char_help = "Diesen Char auswählen";
        let char_text = self.name.as_str();
        let char_text = egui::RichText::new(char_text).size(15.0);
        let char_button = egui::Button::new(char_text).corner_radius(10.0);
        ui.add_sized([96.0, 24.0], char_button).on_hover_ui(|ui| {
            ui.horizontal(|ui| {
                ui.label(char_help);
            });
        })
    }

    fn draw_buttons(&mut self, io: &IoThread, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let button_size = 40.0;
            let save = widgets::create_menu_button("💾", "Save", button_size, ui);
            let reset = widgets::create_menu_button("↺", "Reset", button_size, ui);
            if save.clicked() {
                io.request(IoRequest::Save(self.clone()));
            }
            if reset.clicked() {
                // TODO do not reset to default but to "non-dirty"
                *self = Self::default();
            }
        });
    }
}
