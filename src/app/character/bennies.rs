use crate::app::widgets::{self, BoolStat, DrawInfo, IntStat, ValueSlider as _};
use crate::simulator::{CharModification, Simulator};
use crate::{app, simulator};

use super::Drawable;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Bennies {
    pub(crate) count: IntStat<0, 10>,
    pub(crate) use_for_unshake: BoolStat,
    #[serde(default)]
    pub(crate) use_for_erstschlag: BoolStat,
    pub(crate) use_against_erstschlag: BoolStat,
    pub(crate) use_for_attack: BoolStat,
    pub(crate) use_for_damage: BoolStat,
}

impl Drawable for Bennies {
    fn draw(&mut self, selection: app::CharSelection, sim: &mut Simulator, ui: &mut egui::Ui) {
        let grid = widgets::create_grid("Bennies");

        ui.heading("Bennies");
        grid.show(ui, |ui| {
            self.count.draw(NumBennies, selection, sim, ui);
            ui.end_row();
            self.use_for_unshake
                .draw(UsageInfo::Unshake, selection, sim, ui);
            ui.end_row();
            self.use_for_erstschlag
                .draw(UsageInfo::ForErstschlag, selection, sim, ui);
            ui.end_row();
            self.use_against_erstschlag
                .draw(UsageInfo::AgainstErstschlag, selection, sim, ui);
            ui.end_row();
            self.use_for_attack
                .draw(UsageInfo::Attack, selection, sim, ui);
            ui.end_row();
            self.use_for_damage
                .draw(UsageInfo::Damage, selection, sim, ui);
            ui.end_row();
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumBennies;

impl<const MIN: i8, const MAX: i8> DrawInfo<IntStat<MIN, MAX>> for NumBennies {
    fn as_str(&self) -> &'static str {
        "Anzahl Bennies"
    }

    fn mod_dec(&self, selection: app::CharSelection) -> CharModification {
        let modification: simulator::CharModFunc = Box::new(|c| c.bennies.count.decrement());
        simulator::CharModification::new(selection, modification)
    }

    fn mod_inc(&self, selection: app::CharSelection) -> CharModification {
        let modification: simulator::CharModFunc = Box::new(|c| c.bennies.count.increment());
        simulator::CharModification::new(selection, modification)
    }

    fn mod_set(&self, selection: app::CharSelection, value: IntStat<MIN, MAX>) -> CharModification {
        let modification: simulator::CharModFunc =
            Box::new(move |c| c.bennies.count.set(value.into()));
        simulator::CharModification::new(selection, modification)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UsageInfo {
    Unshake,
    ForErstschlag,
    AgainstErstschlag,
    Attack,
    Damage,
}

impl DrawInfo<BoolStat> for UsageInfo {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Unshake => "Nutzen für Entschütteln",
            Self::ForErstschlag => "Nutzen für Entschütteln um Erstschlag zu ermöglichen",
            Self::AgainstErstschlag => "Nutzen für Entschütteln um Erstschlag zu verhindern",
            Self::Attack => "Nutzen für Angriffe",
            Self::Damage => "Nutzen für Schaden",
        }
    }

    fn mod_dec(&self, selection: app::CharSelection) -> CharModification {
        let modification: simulator::CharModFunc = match self {
            Self::Unshake => Box::new(|c| c.bennies.use_for_unshake.decrement()),
            Self::ForErstschlag => Box::new(|c| c.bennies.use_for_erstschlag.decrement()),
            Self::AgainstErstschlag => Box::new(|c| c.bennies.use_against_erstschlag.decrement()),
            Self::Attack => Box::new(|c| c.bennies.use_for_attack.decrement()),
            Self::Damage => Box::new(|c| c.bennies.use_for_damage.decrement()),
        };
        simulator::CharModification::new(selection, modification)
    }

    fn mod_inc(&self, selection: app::CharSelection) -> CharModification {
        let modification: simulator::CharModFunc = match self {
            Self::Unshake => Box::new(|c| c.bennies.use_for_unshake.increment()),
            Self::ForErstschlag => Box::new(|c| c.bennies.use_for_erstschlag.increment()),
            Self::AgainstErstschlag => Box::new(|c| c.bennies.use_against_erstschlag.increment()),
            Self::Attack => Box::new(|c| c.bennies.use_for_attack.increment()),
            Self::Damage => Box::new(|c| c.bennies.use_for_damage.increment()),
        };
        simulator::CharModification::new(selection, modification)
    }

    fn mod_set(&self, selection: app::CharSelection, value: BoolStat) -> CharModification {
        let modification: simulator::CharModFunc = match self {
            Self::Unshake => Box::new(move |c| c.bennies.use_for_unshake.set(value)),
            Self::ForErstschlag => Box::new(move |c| c.bennies.use_for_erstschlag.set(value)),
            Self::AgainstErstschlag => {
                Box::new(move |c| c.bennies.use_against_erstschlag.set(value))
            }
            Self::Attack => Box::new(move |c| c.bennies.use_for_attack.set(value)),
            Self::Damage => Box::new(move |c| c.bennies.use_for_damage.set(value)),
        };
        simulator::CharModification::new(selection, modification)
    }
}
