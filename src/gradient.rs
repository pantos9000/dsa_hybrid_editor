use egui::{Button, Color32};

#[derive(Debug, Default, Copy, Clone)]
pub struct InvalidGradientValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gradient {
    value: i8,
}

impl Default for Gradient {
    fn default() -> Self {
        Self { value: 100 }
    }
}

impl TryFrom<i8> for Gradient {
    type Error = InvalidGradientValue;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            ..-100 => Err(InvalidGradientValue),
            101.. => Err(InvalidGradientValue),
            value => Ok(Self { value }),
        }
    }
}

impl Gradient {
    pub fn draw(&self, ui: &mut egui::Ui) {
        let text = format!("{}", self.value);
        let color = match self.value {
            ..0 => Color32::DARK_RED,
            0 => Color32::LIGHT_GRAY,
            1.. => Color32::DARK_GREEN,
        };
        let button = Button::new(text).frame(false).fill(color);
        let _ = ui.add(button);
    }
}
