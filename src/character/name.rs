#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Name(String);

impl Default for Name {
    fn default() -> Self {
        Self(String::from("Hans Dampf"))
    }
}

impl crate::app::Drawable for Name {
    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.heading("Heldenname");
        ui.text_edit_singleline(&mut self.0);
    }
}
