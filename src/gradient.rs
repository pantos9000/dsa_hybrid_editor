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

    pub fn draw_sized(&self, max_size: impl Into<egui::Vec2>, ui: &mut egui::Ui) {
        match self.value {
            None => {
                ui.add_sized(max_size, egui::widgets::Spinner::new());
            }
            Some(value) => {
                let drawable = Self::draw_value(value, ui);
                ui.add_sized(max_size, drawable);
            }
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        match self.value {
            None => {
                ui.spinner();
            }
            Some(value) => {
                let drawable = Self::draw_value(value, ui);
                ui.add(drawable);
            }
        }
    }

    fn draw_value(value: i8, ui: &mut egui::Ui) -> impl egui::Widget {
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
        Button::new(text).frame(false).fill(color)
    }
}
