use crate::simulator::Simulator;

use super::Drawable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Name(String);

impl Default for Name {
    fn default() -> Self {
        Self(String::from("Hans Dampf"))
    }
}

impl Drawable for Name {
    fn draw(&mut self, _sim: &Simulator, ui: &mut egui::Ui) {
        self.draw_as_opponent(ui);
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        ui.heading("Heldenname");
        ui.text_edit_singleline(&mut self.0);
    }
}
