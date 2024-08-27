use strum::IntoEnumIterator;

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Weapon {
    no_strength: bool,
    damage: Damage,
    bonus_damage: BonusDamage,
}

impl crate::app::Drawable for Weapon {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let grid = crate::app::create_grid("Waffe");

        ui.heading("Waffe");
        grid.show(ui, |ui| {
            ui.label("Keine Stärke");
            ui.checkbox(
                &mut self.no_strength,
                "(Benutze Schadenswürfel statt Stärke)",
            );
            ui.end_row();

            ui.label("Schaden");
            ui.horizontal(|ui| {
                self.damage.draw_ui(ui);
            });
            ui.end_row();

            ui.label("Schadensbonus");
            self.bonus_damage.draw_ui(ui);
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
        let slider = egui::Slider::new(&mut self.0, -3..=3);
        ui.add(slider);
    }
}
