use strum::IntoEnumIterator;

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Skills {
    kampfen: Skill,
}

impl Skills {
    pub fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let grid = egui::Grid::new("Fähigkeiten").striped(true);

        ui.heading("Fähigkeiten");
        grid.show(ui, |ui| {
            ui.label("Kämpfen");
            self.kampfen.draw_ui(ui);
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
pub enum Skill {
    #[default]
    W4m2,
    W4,
    W6,
    W8,
    W10,
    W12,
    W12p1,
    W12p2,
}

impl Skill {
    fn as_str(&self) -> &'static str {
        match self {
            Skill::W4m2 => "W4-2",
            Skill::W4 => "W4",
            Skill::W6 => "W6",
            Skill::W8 => "W8",
            Skill::W10 => "W10",
            Skill::W12 => "W12",
            Skill::W12p1 => "W12+1",
            Skill::W12p2 => "W12+2",
        }
    }

    pub fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for val in Self::iter() {
                ui.selectable_value(self, val, val.as_str());
            }
        });
    }
}
