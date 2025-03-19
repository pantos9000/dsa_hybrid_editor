use strum::IntoEnumIterator;

use super::Drawable;

// use crate::simulator::Gradient;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Skills {
    pub(crate) kampfen: Skill,
    // #[serde(skip)]
    // kampfen_gradient: Gradient,
}

impl Drawable for Skills {
    fn draw(&mut self, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("Fähigkeiten");

        ui.heading("Fähigkeiten");
        grid.show(ui, |ui| {
            ui.label("Kämpfen");
            self.kampfen.draw(ui);
            // self.kampfen_gradient.draw(ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("OpponentSkills");

        ui.heading("Fähigkeiten");
        grid.show(ui, |ui| {
            ui.label("Kämpfen");
            self.kampfen.draw_as_opponent(ui);
            // self.kampfen_gradient.draw(ui);
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
pub enum Skill {
    #[default]
    W4m2,
    W4,
    W6,
    W8,
    W10,
    W12,
    W12p1,
    W12p2,
}

impl Drawable for Skill {
    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for val in Self::iter() {
                ui.selectable_value(self, val, val.as_str());
            }
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let _ = ui.button(self.as_str());
    }
}

impl Skill {
    fn as_str(&self) -> &'static str {
        match self {
            Skill::W4m2 => "W4-2",
            Skill::W4 => "W4",
            Skill::W6 => "W6",
            Skill::W8 => "W8",
            Skill::W10 => "W10",
            Skill::W12 => "W12",
            Skill::W12p1 => "W12+1",
            Skill::W12p2 => "W12+2",
        }
    }

    #[allow(dead_code)]
    pub fn increment(&mut self) {
        let new = match self {
            Self::W4m2 => Self::W4,
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
    pub fn decrement(&mut self) {
        let new = match self {
            Self::W4m2 => Self::W4m2,
            Self::W4 => Self::W4m2,
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
