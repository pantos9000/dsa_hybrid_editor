use crate::app::widgets::{self, BoolStat, DrawInfo, IntStat, ValueSlider as _};
use crate::simulator::{CharModification, Simulator};

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
    fn draw(&mut self, sim: &mut Simulator, ui: &mut egui::Ui) {
        let grid = widgets::create_grid("Bennies");

        ui.heading("Bennies");
        grid.show(ui, |ui| {
            self.count.draw(NumBennies, sim, ui);
            ui.end_row();
            self.use_for_unshake.draw(UsageInfo::Unshake, sim, ui);
            ui.end_row();
            self.use_for_erstschlag
                .draw(UsageInfo::ForErstschlag, sim, ui);
            ui.end_row();
            self.use_against_erstschlag
                .draw(UsageInfo::AgainstErstschlag, sim, ui);
            ui.end_row();
            self.use_for_attack.draw(UsageInfo::Attack, sim, ui);
            ui.end_row();
            self.use_for_damage.draw(UsageInfo::Damage, sim, ui);
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

    fn mod_dec(&self) -> CharModification {
        Box::new(|c| c.bennies.count.decrement())
    }

    fn mod_inc(&self) -> CharModification {
        Box::new(|c| c.bennies.count.increment())
    }

    fn mod_set(&self, value: IntStat<MIN, MAX>) -> CharModification {
        Box::new(move |c| c.bennies.count.set(value.into()))
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

    fn mod_dec(&self) -> CharModification {
        match self {
            Self::Unshake => Box::new(|c| c.bennies.use_for_unshake.decrement()),
            Self::ForErstschlag => Box::new(|c| c.bennies.use_for_erstschlag.decrement()),
            Self::AgainstErstschlag => Box::new(|c| c.bennies.use_against_erstschlag.decrement()),
            Self::Attack => Box::new(|c| c.bennies.use_for_attack.decrement()),
            Self::Damage => Box::new(|c| c.bennies.use_for_damage.decrement()),
        }
    }

    fn mod_inc(&self) -> CharModification {
        match self {
            Self::Unshake => Box::new(|c| c.bennies.use_for_unshake.increment()),
            Self::ForErstschlag => Box::new(|c| c.bennies.use_for_erstschlag.increment()),
            Self::AgainstErstschlag => Box::new(|c| c.bennies.use_against_erstschlag.increment()),
            Self::Attack => Box::new(|c| c.bennies.use_for_attack.increment()),
            Self::Damage => Box::new(|c| c.bennies.use_for_damage.increment()),
        }
    }

    fn mod_set(&self, value: BoolStat) -> CharModification {
        match self {
            Self::Unshake => Box::new(move |c| c.bennies.use_for_unshake.set(value)),
            Self::ForErstschlag => Box::new(move |c| c.bennies.use_for_erstschlag.set(value)),
            Self::AgainstErstschlag => {
                Box::new(move |c| c.bennies.use_against_erstschlag.set(value))
            }
            Self::Attack => Box::new(move |c| c.bennies.use_for_attack.set(value)),
            Self::Damage => Box::new(move |c| c.bennies.use_for_damage.set(value)),
        }
    }
}
