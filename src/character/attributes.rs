use strum::IntoEnumIterator;

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Attributes {
    ges: Attribute,
    sta: Attribute,
    kon: Attribute,
    int: Attribute,
    wil: Attribute,
}

impl crate::app::Drawable for Attributes {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let grid = crate::app::create_grid("Attribute");

        ui.heading("Attribute");
        grid.show(ui, |ui| {
            ui.label("GES");
            self.ges.draw_ui(ui);
            ui.end_row();

            ui.label("STÄ");
            self.sta.draw_ui(ui);
            ui.end_row();

            ui.label("KON");
            self.kon.draw_ui(ui);
            ui.end_row();

            ui.label("INT");
            self.int.draw_ui(ui);
            ui.end_row();

            ui.label("WIL");
            self.wil.draw_ui(ui);
            ui.end_row();
        });
    }

    fn draw_gradients(&self, ui: &mut egui::Ui, simulator: &crate::simulator::Simulator) {
        let gradient_ges_dec = simulator.gradient(|char| char.attributes.ges.decrement());
        let gradient_ges_inc = simulator.gradient(|char| char.attributes.ges.increment());
        let gradient_sta_dec = simulator.gradient(|char| char.attributes.sta.decrement());
        let gradient_sta_inc = simulator.gradient(|char| char.attributes.sta.increment());
        let gradient_kon_dec = simulator.gradient(|char| char.attributes.kon.decrement());
        let gradient_kon_inc = simulator.gradient(|char| char.attributes.kon.increment());
        let gradient_int_dec = simulator.gradient(|char| char.attributes.int.decrement());
        let gradient_int_inc = simulator.gradient(|char| char.attributes.int.increment());
        let gradient_wil_dec = simulator.gradient(|char| char.attributes.wil.decrement());
        let gradient_wil_inc = simulator.gradient(|char| char.attributes.wil.increment());

        let grid = crate::app::create_grid("Attribute Gradienten");

        ui.heading("Gradienten");
        grid.show(ui, |ui| {
            ui.label("GES");
            gradient_ges_dec.draw_ui(ui);
            gradient_ges_inc.draw_ui(ui);
            ui.end_row();

            ui.label("STÄ");
            gradient_sta_dec.draw_ui(ui);
            gradient_sta_inc.draw_ui(ui);
            ui.end_row();

            ui.label("KON");
            gradient_kon_dec.draw_ui(ui);
            gradient_kon_inc.draw_ui(ui);
            ui.end_row();

            ui.label("INT");
            gradient_int_dec.draw_ui(ui);
            gradient_int_inc.draw_ui(ui);
            ui.end_row();

            ui.label("WIL");
            gradient_wil_dec.draw_ui(ui);
            gradient_wil_inc.draw_ui(ui);
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
pub enum Attribute {
    #[default]
    W4,
    W6,
    W8,
    W10,
    W12,
    W12p1,
    W12p2,
}

impl crate::app::Drawable for Attribute {
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

impl Attribute {
    fn as_str(&self) -> &'static str {
        match self {
            Self::W4 => "W4",
            Self::W6 => "W6",
            Self::W8 => "W8",
            Self::W10 => "W10",
            Self::W12 => "W12",
            Self::W12p1 => "W12+1",
            Self::W12p2 => "W12+2",
        }
    }

    fn increment(&mut self) {
        let new = match self {
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
            Self::W4 => Self::W4,
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
