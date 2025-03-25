use egui::{Button, Color32};

#[derive(Debug, Default, Copy, Clone)]
pub struct InvalidGradientValue;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Gradient {
    value: Option<i8>,
}

impl TryFrom<i8> for Gradient {
    type Error = InvalidGradientValue;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            ..-100 => Err(InvalidGradientValue),
            101.. => Err(InvalidGradientValue),
            value => Ok(Self { value: Some(value) }),
        }
    }
}

impl Gradient {
    pub const NONE: Self = Gradient { value: None };

    pub fn draw(&self, ui: &mut egui::Ui) {
        match self.value {
            None => Self::draw_spinner(ui),
            Some(value) => Self::draw_value(value, ui),
        }
    }

    fn draw_spinner(ui: &mut egui::Ui) {
        let spinner = egui::widgets::Spinner::new();
        ui.add(spinner);
    }

    fn draw_value(value: i8, ui: &mut egui::Ui) {
        let dark = ui.visuals().dark_mode;
        let dark_gray = Color32::from_rgb(64, 64, 64);
        let text = format!("{}", value);
        let color = if dark {
            match value {
                ..0 => Color32::DARK_RED,
                0 => dark_gray,
                1.. => Color32::DARK_GREEN,
            }
        } else {
            match value {
                ..0 => Color32::LIGHT_RED,
                0 => Color32::LIGHT_GRAY,
                1.. => Color32::LIGHT_GREEN,
            }
        };
        let button = Button::new(text).frame(false).fill(color);
        let _ = ui.add(button);
    }
}
