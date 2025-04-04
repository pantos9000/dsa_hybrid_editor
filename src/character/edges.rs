use crate::{
    simulator::{CharModification, Simulator},
    util,
};

use super::Drawable;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Edges {
    pub(crate) blitzhieb: Edge3,
    pub(crate) berserker: Edge3,
    pub(crate) riposte: Edge3,
    pub(crate) ubertolpeln: Edge2,
}

impl Drawable for Edges {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let grid = util::create_grid("Edges");

        ui.heading("Edges");
        grid.show(ui, |ui| {
            self.blitzhieb.draw(Edge3Name::Blitzhieb, sim, ui);
            ui.end_row();
            self.berserker.draw(Edge3Name::Berserker, sim, ui);
            ui.end_row();
            self.riposte.draw(Edge3Name::Riposte, sim, ui);
            ui.end_row();
            self.ubertolpeln.draw(Edge2Name::Übertölpeln, sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = util::create_grid("GegnerEdges");

        ui.heading("Edges");
        grid.show(ui, |ui| {
            self.blitzhieb.draw_as_opponent(Edge3Name::Blitzhieb, ui);
            ui.end_row();
            self.berserker.draw_as_opponent(Edge3Name::Berserker, ui);
            ui.end_row();
            self.riposte.draw_as_opponent(Edge3Name::Riposte, ui);
            ui.end_row();
            self.ubertolpeln.draw_as_opponent(ui);
            ui.end_row();
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edge2Name {
    Übertölpeln,
}

impl Edge2Name {
    fn as_str(&self) -> &'static str {
        match self {
            Edge2Name::Übertölpeln => "Übertölpeln",
        }
    }

    fn modification_dec(&self) -> CharModification {
        match self {
            Edge2Name::Übertölpeln => Box::new(|c| c.edges.ubertolpeln.decrement()),
        }
    }

    fn modification_inc(&self) -> CharModification {
        match self {
            Edge2Name::Übertölpeln => Box::new(|c| c.edges.ubertolpeln.increment()),
        }
    }

    fn modification_toggle(&self) -> CharModification {
        match self {
            Edge2Name::Übertölpeln => Box::new(|c| c.edges.ubertolpeln.toggle()),
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Edge2(bool);

impl Edge2 {
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

    fn draw(&mut self, name: Edge2Name, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label(name.as_str());

        ui.checkbox(&mut self.0, name.as_str()).on_hover_ui(|ui| {
            ui.horizontal(|ui| {
                sim.gradient(name.modification_toggle()).draw(ui);
            });
        });

        ui.horizontal(|ui| {
            sim.gradient(name.modification_dec()).draw(ui);
            sim.gradient(name.modification_inc()).draw(ui);
        });
    }

    fn draw_as_opponent(&self, ui: &mut egui::Ui) {
        ui.label("Übertölpeln");
        let _ = ui.button(self.as_str());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edge3Name {
    Blitzhieb,
    Berserker,
    Riposte,
}

impl Edge3Name {
    fn as_str(&self, val: Edge3) -> &'static str {
        match (self, val) {
            (_, Edge3::None) => "Kein",
            (Edge3Name::Blitzhieb, Edge3::Normal) => "Blitzhieb",
            (Edge3Name::Blitzhieb, Edge3::Improved) => "Verb. Blitzhieb",
            (Edge3Name::Berserker, Edge3::Normal) => "Berserker",
            (Edge3Name::Berserker, Edge3::Improved) => "Berserker Sofort",
            (Edge3Name::Riposte, Edge3::Normal) => "Riposte",
            (Edge3Name::Riposte, Edge3::Improved) => "Verb. Riposte",
        }
    }

    fn modification_dec(&self) -> CharModification {
        match self {
            Edge3Name::Blitzhieb => Box::new(|c| c.edges.blitzhieb.decrement()),
            Edge3Name::Berserker => Box::new(|c| c.edges.berserker.decrement()),
            Edge3Name::Riposte => Box::new(|c| c.edges.riposte.decrement()),
        }
    }

    fn modification_inc(&self) -> CharModification {
        match self {
            Edge3Name::Blitzhieb => Box::new(|c| c.edges.blitzhieb.increment()),
            Edge3Name::Berserker => Box::new(|c| c.edges.berserker.increment()),
            Edge3Name::Riposte => Box::new(|c| c.edges.riposte.increment()),
        }
    }

    fn modification_set(&self, value: Edge3) -> CharModification {
        match self {
            Edge3Name::Blitzhieb => Box::new(move |c| c.edges.blitzhieb = value),
            Edge3Name::Berserker => Box::new(move |c| c.edges.berserker = value),
            Edge3Name::Riposte => Box::new(move |c| c.edges.riposte = value),
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Edge3 {
    #[default]
    None,
    Normal,
    Improved,
}

impl Edge3 {
    fn decrement(&mut self) {
        let new = match self {
            Edge3::None => Self::None,
            Edge3::Normal => Self::None,
            Edge3::Improved => Self::Normal,
        };
        *self = new;
    }

    fn increment(&mut self) {
        let new = match self {
            Edge3::None => Self::Normal,
            Edge3::Normal => Self::Improved,
            Edge3::Improved => Self::Improved,
        };
        *self = new;
    }

    fn draw_selectable(&mut self, name: Edge3Name, val: Self, sim: &Simulator, ui: &mut egui::Ui) {
        ui.selectable_value(self, val, name.as_str(val))
            .on_hover_ui(|ui| {
                ui.horizontal(|ui| {
                    sim.gradient(name.modification_set(val)).draw(ui);
                });
            });
    }

    fn draw(&mut self, name: Edge3Name, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label("Blitzhieb");

        ui.horizontal(|ui| {
            self.draw_selectable(name, Self::None, sim, ui);
            self.draw_selectable(name, Self::Normal, sim, ui);
            self.draw_selectable(name, Self::Improved, sim, ui);
        });

        ui.horizontal(|ui| {
            sim.gradient(name.modification_dec()).draw(ui);
            sim.gradient(name.modification_inc()).draw(ui);
        });
    }

    fn draw_as_opponent(&self, name: Edge3Name, ui: &mut egui::Ui) {
        ui.label("Blitzhieb");
        let _ = ui.button(name.as_str(*self));
    }
}
