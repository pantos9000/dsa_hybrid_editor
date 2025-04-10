use crate::simulator::{CharModification, Simulator};

pub fn create_grid(id_salt: impl std::hash::Hash) -> egui::Grid {
    egui::Grid::new(id_salt)
        .num_columns(2)
        .min_col_width(80.0)
        .spacing([20.0, 4.0])
        .striped(true)
}

pub fn create_frame(ui: &egui::Ui) -> egui::Frame {
    egui::Frame::default()
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .corner_radius(ui.visuals().widgets.noninteractive.corner_radius)
        .inner_margin(10.0)
        .outer_margin(5.0)
        .fill(egui::Color32::TRANSPARENT)
}

pub fn create_menu_button(text: &str, help: &str, size: f32, ui: &mut egui::Ui) -> egui::Response {
    let text_size = size * 0.6;
    let text = egui::RichText::new(text).size(text_size);
    let button = egui::Button::new(text).corner_radius(10.0);
    ui.add_sized([size, size], button).on_hover_ui(|ui| {
        egui::show_tooltip(ui.ctx(), ui.layer_id(), egui::Id::new("my_tooltip"), |ui| {
            ui.horizontal(|ui| {
                ui.label(help);
            });
        });
    })
}

pub trait LogError {
    fn or_log_err(self, context: &str);
}

impl<E> LogError for Result<(), E>
where
    E: std::fmt::Display,
{
    fn or_log_err(self, context: &str) {
        let Err(err) = self else {
            return;
        };
        log::error!("{context}: {err}");
    }
}

pub trait ModifierName {
    fn as_str(&self) -> &str;
    fn modification_dec(&self) -> CharModification;
    fn modification_inc(&self) -> CharModification;
    // fn modification_set<const MIN: i8, const MAX: i8>(
    //     &self,
    //     value: Modifier<MIN, MAX>,
    // ) -> CharModification;
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Modifier<const MIN: i8, const MAX: i8>(i8);

impl<const MIN: i8, const MAX: i8> From<i8> for Modifier<MIN, MAX> {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl<const MIN: i8, const MAX: i8> From<Modifier<MIN, MAX>> for i8 {
    fn from(value: Modifier<MIN, MAX>) -> Self {
        value.0
    }
}

impl<const MIN: i8, const MAX: i8> Modifier<MIN, MAX> {
    fn as_string(&self) -> String {
        format!("{}", self.0)
    }

    pub fn draw(&mut self, name: impl ModifierName, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label(name.as_str());

        let slider = egui::Slider::new(&mut self.0, MIN..=MAX);
        ui.add(slider);

        ui.horizontal(|ui| {
            sim.gradient(name.modification_dec()).draw(ui);
            sim.gradient(name.modification_inc()).draw(ui);
        });
    }

    pub fn draw_as_opponent(&mut self, name: impl ModifierName, ui: &mut egui::Ui) {
        ui.label(name.as_str());
        let _ = ui.button(self.as_string());
    }

    pub fn decrement(&mut self) {
        let new = match self.0 {
            val if val < MIN => unreachable!(),
            val if val > MAX => unreachable!(),
            val if val == MIN => MIN,
            val => val - 1,
        };
        self.0 = new;
    }

    pub fn increment(&mut self) {
        let new = match self.0 {
            val if val < MIN => unreachable!(),
            val if val > MAX => unreachable!(),
            val if val == MAX => MAX,
            val => val + 1,
        };
        self.0 = new;
    }
}
