use super::{Character, Drawable};
use crate::app::widgets::{DrawInfo, IntStat, ValueSlider as _};
use crate::simulator::{CharModification, Simulator};
use crate::util;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct PassiveModifiers {
    pub(crate) life: IntStat<-20, 20>,
    pub(crate) parry: IntStat<-4, 4>,
    pub(crate) robustness: IntStat<-4, 4>,
    pub(crate) attack: IntStat<-4, 4>,
    pub(crate) attack_wild: WildAttack,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct WildAttack(bool);

impl WildAttack {
    pub fn is_set(self) -> bool {
        self.0
    }

    fn decrement(&mut self) {
        self.0 = false;
    }

    fn increment(&mut self) {
        self.0 = true;
    }

    fn toggle(&mut self) {
        self.0 = !self.0;
    }

    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let mod_toggle: CharModification = Box::new(|c| c.passive_modifiers.attack_wild.toggle());
        let mod_dec: CharModification = Box::new(|c| c.passive_modifiers.attack_wild.decrement());
        let mod_inc: CharModification = Box::new(|c| c.passive_modifiers.attack_wild.increment());

        ui.checkbox(&mut self.0, "Wild angreifen")
            .on_hover_ui(|ui| {
                ui.horizontal(|ui| {
                    sim.gradient(mod_toggle).draw(ui);
                });
            });

        ui.horizontal(|ui| {
            sim.gradient(mod_dec).draw(ui);
            sim.gradient(mod_inc).draw(ui);
        });
    }

    fn draw_as_opponent(self, ui: &mut egui::Ui) {
        ui.label("Wild angreifen");
        let text = if self.0 { "Ja" } else { "Nein" };
        let _ = ui.button(text);
    }
}

impl Drawable for PassiveModifiers {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let name = "Modifikatoren";
        let grid = util::create_grid(name);
        ui.heading(name);
        grid.show(ui, |ui| {
            self.life.draw(PassiveModifier::Life, sim, ui);
            ui.end_row();
            self.parry.draw(PassiveModifier::Parry, sim, ui);
            ui.end_row();
            self.robustness.draw(PassiveModifier::Robustness, sim, ui);
            ui.end_row();
            self.attack.draw(PassiveModifier::Attack, sim, ui);
            ui.end_row();
            self.attack_wild.draw(sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let name = "Gegner Modifikatoren";
        let grid = util::create_grid(name);
        ui.heading(name);
        grid.show(ui, |ui| {
            self.life.draw_as_opponent(PassiveModifier::Life, ui);
            ui.end_row();
            self.parry.draw_as_opponent(PassiveModifier::Parry, ui);
            ui.end_row();
            self.robustness
                .draw_as_opponent(PassiveModifier::Robustness, ui);
            ui.end_row();
            self.attack.draw_as_opponent(PassiveModifier::Attack, ui);
            ui.end_row();
            self.attack_wild.draw_as_opponent(ui);
            ui.end_row();
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PassiveModifier {
    Life,
    Parry,
    Robustness,
    Attack,
}

impl<const MIN: i8, const MAX: i8> DrawInfo<IntStat<MIN, MAX>> for PassiveModifier {
    fn as_str(&self) -> &'static str {
        match self {
            PassiveModifier::Life => "LeP",
            PassiveModifier::Parry => "PA",
            PassiveModifier::Robustness => "ROB",
            PassiveModifier::Attack => "AT",
        }
    }

    fn mod_dec(&self) -> CharModification {
        match self {
            PassiveModifier::Life => Box::new(|c| c.passive_modifiers.life.decrement()),
            PassiveModifier::Parry => Box::new(|c| c.passive_modifiers.parry.decrement()),
            PassiveModifier::Robustness => Box::new(|c| c.passive_modifiers.robustness.decrement()),
            PassiveModifier::Attack => Box::new(|c| c.passive_modifiers.attack.decrement()),
        }
    }

    fn mod_inc(&self) -> CharModification {
        match self {
            PassiveModifier::Life => Box::new(|c| c.passive_modifiers.life.increment()),
            PassiveModifier::Parry => Box::new(|c| c.passive_modifiers.parry.increment()),
            PassiveModifier::Robustness => Box::new(|c| c.passive_modifiers.robustness.increment()),
            PassiveModifier::Attack => Box::new(|c| c.passive_modifiers.attack.increment()),
        }
    }

    fn mod_set(&self, value: IntStat<MIN, MAX>) -> CharModification {
        match self {
            PassiveModifier::Life => Box::new(move |c| c.passive_modifiers.life.set(value.into())),
            PassiveModifier::Parry => {
                Box::new(move |c| c.passive_modifiers.parry.set(value.into()))
            }
            PassiveModifier::Robustness => {
                Box::new(move |c| c.passive_modifiers.robustness.set(value.into()))
            }
            PassiveModifier::Attack => {
                Box::new(move |c| c.passive_modifiers.attack.set(value.into()))
            }
        }
    }
}

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
        let edge_bonus = match character.edges.lebenskraft {
            super::Edge3::None => 0,
            super::Edge3::Normal => 5,
            super::Edge3::Improved => 10,
        };
        life = life.saturating_add(edge_bonus);
        life = life.saturating_add_signed(character.passive_modifiers.life.into());
        life
    }

    fn calc_parry(character: &Character) -> u8 {
        let mut parry = 2 + u8::from(character.skills.kampfen) / 2;
        parry = parry.saturating_add_signed(character.passive_modifiers.parry.into());
        if character.edges.beidhandig.is_set()
            && character.weapon.active
            && character.secondary_weapon.active
        {
            parry += 1;
        }
        parry
    }

    fn calc_robustness(character: &Character) -> u8 {
        let mut robustness = 2 + u8::from(character.attributes.kon) / 2;
        robustness = robustness.saturating_add_signed(character.armor.torso.into());
        robustness =
            robustness.saturating_add_signed(character.passive_modifiers.robustness.into());
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
