use crate::{
    simulator::{CharModification, Simulator},
    util,
};

use super::Drawable;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Edges {
    pub(crate) lebenskraft: Edge3,
    pub(crate) blitzhieb: Edge3,
    pub(crate) berserker: Edge3,
    pub(crate) riposte: Edge3,
    pub(crate) tuchfuhlung: Edge3,
    pub(crate) kampfreflexe: Edge2,
    pub(crate) erstschlag: Edge2,
    pub(crate) ubertolpeln: Edge2,
}

impl Drawable for Edges {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        let grid = util::create_grid("Edges");

        ui.heading("Edges");
        grid.show(ui, |ui| {
            self.lebenskraft.draw(Edge3Name::Lebenskraft, sim, ui);
            ui.end_row();
            self.blitzhieb.draw(Edge3Name::Blitzhieb, sim, ui);
            ui.end_row();
            self.berserker.draw(Edge3Name::Berserker, sim, ui);
            ui.end_row();
            self.riposte.draw(Edge3Name::Riposte, sim, ui);
            ui.end_row();
            self.tuchfuhlung.draw(Edge3Name::Tuchfühlung, sim, ui);
            ui.end_row();
            self.kampfreflexe.draw(Edge2Name::Kampfreflexe, sim, ui);
            ui.end_row();
            self.erstschlag.draw(Edge2Name::Erstschlag, sim, ui);
            ui.end_row();
            self.ubertolpeln.draw(Edge2Name::Übertölpeln, sim, ui);
            ui.end_row();
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = util::create_grid("GegnerEdges");

        ui.heading("Edges");
        grid.show(ui, |ui| {
            self.lebenskraft
                .draw_as_opponent(Edge3Name::Lebenskraft, ui);
            ui.end_row();
            self.blitzhieb.draw_as_opponent(Edge3Name::Blitzhieb, ui);
            ui.end_row();
            self.berserker.draw_as_opponent(Edge3Name::Berserker, ui);
            ui.end_row();
            self.riposte.draw_as_opponent(Edge3Name::Riposte, ui);
            ui.end_row();
            self.tuchfuhlung
                .draw_as_opponent(Edge3Name::Tuchfühlung, ui);
            ui.end_row();
            self.kampfreflexe
                .draw_as_opponent(Edge2Name::Kampfreflexe, ui);
            ui.end_row();
            self.erstschlag.draw_as_opponent(Edge2Name::Erstschlag, ui);
            ui.end_row();
            self.ubertolpeln
                .draw_as_opponent(Edge2Name::Übertölpeln, ui);
            ui.end_row();
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edge2Name {
    Übertölpeln,
    Erstschlag,
    Kampfreflexe,
}

impl Edge2Name {
    fn as_str(&self) -> &'static str {
        match self {
            Edge2Name::Übertölpeln => "Übertölpeln",
            Edge2Name::Erstschlag => "Erstschlag",
            Edge2Name::Kampfreflexe => "Kampfreflexe",
        }
    }

    fn modification_dec(&self) -> CharModification {
        match self {
            Edge2Name::Übertölpeln => Box::new(|c| c.edges.ubertolpeln.decrement()),
            Edge2Name::Erstschlag => Box::new(|c| c.edges.erstschlag.decrement()),
            Edge2Name::Kampfreflexe => Box::new(|c| c.edges.kampfreflexe.decrement()),
        }
    }

    fn modification_inc(&self) -> CharModification {
        match self {
            Edge2Name::Übertölpeln => Box::new(|c| c.edges.ubertolpeln.increment()),
            Edge2Name::Erstschlag => Box::new(|c| c.edges.erstschlag.increment()),
            Edge2Name::Kampfreflexe => Box::new(|c| c.edges.kampfreflexe.increment()),
        }
    }

    fn modification_toggle(&self) -> CharModification {
        match self {
            Edge2Name::Übertölpeln => Box::new(|c| c.edges.ubertolpeln.toggle()),
            Edge2Name::Erstschlag => Box::new(|c| c.edges.erstschlag.toggle()),
            Edge2Name::Kampfreflexe => Box::new(|c| c.edges.kampfreflexe.toggle()),
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

    fn draw_as_opponent(&self, name: Edge2Name, ui: &mut egui::Ui) {
        ui.label(name.as_str());
        let _ = ui.button(self.as_str());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edge3Name {
    Blitzhieb,
    Berserker,
    Riposte,
    Tuchfühlung,
    Lebenskraft,
}

impl Edge3Name {
    fn as_str(&self) -> &'static str {
        match self {
            Edge3Name::Blitzhieb => "Blitzhieb",
            Edge3Name::Berserker => "Berserker",
            Edge3Name::Riposte => "Riposte",
            Edge3Name::Tuchfühlung => "Tuchfühlung",
            Edge3Name::Lebenskraft => "Lebenskraft",
        }
    }

    fn modification_dec(&self) -> CharModification {
        match self {
            Edge3Name::Blitzhieb => Box::new(|c| c.edges.blitzhieb.decrement()),
            Edge3Name::Berserker => Box::new(|c| c.edges.berserker.decrement()),
            Edge3Name::Riposte => Box::new(|c| c.edges.riposte.decrement()),
            Edge3Name::Tuchfühlung => Box::new(|c| c.edges.tuchfuhlung.decrement()),
            Edge3Name::Lebenskraft => Box::new(|c| c.edges.lebenskraft.decrement()),
        }
    }

    fn modification_inc(&self) -> CharModification {
        match self {
            Edge3Name::Blitzhieb => Box::new(|c| c.edges.blitzhieb.increment()),
            Edge3Name::Berserker => Box::new(|c| c.edges.berserker.increment()),
            Edge3Name::Riposte => Box::new(|c| c.edges.riposte.increment()),
            Edge3Name::Tuchfühlung => Box::new(|c| c.edges.tuchfuhlung.increment()),
            Edge3Name::Lebenskraft => Box::new(|c| c.edges.lebenskraft.increment()),
        }
    }

    fn modification_set(&self, value: Edge3) -> CharModification {
        match self {
            Edge3Name::Blitzhieb => Box::new(move |c| c.edges.blitzhieb = value),
            Edge3Name::Berserker => Box::new(move |c| c.edges.berserker = value),
            Edge3Name::Riposte => Box::new(move |c| c.edges.riposte = value),
            Edge3Name::Tuchfühlung => Box::new(move |c| c.edges.tuchfuhlung = value),
            Edge3Name::Lebenskraft => Box::new(move |c| c.edges.lebenskraft = value),
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
    fn as_str(&self, name: Edge3Name) -> &'static str {
        match (self, name) {
            (Edge3::None, _) => "Nein",
            (Edge3::Normal, Edge3Name::Blitzhieb) => "Blitzhieb",
            (Edge3::Improved, Edge3Name::Blitzhieb) => "Verb. Blitzhieb",
            (Edge3::Normal, Edge3Name::Berserker) => "Berserker",
            (Edge3::Improved, Edge3Name::Berserker) => "Berserker Sofort",
            (Edge3::Normal, Edge3Name::Riposte) => "Riposte",
            (Edge3::Improved, Edge3Name::Riposte) => "Verb. Riposte",
            (Edge3::Normal, Edge3Name::Tuchfühlung) => "Tuchfühlung",
            (Edge3::Improved, Edge3Name::Tuchfühlung) => "Meisterl. Tuchfühlung",
            (Edge3::Normal, Edge3Name::Lebenskraft) => "Lebenskraft",
            (Edge3::Improved, Edge3Name::Lebenskraft) => "Noch mehr Lebenskraft",
        }
    }

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
        ui.selectable_value(self, val, val.as_str(name))
            .on_hover_ui(|ui| {
                ui.horizontal(|ui| {
                    sim.gradient(name.modification_set(val)).draw(ui);
                });
            });
    }

    fn draw(&mut self, name: Edge3Name, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label(name.as_str());

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
        ui.label(name.as_str());
        let _ = ui.button(self.as_str(name));
    }
}
