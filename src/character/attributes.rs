use strum::IntoEnumIterator;

use crate::simulator::Gradient;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Attributes {
    ges: Attribute,
    #[serde(skip)]
    ges_gradient: Gradient,
    sta: Attribute,
    #[serde(skip)]
    sta_gradient: Gradient,
    kon: Attribute,
    #[serde(skip)]
    kon_gradient: Gradient,
    int: Attribute,
    #[serde(skip)]
    int_gradient: Gradient,
    wil: Attribute,
    #[serde(skip)]
    wil_gradient: Gradient,
}

impl crate::app::Drawable for Attributes {
    fn draw(&mut self, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("Attribute");

        ui.heading("Attribute");
        grid.show(ui, |ui| {
            ui.label("GES");
            self.ges.draw(ui);
            self.ges_gradient.draw(ui);
            ui.end_row();

            ui.label("STÄ");
            self.sta.draw(ui);
            self.sta_gradient.draw(ui);
            ui.end_row();

            ui.label("KON");
            self.kon.draw(ui);
            self.kon_gradient.draw(ui);
            ui.end_row();

            ui.label("INT");
            self.int.draw(ui);
            self.int_gradient.draw(ui);
            ui.end_row();

            ui.label("WIL");
            self.wil.draw(ui);
            self.wil_gradient.draw(ui);
            ui.end_row();
        });
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Attribute {
    #[default]
    W4,
    W6,
    W8,
    W10,
    W12,
    W12p1,
    W12p2,
}

impl crate::app::Drawable for Attribute {
    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for val in Self::iter() {
                ui.selectable_value(self, val, val.as_str());
            }
        });
    }
}

impl Attribute {
    fn as_str(&self) -> &'static str {
        match self {
            Self::W4 => "W4",
            Self::W6 => "W6",
            Self::W8 => "W8",
            Self::W10 => "W10",
            Self::W12 => "W12",
            Self::W12p1 => "W12+1",
            Self::W12p2 => "W12+2",
        }
    }

    #[allow(dead_code)]
    fn increment(&mut self) {
        let new = match self {
            Self::W4 => Self::W6,
            Self::W6 => Self::W8,
            Self::W8 => Self::W10,
            Self::W10 => Self::W12,
            Self::W12 => Self::W12p1,
            Self::W12p1 => Self::W12p2,
            Self::W12p2 => Self::W12p2,
        };
        *self = new;
    }

    #[allow(dead_code)]
    fn decrement(&mut self) {
        let new = match self {
            Self::W4 => Self::W4,
            Self::W6 => Self::W4,
            Self::W8 => Self::W6,
            Self::W10 => Self::W8,
            Self::W12 => Self::W10,
            Self::W12p1 => Self::W12,
            Self::W12p2 => Self::W12p1,
        };
        *self = new;
    }
}
