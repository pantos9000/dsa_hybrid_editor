use crate::character::Character;

/// 2 go in, 1 comes out
#[derive(Debug, Default)]
pub struct Simulator {
    character: Character,
    // TODO cache
}

impl Simulator {
    pub fn update_character(&mut self, character: &Character) {
        self.character = character.clone();
    }
}

/// Invariance: always between -100/100
#[derive(Debug, Default, Clone)]
pub struct Gradient(i32);

impl From<i32> for Gradient {
    fn from(value: i32) -> Self {
        assert!(value < 100, "value is too big");
        assert!(value > -100, "value is too small");
        Self(value)
    }
}

impl Gradient {
    pub fn draw(&self, ui: &mut egui::Ui) {
        ui.label(self.0.to_string());
    }
}
