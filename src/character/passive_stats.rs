use super::{Character, Drawable};
use crate::simulator::Simulator;

#[derive(Debug, Clone)]
pub struct PassiveStats {
    pub(crate) life: u8,
    pub(crate) parry: u8,
    pub(crate) robustness: u8,
}

impl PassiveStats {
    pub fn new(character: &Character) -> Self {
        Self {
            life: Self::calc_life(character),
            parry: Self::calc_parry(character),
            robustness: Self::calc_robustness(character),
        }
    }

    fn calc_life(character: &Character) -> u8 {
        let mut life = 24 + u8::from(character.attributes.kon) + u8::from(character.attributes.wil);
        life += match character.edges.lebenskraft {
            super::Edge3::None => 0,
            super::Edge3::Normal => 5,
            super::Edge3::Improved => 10,
        };
        life
    }

    fn calc_parry(character: &Character) -> u8 {
        let mut parry = 2 + u8::from(character.skills.kampfen) / 2;
        parry = parry.saturating_add_signed(character.weapon.bonus_parry.into());
        parry
    }

    fn calc_robustness(character: &Character) -> u8 {
        let mut robustness = 2 + u8::from(character.attributes.kon) / 2;
        robustness += u8::from(character.armor.torso);
        robustness
    }

    fn draw_stats(&self, grid_name: &str, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid(grid_name);

        ui.heading("Passive Werte");
        grid.show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("LeP");
                let _ = ui.button(self.life.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("PA");
                let _ = ui.button(self.parry.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("ROB");
                let _ = ui.button(self.robustness.to_string());
            });
        });
    }
}

impl Drawable for PassiveStats {
    fn draw(&mut self, _sim: &Simulator, ui: &mut egui::Ui) {
        self.draw_stats("PassiveWerte", ui);
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        self.draw_stats("GegnerPassiveWerte", ui);
    }
}
