#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Name(String);

impl Default for Name {
    fn default() -> Self {
        Self(String::from("Hans Dampf"))
    }
}

impl Name {
    pub fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Heldenname");
        ui.text_edit_singleline(&mut self.0);
    }
}
