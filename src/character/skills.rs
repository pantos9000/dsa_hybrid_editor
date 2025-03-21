use strum::IntoEnumIterator;

use crate::simulator::{CharacterModification, Simulator};

use super::{Character, Drawable};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Skills {
    pub(crate) kämpfen: Skill,
}

impl Drawable for Skills {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("Fähigkeiten");

        ui.heading("Fähigkeiten");
        grid.show(ui, |ui| {
            let mut draw = |skill: &mut Skill, name, modification: CharacterModification| {
                ui.label(name);
                skill.draw(ui);
                sim.gradient(modification).draw(ui);
                ui.end_row();
            };

            let mod_kä = |c: &mut Character| c.skills.kämpfen.increment();

            draw(&mut self.kämpfen, "Kämpfen", Box::new(mod_kä));
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let draw = |skill: &mut Skill, name, ui: &mut egui::Ui| {
            ui.label(name);
            skill.draw_as_opponent(ui);
            ui.end_row();
        };
        let grid = crate::util::create_grid("OpponentSkills");

        ui.heading("Fähigkeiten");
        grid.show(ui, |ui| draw(&mut self.kämpfen, "Kämpfen", ui));
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

impl Skill {
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
