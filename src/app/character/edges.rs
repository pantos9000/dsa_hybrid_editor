use crate::app::widgets::{self, BoolStat, DrawInfo, ValueSelector};
use crate::simulator::{CharModification, Simulator};

use super::Drawable;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Edges {
    pub(crate) lebenskraft: Edge3,
    pub(crate) blitzhieb: Edge3,
    pub(crate) berserker: Edge3,
    pub(crate) riposte: Edge3,
    pub(crate) tuchfuhlung: Edge3,
    pub(crate) kampfreflexe: BoolStat,
    pub(crate) erstschlag: BoolStat,
    pub(crate) beidhandiger_kampf: BoolStat,
    pub(crate) beidhandig: BoolStat,
    pub(crate) ubertolpeln: BoolStat,
    pub(crate) erbarmungslos: BoolStat,
    pub(crate) machtiger_hieb: BoolStat,
    pub(crate) schnell: BoolStat,
    pub(crate) kampfkunstler: BoolStat,
    pub(crate) kuhler_kopf: Edge3,
}

impl Edges {
    fn edge3_iter(&mut self) -> impl Iterator<Item = (&mut Edge3, Edge3Info)> {
        [
            (&mut self.berserker, Edge3Info::Berserker),
            (&mut self.tuchfuhlung, Edge3Info::Tuchfühlung),
            (&mut self.lebenskraft, Edge3Info::Lebenskraft),
            (&mut self.blitzhieb, Edge3Info::Blitzhieb),
            (&mut self.riposte, Edge3Info::Riposte),
            (&mut self.kuhler_kopf, Edge3Info::KühlerKopf),
        ]
        .into_iter()
    }

    fn edge2_iter(&mut self) -> impl Iterator<Item = (&mut BoolStat, Edge2Info)> {
        [
            (&mut self.schnell, Edge2Info::Schnell),
            (&mut self.beidhandig, Edge2Info::Beidhändig),
            (&mut self.beidhandiger_kampf, Edge2Info::BeidhändigerKampf),
            (&mut self.erstschlag, Edge2Info::Erstschlag),
            (&mut self.ubertolpeln, Edge2Info::Übertölpeln),
            (&mut self.machtiger_hieb, Edge2Info::MächtigerHieb),
            (&mut self.erbarmungslos, Edge2Info::Erbarmungslos),
            (&mut self.kampfreflexe, Edge2Info::Kampfreflexe),
            (&mut self.kampfkunstler, Edge2Info::Kampfkünstler),
        ]
        .into_iter()
    }
}

impl Drawable for Edges {
    fn draw(&mut self, sim: &mut Simulator, ui: &mut egui::Ui) {
        let grid = widgets::create_grid("Edges");

        ui.heading("Edges");
        grid.show(ui, |ui| {
            for (edge, info) in self.edge3_iter() {
                edge.draw(info, sim, ui);
                ui.end_row();
            }
            for (edge, info) in self.edge2_iter() {
                edge.draw(info, sim, ui);
                ui.end_row();
            }
        });
    }

    fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        let grid = widgets::create_grid("GegnerEdges");

        ui.heading("Edges");
        grid.show(ui, |ui| {
            for (edge, info) in self.edge3_iter() {
                edge.draw_as_opponent(info, ui);
                ui.end_row();
            }
            for (edge, info) in self.edge2_iter() {
                edge.draw_as_opponent(info, ui);
                ui.end_row();
            }
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edge2Info {
    Übertölpeln,
    Erbarmungslos,
    MächtigerHieb,
    Erstschlag,
    Kampfreflexe,
    Schnell,
    BeidhändigerKampf,
    Beidhändig,
    Kampfkünstler,
}

impl DrawInfo<BoolStat> for Edge2Info {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Übertölpeln => "Übertölpeln",
            Self::Erbarmungslos => "Erbarmungslos",
            Self::MächtigerHieb => "Mächtiger Hieb",
            Self::Erstschlag => "Erstschlag",
            Self::BeidhändigerKampf => "Beidhändiger Kampf",
            Self::Beidhändig => "Beidhändig (Hintergrund)",
            Self::Kampfreflexe => "Kampfreflexe",
            Self::Schnell => "Schnell (Hintergrund)",
            Self::Kampfkünstler => "Kampfkünstler",
        }
    }

    fn mod_dec(&self) -> CharModification {
        match self {
            Self::Übertölpeln => Box::new(|c| c.edges.ubertolpeln.decrement()),
            Self::Erbarmungslos => Box::new(|c| c.edges.erbarmungslos.decrement()),
            Self::MächtigerHieb => Box::new(|c| c.edges.machtiger_hieb.decrement()),
            Self::Erstschlag => Box::new(|c| c.edges.erstschlag.decrement()),
            Self::BeidhändigerKampf => Box::new(|c| c.edges.beidhandiger_kampf.decrement()),
            Self::Beidhändig => Box::new(|c| c.edges.beidhandig.decrement()),
            Self::Kampfreflexe => Box::new(|c| c.edges.kampfreflexe.decrement()),
            Self::Schnell => Box::new(|c| c.edges.schnell.decrement()),
            Self::Kampfkünstler => Box::new(|c| c.edges.kampfkunstler.decrement()),
        }
    }

    fn mod_inc(&self) -> CharModification {
        match self {
            Self::Übertölpeln => Box::new(|c| c.edges.ubertolpeln.increment()),
            Self::Erbarmungslos => Box::new(|c| c.edges.erbarmungslos.increment()),
            Self::MächtigerHieb => Box::new(|c| c.edges.machtiger_hieb.increment()),
            Self::Erstschlag => Box::new(|c| c.edges.erstschlag.increment()),
            Self::BeidhändigerKampf => Box::new(|c| c.edges.beidhandiger_kampf.increment()),
            Self::Beidhändig => Box::new(|c| c.edges.beidhandig.increment()),
            Self::Kampfreflexe => Box::new(|c| c.edges.kampfreflexe.increment()),
            Self::Schnell => Box::new(|c| c.edges.schnell.increment()),
            Self::Kampfkünstler => Box::new(|c| c.edges.kampfkunstler.increment()),
        }
    }

    fn mod_set(&self, value: BoolStat) -> CharModification {
        match self {
            Self::Übertölpeln => Box::new(move |c| c.edges.ubertolpeln.set(value)),
            Self::Erbarmungslos => Box::new(move |c| c.edges.erbarmungslos.set(value)),
            Self::MächtigerHieb => Box::new(move |c| c.edges.machtiger_hieb.set(value)),
            Self::Erstschlag => Box::new(move |c| c.edges.erstschlag.set(value)),
            Self::BeidhändigerKampf => Box::new(move |c| c.edges.beidhandiger_kampf.set(value)),
            Self::Beidhändig => Box::new(move |c| c.edges.beidhandig.set(value)),
            Self::Kampfreflexe => Box::new(move |c| c.edges.kampfreflexe.set(value)),
            Self::Schnell => Box::new(move |c| c.edges.schnell.set(value)),
            Self::Kampfkünstler => Box::new(move |c| c.edges.kampfkunstler.set(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge3Info {
    Blitzhieb,
    Berserker,
    Riposte,
    Tuchfühlung,
    Lebenskraft,
    KühlerKopf,
}

impl DrawInfo<Edge3> for Edge3Info {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Blitzhieb => "Blitzhieb",
            Self::Berserker => "Berserker (Hintergrund)",
            Self::Riposte => "Riposte",
            Self::Tuchfühlung => "Tuchfühlung",
            Self::Lebenskraft => "Lebenskraft",
            Self::KühlerKopf => "Kühler Kopf",
        }
    }

    fn mod_dec(&self) -> CharModification {
        match self {
            Self::Blitzhieb => Box::new(|c| c.edges.blitzhieb.decrement()),
            Self::Berserker => Box::new(|c| c.edges.berserker.decrement()),
            Self::Riposte => Box::new(|c| c.edges.riposte.decrement()),
            Self::Tuchfühlung => Box::new(|c| c.edges.tuchfuhlung.decrement()),
            Self::Lebenskraft => Box::new(|c| c.edges.lebenskraft.decrement()),
            Self::KühlerKopf => Box::new(|c| c.edges.kuhler_kopf.decrement()),
        }
    }

    fn mod_inc(&self) -> CharModification {
        match self {
            Self::Blitzhieb => Box::new(|c| c.edges.blitzhieb.increment()),
            Self::Berserker => Box::new(|c| c.edges.berserker.increment()),
            Self::Riposte => Box::new(|c| c.edges.riposte.increment()),
            Self::Tuchfühlung => Box::new(|c| c.edges.tuchfuhlung.increment()),
            Self::Lebenskraft => Box::new(|c| c.edges.lebenskraft.increment()),
            Self::KühlerKopf => Box::new(|c| c.edges.kuhler_kopf.increment()),
        }
    }

    fn mod_set(&self, value: Edge3) -> CharModification {
        match self {
            Self::Blitzhieb => Box::new(move |c| c.edges.blitzhieb = value),
            Self::Berserker => Box::new(move |c| c.edges.berserker = value),
            Self::Riposte => Box::new(move |c| c.edges.riposte = value),
            Self::Tuchfühlung => Box::new(move |c| c.edges.tuchfuhlung = value),
            Self::Lebenskraft => Box::new(move |c| c.edges.lebenskraft = value),
            Self::KühlerKopf => Box::new(move |c| c.edges.kuhler_kopf = value),
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
pub enum Edge3 {
    #[default]
    None,
    Normal,
    Improved,
}

impl ValueSelector for Edge3 {
    type Info = Edge3Info;

    fn possible_values() -> impl Iterator<Item = Self> {
        use strum::IntoEnumIterator as _;

        Self::iter()
    }

    fn as_str(&self, info: &Self::Info) -> &'static str {
        match (self, info) {
            (Self::None, _) => "Nein",
            (Self::Normal, Edge3Info::Blitzhieb) => "Blitzhieb",
            (Self::Improved, Edge3Info::Blitzhieb) => "Verb. Blitzhieb",
            (Self::Normal, Edge3Info::Berserker) => "Berserker",
            (Self::Improved, Edge3Info::Berserker) => "Berserker Sofort",
            (Self::Normal, Edge3Info::Riposte) => "Riposte",
            (Self::Improved, Edge3Info::Riposte) => "Verb. Riposte",
            (Self::Normal, Edge3Info::Tuchfühlung) => "Tuchfühlung",
            (Self::Improved, Edge3Info::Tuchfühlung) => "Meisterl. Tuchfühlung",
            (Self::Normal, Edge3Info::Lebenskraft) => "Lebenskraft",
            (Self::Improved, Edge3Info::Lebenskraft) => "Noch mehr Lebenskraft",
            (Self::Normal, Edge3Info::KühlerKopf) => "Kühler Kopf",
            (Self::Improved, Edge3Info::KühlerKopf) => "Kühlerer Kopf",
        }
    }
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
}
