use strum::IntoEnumIterator;

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Weapon {
    damage: Damage,
    bonus_damage: BonusDamage,
}

impl crate::app::Drawable for Weapon {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let grid = crate::util::create_grid("Waffe");

        ui.heading("Waffe");
        grid.show(ui, |ui| {
            ui.label("Schaden");
            self.damage.draw_ui(ui);
            ui.end_row();

            ui.label("Schadensbonus");
            self.bonus_damage.draw_ui(ui);
            ui.end_row();
        });
    }

    fn draw_gradients(&self, ui: &mut egui::Ui, simulator: &crate::simulator::Simulator) {
        let gradient_damage_dec = simulator.gradient(|char| char.weapon.damage.decrement());
        let gradient_damage_inc = simulator.gradient(|char| char.weapon.damage.increment());
        let gradient_bonus_dec = simulator.gradient(|char| char.weapon.bonus_damage.decrement());
        let gradient_bonus_inc = simulator.gradient(|char| char.weapon.bonus_damage.increment());

        let grid = crate::util::create_grid("Waffe Gradienten");

        ui.heading("Gradienten");
        grid.show(ui, |ui| {
            ui.label("Schaden");
            gradient_damage_dec.draw_ui(ui);
            gradient_damage_inc.draw_ui(ui);
            ui.end_row();

            ui.label("Schadensbonus");
            gradient_bonus_dec.draw_ui(ui);
            gradient_bonus_inc.draw_ui(ui);
            ui.end_row();
        });
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Damage {
    #[default]
    W4,
    W6,
    W8,
    W10,
    W12,
}

impl crate::app::Drawable for Damage {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for val in Self::iter() {
                ui.selectable_value(self, val, val.as_str());
            }
        });
    }

    fn draw_gradients(&self, _ui: &mut egui::Ui, _simulatorr: &crate::simulator::Simulator) {
        unreachable!();
    }
}

impl Damage {
    fn as_str(&self) -> &'static str {
        match self {
            Damage::W4 => "W4",
            Damage::W6 => "W6",
            Damage::W8 => "W8",
            Damage::W10 => "W10",
            Damage::W12 => "W12",
        }
    }

    fn decrement(&mut self) {
        let new = match self {
            Self::W4 => Self::W4,
            Self::W6 => Self::W4,
            Self::W8 => Self::W6,
            Self::W10 => Self::W8,
            Self::W12 => Self::W10,
        };
        *self = new;
    }

    fn increment(&mut self) {
        let new = match self {
            Self::W4 => Self::W6,
            Self::W6 => Self::W8,
            Self::W8 => Self::W10,
            Self::W10 => Self::W12,
            Self::W12 => Self::W12,
        };
        *self = new;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BonusDamage(i32);

impl From<i32> for BonusDamage {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<BonusDamage> for i32 {
    fn from(value: BonusDamage) -> Self {
        value.0
    }
}

impl crate::app::Drawable for BonusDamage {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let slider = egui::Slider::new(&mut self.0, Self::MIN..=Self::MAX);
        ui.add(slider);
    }

    fn draw_gradients(&self, _ui: &mut egui::Ui, _simulator: &crate::simulator::Simulator) {
        unreachable!();
    }
}

impl BonusDamage {
    const MIN: i32 = -3;
    const MAX: i32 = 3;

    fn decrement(&mut self) {
        let new = match self.0 {
            val if val < Self::MIN => unreachable!(),
            val if val > Self::MAX => unreachable!(),
            Self::MIN => Self::MIN,
            val => val - 1,
        };
        self.0 = new;
    }

    fn increment(&mut self) {
        let new = match self.0 {
            val if val < Self::MIN => unreachable!(),
            val if val > Self::MAX => unreachable!(),
            Self::MAX => Self::MAX,
            val => val + 1,
        };
        self.0 = new;
    }
}
