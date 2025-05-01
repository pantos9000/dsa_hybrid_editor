use super::{Character, Drawable};
use crate::app::widgets::{self, BoolStat, DrawInfo, IntStat, ValueSlider as _};
use crate::simulator::{CharModification, Simulator};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct PassiveModifiers {
    pub(crate) life: IntStat<-20, 20>,
    pub(crate) parry: IntStat<-4, 4>,
    pub(crate) robustness: IntStat<-4, 4>,
    pub(crate) attack: IntStat<-4, 4>,
    pub(crate) attack_wild: BoolStat,
    pub(crate) attack_head: BoolStat,
}

enum StrategyInfo {
    AttackWild,
    AttackHead,
}

impl DrawInfo<BoolStat> for StrategyInfo {
    fn as_str(&self) -> &'static str {
        match self {
            Self::AttackWild => "Wild angreifen",
            Self::AttackHead => "Auf Kopf zielen",
        }
    }

    fn mod_dec(&self) -> CharModification {
        match self {
            Self::AttackWild => Box::new(|c| c.passive_modifiers.attack_wild.decrement()),
            Self::AttackHead => Box::new(|c| c.passive_modifiers.attack_head.decrement()),
        }
    }

    fn mod_inc(&self) -> CharModification {
        match self {
            Self::AttackWild => Box::new(|c| c.passive_modifiers.attack_wild.increment()),
            Self::AttackHead => Box::new(|c| c.passive_modifiers.attack_head.increment()),
        }
    }

    fn mod_set(&self, value: BoolStat) -> CharModification {
        match self {
            Self::AttackWild => Box::new(move |c| c.passive_modifiers.attack_wild.set(value)),
            Self::AttackHead => Box::new(move |c| c.passive_modifiers.attack_head.set(value)),
        }
    }
}

impl Drawable for PassiveModifiers {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let name = "Modifikatoren";
        let grid = widgets::create_grid(name);
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
            self.attack_wild.draw(StrategyInfo::AttackWild, sim, ui);
            ui.end_row();
            self.attack_head.draw(StrategyInfo::AttackHead, sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let name = "Gegner Modifikatoren";
        let grid = widgets::create_grid(name);
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
            self.attack_wild
                .draw_as_opponent(StrategyInfo::AttackWild, ui);
            ui.end_row();
            self.attack_head
                .draw_as_opponent(StrategyInfo::AttackHead, ui);
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
        robustness =
            robustness.saturating_add_signed(character.passive_modifiers.robustness.into());
        robustness
    }

    fn draw_stats(&self, grid_name: &str, ui: &mut egui::Ui) {
        let grid = widgets::create_grid(grid_name);

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
