mod attributes;

use attributes::Attributes;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Character {
    name: String,
    attributes: Attributes,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Hans Dampf".to_owned(),
            attributes: Attributes::default(),
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

        ui.horizontal(|ui| {
            ui.label("Heldenname: ");
            ui.text_edit_singleline(&mut self.name);
        });

        frame.show(ui, |ui| {
            self.attributes.draw_ui(ui);
        });
    }
}
