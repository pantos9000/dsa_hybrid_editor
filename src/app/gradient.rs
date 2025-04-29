use egui::{Button, Color32};

#[derive(Debug, Default, Copy, Clone)]
pub struct InvalidGradientValue;

#[derive(Debug, Default, Copy, Clone)]
pub struct InvalidTotalValue;

#[derive(Debug, Default, Copy, Clone)]
pub struct MissingTotalValue;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Total(Option<i8>);

impl TryFrom<i8> for Total {
    type Error = InvalidTotalValue;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            ..0 => Err(InvalidTotalValue),
            101.. => Err(InvalidTotalValue),
            value => Ok(Self(Some(value))),
        }
    }
}

impl TryFrom<Total> for i8 {
    type Error = MissingTotalValue;

    fn try_from(value: Total) -> Result<Self, Self::Error> {
        value.0.ok_or(MissingTotalValue)
    }
}

impl std::ops::Sub for Total {
    type Output = Gradient;

    fn sub(self, rhs: Self) -> Self::Output {
        let Some(lhs) = self.0 else {
            return Gradient::NONE;
        };
        let Some(rhs) = rhs.0 else {
            return Gradient::NONE;
        };

        Gradient {
            value: Some(lhs - rhs),
        }
    }
}

impl Total {
    pub const NONE: Self = Self(None);
    pub const ZERO: Self = Self(Some(0));

    pub fn draw(self, max_size: impl Into<egui::Vec2>, ui: &mut egui::Ui) {
        match self.0 {
            None => {
                ui.add_sized(max_size, egui::widgets::Spinner::new());
            }
            Some(value) => {
                let drawable = draw_value(value, ui, false, false);
                ui.add_sized(max_size, drawable);
            }
        }
    }
}

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

    pub fn draw(self, ui: &mut egui::Ui) {
        match self.value {
            None => {
                ui.spinner();
            }
            Some(value) => {
                let drawable = draw_value(value, ui, true, true);
                ui.add(drawable);
            }
        }
    }
}

fn draw_value(
    value: i8,
    ui: &mut egui::Ui,
    draw_sign: bool,
    color: bool,
) -> impl egui::Widget + use<> {
    let dark = ui.visuals().dark_mode;
    let dark_gray = Color32::from_rgb(64, 64, 64);
    let sign = if draw_sign && value > 0 { "+" } else { "" };
    let text = format!("{sign}{value}");
    let color = match (color, dark, value) {
        // colors disabled
        (false, true, _) => dark_gray,
        (false, false, _) => Color32::LIGHT_GRAY,
        // dark mode on
        (true, true, ..-2) => Color32::DARK_RED,
        (true, true, -2..=2) => dark_gray,
        (true, true, 3..) => Color32::DARK_GREEN,
        // dark mode off
        (true, false, ..-2) => Color32::LIGHT_RED,
        (true, false, -2..=2) => Color32::LIGHT_GRAY,
        (true, false, 3..) => Color32::LIGHT_GREEN,
    };
    Button::new(text).frame(false).fill(color)
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<Total> for Option<i8> {
        fn from(value: Total) -> Self {
            value.0
        }
    }
}
