mod attributes;
mod name;
mod skills;
mod weapon;

use attributes::Attributes;
use name::Name;
use skills::Skills;
use weapon::Weapon;

use crate::simulator::Simulator;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Character {
    name: Name,
    attributes: Attributes,
    skills: Skills,
    weapon: Weapon,
}

impl crate::app::Drawable for Character {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        draw_ui_in_frame(ui, &mut self.name);
        draw_ui_in_frame(ui, &mut self.attributes);
        draw_ui_in_frame(ui, &mut self.skills);
        draw_ui_in_frame(ui, &mut self.weapon);
    }

    fn draw_gradients(&self, ui: &mut egui::Ui, simulator: &Simulator) {
        draw_gradients_in_frame(ui, &self.name, simulator);
        draw_gradients_in_frame(ui, &self.attributes, simulator);
        draw_gradients_in_frame(ui, &self.skills, simulator);
        draw_gradients_in_frame(ui, &self.weapon, simulator);
    }
}

fn draw_ui_in_frame(ui: &mut egui::Ui, drawable: &mut impl crate::app::Drawable) {
    let frame = crate::app::create_frame(ui);
    frame.show(ui, |ui| {
        drawable.draw_ui(ui);
    });
}

fn draw_gradients_in_frame(
    ui: &mut egui::Ui,
    drawable: &impl crate::app::Drawable,
    simulator: &Simulator,
) {
    let frame = crate::app::create_frame(ui);
    frame.show(ui, |ui| {
        drawable.draw_gradients(ui, simulator);
    });
}
