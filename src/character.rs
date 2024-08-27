mod attributes;
mod skills;

use attributes::Attributes;
use skills::Skills;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Character {
    name: String,
    attributes: Attributes,
    skills: Skills,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Hans Dampf".to_owned(),
            attributes: Attributes::default(),
            skills: Skills::default(),
        }
    }
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
            ui.heading("Heldenname");
            ui.text_edit_singleline(&mut self.name);
        });

        frame.show(ui, |ui| {
            self.attributes.draw_ui(ui);
        });

        frame.show(ui, |ui| {
            self.skills.draw_ui(ui);
        });
    }
}
