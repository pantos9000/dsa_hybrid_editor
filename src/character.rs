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

impl crate::app::Drawable for Character {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let frame = crate::app::create_frame(ui);

        draw_in_frame(ui, &frame, &mut self.name);
        draw_in_frame(ui, &frame, &mut self.attributes);
        draw_in_frame(ui, &frame, &mut self.skills);
    }
}

fn draw_in_frame(ui: &mut egui::Ui, frame: &egui::Frame, drawable: &mut impl crate::app::Drawable) {
    frame.show(ui, |ui| {
        drawable.draw_ui(ui);
    });
}
