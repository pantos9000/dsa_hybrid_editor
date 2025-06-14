use crate::simulator::Simulator;

use super::Drawable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Name(String);

impl Name {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for Name {
    fn default() -> Self {
        Self(String::from("Hans Dampf"))
    }
}

impl Drawable for Name {
    fn draw(&mut self, _sim: &mut Simulator, ui: &mut egui::Ui) {
        self.draw_as_opponent(ui);
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        ui.heading("Heldenname");
        ui.text_edit_singleline(&mut self.0);
    }
}

impl Name {
    pub fn clear(&mut self) {
        self.0.clear();
    }
}
