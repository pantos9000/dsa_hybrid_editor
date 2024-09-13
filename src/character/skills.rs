use strum::IntoEnumIterator;

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Skills {
    kampfen: Skill,
}

impl crate::app::Drawable for Skills {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let grid = crate::app::create_grid("Fähigkeiten");

        ui.heading("Fähigkeiten");
        grid.show(ui, |ui| {
            ui.label("Kämpfen");
            self.kampfen.draw_ui(ui);
            ui.end_row();
        });
    }

    fn draw_gradients(&self, ui: &mut egui::Ui, simulator: &crate::simulator::Simulator) {
        let gradient_kam_dec = simulator.gradient(|char| char.skills.kampfen.decrement());
        let gradient_kam_inc = simulator.gradient(|char| char.skills.kampfen.increment());

        let grid = crate::app::create_grid("Fähigkeiten Gradienten");

        ui.heading("Gradienten");
        grid.show(ui, |ui| {
            ui.label("Kämpfen");
            gradient_kam_dec.draw_ui(ui);
            gradient_kam_inc.draw_ui(ui);
        });
        ui.end_row();
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

impl crate::app::Drawable for Skill {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for val in Self::iter() {
                ui.selectable_value(self, val, val.as_str());
            }
        });
    }

    fn draw_gradients(&self, _ui: &mut egui::Ui, _simulator: &crate::simulator::Simulator) {
        unreachable!();
    }
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

    fn increment(&mut self) {
        let new = match self {
            Self::W4m2 => Self::W4,
            Self::W4 => Self::W6,
            Self::W6 => Self::W8,
            Self::W8 => Self::W10,
            Self::W10 => Self::W12,
            Self::W12 => Self::W12p1,
            Self::W12p1 => Self::W12p2,
            Self::W12p2 => Self::W12p2,
        };
        *self = new;
    }

    fn decrement(&mut self) {
        let new = match self {
            Self::W4m2 => Self::W4m2,
            Self::W4 => Self::W4m2,
            Self::W6 => Self::W4,
            Self::W8 => Self::W6,
            Self::W10 => Self::W8,
            Self::W12 => Self::W10,
            Self::W12p1 => Self::W12,
            Self::W12p2 => Self::W12p1,
        };
        *self = new;
    }
}
