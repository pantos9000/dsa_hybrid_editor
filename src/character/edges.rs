use crate::{simulator::Simulator, util};

use super::{Character, Drawable};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Edges {
    pub(crate) blitzhieb: Blitzhieb,
    pub(crate) berserker: Berserker,
    pub(crate) ubertolpeln: Ubertolpeln,
}

impl Drawable for Edges {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let grid = util::create_grid("Edges");

        ui.heading("Edges");
        grid.show(ui, |ui| {
            self.blitzhieb.draw(sim, ui);
            ui.end_row();
            self.berserker.draw(sim, ui);
            ui.end_row();
            self.ubertolpeln.draw(sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = util::create_grid("GegnerEdges");

        ui.heading("Edges");
        grid.show(ui, |ui| {
            self.blitzhieb.draw_as_opponent(ui);
            ui.end_row();
            self.berserker.draw_as_opponent(ui);
            ui.end_row();
            self.ubertolpeln.draw_as_opponent(ui);
            ui.end_row();
        });
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Blitzhieb {
    #[default]
    None,
    Normal,
    Improved,
}

impl Blitzhieb {
    fn as_str(&self) -> &'static str {
        match self {
            Blitzhieb::None => "Nein",
            Blitzhieb::Normal => "Blitzhieb",
            Blitzhieb::Improved => "Verb. Blitzhieb",
        }
    }

    fn decrement(&mut self) {
        let new = match self {
            Blitzhieb::None => Self::Normal,
            Blitzhieb::Normal => Self::Improved,
            Blitzhieb::Improved => Self::Improved,
        };
        *self = new;
    }

    fn increment(&mut self) {
        let new = match self {
            Blitzhieb::None => Self::None,
            Blitzhieb::Normal => Self::None,
            Blitzhieb::Improved => Self::Normal,
        };
        *self = new;
    }

    fn draw_selectable(&mut self, val: Self, sim: &Simulator, ui: &mut egui::Ui) {
        ui.selectable_value(self, val, val.as_str())
            .on_hover_ui(|ui| {
                ui.horizontal(|ui| {
                    let modification = Box::new(move |c: &mut Character| {
                        c.edges.blitzhieb = val;
                    });
                    sim.gradient(modification).draw(ui);
                });
            });
    }

    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label("Blitzhieb");

        ui.horizontal(|ui| {
            self.draw_selectable(Blitzhieb::None, sim, ui);
            self.draw_selectable(Blitzhieb::Normal, sim, ui);
            self.draw_selectable(Blitzhieb::Improved, sim, ui);
        });

        let mod_dec = Box::new(|c: &mut Character| c.edges.blitzhieb.decrement());
        let mod_inc = Box::new(|c: &mut Character| c.edges.blitzhieb.increment());
        ui.horizontal(|ui| {
            sim.gradient(mod_dec).draw(ui);
            sim.gradient(mod_inc).draw(ui);
        });
    }

    fn draw_as_opponent(&self, ui: &mut egui::Ui) {
        ui.label("Blitzhieb");
        let _ = ui.button(self.as_str());
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Ubertolpeln(bool);

impl Ubertolpeln {
    pub fn is_set(&self) -> bool {
        self.0
    }

    fn as_str(&self) -> &'static str {
        match self.0 {
            true => "Ja",
            false => "Nein",
        }
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
        ui.label("Übertölpeln");

        ui.checkbox(&mut self.0, "Übertölpeln").on_hover_ui(|ui| {
            ui.horizontal(|ui| {
                let mod_toggle = Box::new(|c: &mut Character| c.edges.ubertolpeln.toggle());
                sim.gradient(mod_toggle).draw(ui);
            });
        });

        let mod_dec = Box::new(|c: &mut Character| c.edges.ubertolpeln.decrement());
        let mod_inc = Box::new(|c: &mut Character| c.edges.ubertolpeln.increment());
        ui.horizontal(|ui| {
            sim.gradient(mod_dec).draw(ui);
            sim.gradient(mod_inc).draw(ui);
        });
    }

    fn draw_as_opponent(&self, ui: &mut egui::Ui) {
        ui.label("Übertölpeln");
        let _ = ui.button(self.as_str());
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Berserker {
    #[default]
    None,
    Normal,
    Immediate,
}

impl Berserker {
    fn as_str(&self) -> &'static str {
        match self {
            Berserker::None => "Nein",
            Berserker::Normal => "Berserker",
            Berserker::Immediate => "SofortBerserker",
        }
    }

    fn decrement(&mut self) {
        let new = match self {
            Berserker::None => Self::None,
            Berserker::Normal => Self::None,
            Berserker::Immediate => Self::Normal,
        };
        *self = new;
    }

    fn increment(&mut self) {
        let new = match self {
            Berserker::None => Self::Normal,
            Berserker::Normal => Self::Immediate,
            Berserker::Immediate => Self::Immediate,
        };
        *self = new;
    }

    fn draw_selectable(&mut self, val: Self, sim: &Simulator, ui: &mut egui::Ui) {
        ui.selectable_value(self, val, val.as_str())
            .on_hover_ui(|ui| {
                ui.horizontal(|ui| {
                    let modification = Box::new(move |c: &mut Character| {
                        c.edges.berserker = val;
                    });
                    sim.gradient(modification).draw(ui);
                });
            });
    }

    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label("Berserker");

        ui.horizontal(|ui| {
            self.draw_selectable(Berserker::None, sim, ui);
            self.draw_selectable(Berserker::Normal, sim, ui);
            self.draw_selectable(Berserker::Immediate, sim, ui);
        });

        let mod_dec = Box::new(|c: &mut Character| c.edges.berserker.decrement());
        let mod_inc = Box::new(|c: &mut Character| c.edges.berserker.increment());
        ui.horizontal(|ui| {
            sim.gradient(mod_dec).draw(ui);
            sim.gradient(mod_inc).draw(ui);
        });
    }

    fn draw_as_opponent(&self, ui: &mut egui::Ui) {
        ui.label("Berserker");
        let _ = ui.button(self.as_str());
    }
}
