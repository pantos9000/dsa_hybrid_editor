#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Name(String);

impl Default for Name {
    fn default() -> Self {
        Self(String::from("Hans Dampf"))
    }
}

impl crate::app::Drawable for Name {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Heldenname");
        ui.text_edit_singleline(&mut self.0);
    }

    fn draw_gradients(&self, ui: &mut egui::Ui, _simulator: &crate::simulator::Simulator) {
        ui.heading("");
        ui.label("");
        // TODO revisit - how to properly make this bigger?
        // TODO or display overall probabilities?
    }
}
