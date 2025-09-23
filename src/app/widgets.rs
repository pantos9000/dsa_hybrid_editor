use crate::{
    app,
    simulator::{CharModification, Simulator},
};

pub trait DrawInfo<Value> {
    fn as_str(&self) -> &'static str;
    fn mod_dec(&self, selection: app::CharSelection) -> CharModification;
    fn mod_inc(&self, selection: app::CharSelection) -> CharModification;
    fn mod_set(&self, selection: app::CharSelection, value: Value) -> CharModification;
}

pub trait ValueSelector: Sized + Copy + Eq {
    type Info: DrawInfo<Self>;

    fn possible_values() -> impl Iterator<Item = Self>;
    fn as_str(&self, info: &Self::Info) -> &'static str;

    fn draw(
        &mut self,
        info: Self::Info,
        selection: app::CharSelection,
        sim: &mut Simulator,
        ui: &mut egui::Ui,
    ) {
        ui.label(info.as_str());

        ui.horizontal(|ui| {
            for val in Self::possible_values() {
                ui.selectable_value(self, val, val.as_str(&info))
                    .on_hover_ui(|ui| {
                        ui.horizontal(|ui| {
                            sim.gradient(info.mod_set(selection, val)).draw(ui);
                        });
                    });
            }
        });

        ui.horizontal(|ui| {
            sim.gradient(info.mod_dec(selection)).draw(ui);
            sim.gradient(info.mod_inc(selection)).draw(ui);
        });
    }
}

pub trait ValueSlider: Sized {
    fn min() -> i8;
    fn max() -> i8;
    fn inner_mut(&mut self) -> &mut i8;

    fn set(&mut self, value: i8) {
        *self.inner_mut() = value.clamp(Self::min(), Self::max());
    }

    fn decrement(&mut self) {
        let min = Self::min();
        let max = Self::max();
        let new = match *self.inner_mut() {
            val if val < min => unreachable!(),
            val if val > max => unreachable!(),
            val if val == min => min,
            val => val - 1,
        };
        *self.inner_mut() = new;
    }

    fn increment(&mut self) {
        let min = Self::min();
        let max = Self::max();
        let new = match *self.inner_mut() {
            val if val < min => unreachable!(),
            val if val > max => unreachable!(),
            val if val == max => max,
            val => val + 1,
        };
        *self.inner_mut() = new;
    }

    fn draw(
        &mut self,
        info: impl DrawInfo<Self>,
        selection: app::CharSelection,
        sim: &mut Simulator,
        ui: &mut egui::Ui,
    ) {
        ui.label(info.as_str());

        let slider = egui::Slider::new(self.inner_mut(), Self::min()..=Self::max());
        ui.add(slider);

        ui.horizontal(|ui| {
            sim.gradient(info.mod_dec(selection)).draw(ui);
            sim.gradient(info.mod_inc(selection)).draw(ui);
        });
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct IntStat<const MIN: i8, const MAX: i8>(i8);

impl<const MIN: i8, const MAX: i8> ValueSlider for IntStat<MIN, MAX> {
    fn min() -> i8 {
        MIN
    }

    fn max() -> i8 {
        MAX
    }

    fn inner_mut(&mut self) -> &mut i8 {
        &mut self.0
    }
}

impl<const MIN: i8, const MAX: i8> From<IntStat<MIN, MAX>> for i8 {
    fn from(value: IntStat<MIN, MAX>) -> Self {
        value.0
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct BoolStat(bool);

impl BoolStat {
    pub fn decrement(&mut self) {
        self.0 = false;
    }

    pub fn increment(&mut self) {
        self.0 = true;
    }

    pub fn set(&mut self, value: Self) {
        self.0 = value.0;
    }

    pub fn is_set(self) -> bool {
        self.0
    }

    fn toggled(self) -> Self {
        Self(!self.0)
    }

    pub fn draw(
        &mut self,
        info: impl DrawInfo<Self>,
        selection: app::CharSelection,
        sim: &mut Simulator,
        ui: &mut egui::Ui,
    ) {
        ui.checkbox(&mut self.0, info.as_str()).on_hover_ui(|ui| {
            ui.horizontal(|ui| {
                sim.gradient(info.mod_set(selection, self.toggled()))
                    .draw(ui);
            });
        });

        ui.horizontal(|ui| {
            sim.gradient(info.mod_dec(selection)).draw(ui);
            sim.gradient(info.mod_inc(selection)).draw(ui);
        });
    }
}

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
        ui.horizontal(|ui| {
            ui.label(help);
        });
    })
}
