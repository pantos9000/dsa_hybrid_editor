mod attributes;
mod name;
mod skills;
mod weapon;

pub use attributes::Attributes;
pub use name::Name;
pub use skills::Skills;
pub use weapon::Weapon;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Character {
    name: Name,
    attributes: Attributes,
    skills: Skills,
    weapon: Weapon,
}

impl crate::app::Drawable for Character {
    fn draw(&mut self, ui: &mut egui::Ui) {
        crate::util::create_frame(ui).show(ui, |ui| {
            draw_ui_in_frame(ui, &mut self.name);
            draw_ui_in_frame(ui, &mut self.attributes);
            draw_ui_in_frame(ui, &mut self.skills);
            draw_ui_in_frame(ui, &mut self.weapon);
        });
    }
}

fn draw_ui_in_frame(ui: &mut egui::Ui, drawable: &mut impl crate::app::Drawable) {
    ui.with_layout(
        egui::Layout::top_down_justified(egui::Align::Center),
        |ui| {
            crate::util::create_frame(ui).show(ui, |ui| {
                drawable.draw(ui);
            });
        },
    );
}
