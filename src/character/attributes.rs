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
}
