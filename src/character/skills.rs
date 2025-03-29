use strum::IntoEnumIterator;

use crate::simulator::{CharModification, Simulator};

use super::Drawable;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Skills {
    pub(crate) kampfen: Skill,
}

impl Drawable for Skills {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("Fähigkeiten");

        ui.heading("Fähigkeiten");
        grid.show(ui, |ui| {
            self.kampfen.draw(SkillName::Kämpfen, sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("GegnerFähigkeiten");

        ui.heading("Fähigkeiten");
        grid.show(ui, |ui| {
            self.kampfen.draw_as_opponent(SkillName::Kämpfen, ui);
            ui.end_row();
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display)]
enum SkillName {
    Kämpfen,
}

impl SkillName {
    fn modification_dec(&self) -> CharModification {
        match self {
            SkillName::Kämpfen => Box::new(|c| c.skills.kampfen.decrement()),
        }
    }

    fn modification_inc(&self) -> CharModification {
        match self {
            SkillName::Kämpfen => Box::new(|c| c.skills.kampfen.increment()),
        }
    }

    fn modification_set(&self, value: Skill) -> CharModification {
        match self {
            SkillName::Kämpfen => Box::new(move |c| c.skills.kampfen = value),
        }
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
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

impl From<Skill> for u8 {
    fn from(skill: Skill) -> Self {
        match skill {
            Skill::W4m2 => 2,
            Skill::W4 => 4,
            Skill::W6 => 6,
            Skill::W8 => 8,
            Skill::W10 => 10,
            Skill::W12 => 12,
            Skill::W12p1 => 13,
            Skill::W12p2 => 14,
        }
    }
}

impl Skill {
    fn draw(&mut self, name: SkillName, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label(format!("{name}"));
        ui.horizontal(|ui| {
            for val in Self::iter() {
                ui.selectable_value(self, val, val.as_str())
                    .on_hover_ui(|ui| {
                        ui.horizontal(|ui| {
                            sim.gradient(name.modification_set(val)).draw(ui);
                        });
                    });
            }
        });

        ui.horizontal(|ui| {
            sim.gradient(name.modification_dec()).draw(ui);
            sim.gradient(name.modification_inc()).draw(ui);
        });
    }

    fn draw_as_opponent(&mut self, name: SkillName, ui: &mut egui::Ui) {
        ui.label(format!("{name}"));
        let _ = ui.button(self.as_str());
    }

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
