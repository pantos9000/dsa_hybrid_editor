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

    pub fn gradient(&self, char_modifications: impl FnOnce(&mut Character)) -> Gradient {
        // TODO
        42.into()
    }
}

/// Invariance: always between -100/100
pub struct Gradient(i32);

impl From<i32> for Gradient {
    fn from(value: i32) -> Self {
        assert!(value < 100, "value is too big");
        assert!(value > -100, "value is too small");
        Self(value)
    }
}

impl From<Gradient> for i32 {
    fn from(value: Gradient) -> Self {
        value.0
    }
}

impl Gradient {
    pub fn draw_ui(&self, ui: &mut egui::Ui) {
        ui.label(self.0.to_string());
    }
}
