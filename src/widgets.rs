use crate::simulator::{CharModification, Simulator};

pub trait DrawInfo<Value> {
    fn as_str(&self) -> &'static str;
    fn mod_dec(&self) -> CharModification;
    fn mod_inc(&self) -> CharModification;
    fn mod_set(&self, value: Value) -> CharModification;
}

pub trait ValueSelector: Sized + Copy + Eq {
    type Info: DrawInfo<Self>;

    fn possible_values() -> impl Iterator<Item = Self>;
    fn as_str(&self, info: &Self::Info) -> &'static str;

    fn draw(&mut self, info: Self::Info, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label(info.as_str());

        ui.horizontal(|ui| {
            for val in Self::possible_values() {
                ui.selectable_value(self, val, val.as_str(&info))
                    .on_hover_ui(|ui| {
                        ui.horizontal(|ui| {
                            sim.gradient(info.mod_set(val)).draw(ui);
                        });
                    });
            }
        });

        ui.horizontal(|ui| {
            sim.gradient(info.mod_dec()).draw(ui);
            sim.gradient(info.mod_inc()).draw(ui);
        });
    }

    fn draw_as_opponent(&mut self, info: Self::Info, ui: &mut egui::Ui) {
        ui.label(info.as_str());
        let _ = ui.button(self.as_str(&info));
    }
}

pub trait ValueSlider: Sized {
    fn min() -> i8;
    fn max() -> i8;
    fn inner_mut(&mut self) -> &mut i8;
    fn as_string(&self) -> String;

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

    fn draw(&mut self, info: impl DrawInfo<Self>, sim: &Simulator, ui: &mut egui::Ui) {
        ui.label(info.as_str());

        let slider = egui::Slider::new(self.inner_mut(), Self::min()..=Self::max());
        ui.add(slider);

        ui.horizontal(|ui| {
            sim.gradient(info.mod_dec()).draw(ui);
            sim.gradient(info.mod_inc()).draw(ui);
        });
    }

    fn draw_as_opponent(&mut self, info: impl DrawInfo<Self>, ui: &mut egui::Ui) {
        ui.label(info.as_str());
        let _ = ui.button(self.as_string());
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

    fn as_string(&self) -> String {
        format!("{}", self.0)
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
    fn as_str(self) -> &'static str {
        if self.0 {
            "Ja"
        } else {
            "Nein"
        }
    }

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

    pub fn draw(&mut self, info: impl DrawInfo<Self>, sim: &Simulator, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.0, info.as_str()).on_hover_ui(|ui| {
            ui.horizontal(|ui| {
                sim.gradient(info.mod_set(self.toggled())).draw(ui);
            });
        });

        ui.horizontal(|ui| {
            sim.gradient(info.mod_dec()).draw(ui);
            sim.gradient(info.mod_inc()).draw(ui);
        });
    }

    pub fn draw_as_opponent(&mut self, info: impl DrawInfo<Self>, ui: &mut egui::Ui) {
        ui.label(info.as_str());
        let _ = ui.button(self.as_str());
    }
}

// TODO: slider
