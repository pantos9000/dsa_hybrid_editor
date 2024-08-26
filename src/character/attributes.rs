use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Attributes {
    ges: Attribute,
    sta: Attribute,
    kon: Attribute,
    int: Attribute,
    wil: Attribute,
}

impl Attributes {
    pub fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let grid = egui::Grid::new("Attribute").striped(true);

        ui.label("Attribute");
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
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter, serde::Serialize, serde::Deserialize,
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

impl Attribute {
    fn as_str(&self) -> &'static str {
        match self {
            Attribute::W4 => "W4",
            Attribute::W6 => "W6",
            Attribute::W8 => "W8",
            Attribute::W10 => "W10",
            Attribute::W12 => "W12",
            Attribute::W12p1 => "W12+1",
            Attribute::W12p2 => "W12+2",
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
