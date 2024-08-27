mod attributes;
mod name;
mod skills;
mod weapon;

use attributes::Attributes;
use name::Name;
use skills::Skills;
use weapon::Weapon;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Character {
    name: Name,
    attributes: Attributes,
    skills: Skills,
    weapon: Weapon,
}

impl crate::app::Drawable for Character {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        draw_in_frame(ui, &mut self.name);
        draw_in_frame(ui, &mut self.attributes);
        draw_in_frame(ui, &mut self.skills);
        draw_in_frame(ui, &mut self.weapon);
    }
}

fn draw_in_frame(ui: &mut egui::Ui, drawable: &mut impl crate::app::Drawable) {
    let frame = crate::app::create_frame(ui);
    frame.show(ui, |ui| {
        drawable.draw_ui(ui);
    });
}
