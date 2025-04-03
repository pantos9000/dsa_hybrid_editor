use crate::{simulator::Simulator, util};

use super::{Character, Drawable};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Edges {
    pub(crate) blitzhieb: Blitzhieb,
}

impl Drawable for Edges {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let grid = util::create_grid("Edges");

        ui.heading("Edges");
        grid.show(ui, |ui| {
            self.blitzhieb.draw(sim, ui);
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = util::create_grid("GegnerEdges");

        ui.heading("Edges");
        grid.show(ui, |ui| {
            self.blitzhieb.draw_as_opponent(ui);
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
            Blitzhieb::None => "Kein",
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
