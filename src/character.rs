mod attributes;
mod name;
mod skills;

use attributes::Attributes;
use name::Name;
use skills::Skills;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Character {
    name: Name,
    attributes: Attributes,
    skills: Skills,
}

impl Character {
    pub fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let frame = egui::Frame::default()
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .rounding(ui.visuals().widgets.noninteractive.rounding)
            .inner_margin(10.0)
            .outer_margin(5.0)
            .fill(egui::Color32::TRANSPARENT);

        frame.show(ui, |ui| {
            self.name.draw_ui(ui);
        });

        frame.show(ui, |ui| {
            self.attributes.draw_ui(ui);
        });

        frame.show(ui, |ui| {
            self.skills.draw_ui(ui);
        });
    }
}
