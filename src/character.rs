mod attributes;
mod name;
mod skills;
mod weapon;

pub use attributes::Attributes;
use egui::Layout;
pub use name::Name;
pub use skills::Skills;
pub use weapon::Weapon;

use crate::util::LogError;
use crate::{simulator::Simulator, util};

use anyhow::{Context, Result};

/// Represents a drawable element of a char
trait Drawable {
    fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui);
    fn draw_as_opponent(&mut self, ui: &mut egui::Ui);
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Character {
    pub(crate) name: Name,
    pub(crate) attributes: Attributes,
    pub(crate) skills: Skills,
    pub(crate) weapon: Weapon,
}

impl Character {
    const BUTTON_SIZE: [f32; 2] = [40.0, 40.0];

    pub fn draw(&mut self, sim: &Simulator, ui: &mut egui::Ui) {
        util::create_frame(ui).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    self.draw_buttons(ui);
                    ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                        let no_mod = Box::new(|_: &mut Character| ());
                        sim.gradient(no_mod).draw_sized(Self::BUTTON_SIZE, ui);
                    });
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.name.draw(sim, ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.attributes.draw(sim, ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.skills.draw(sim, ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.weapon.draw(sim, ui);
                });
            });
        });
    }

    pub fn draw_as_opponent(&mut self, ui: &mut egui::Ui) {
        util::create_frame(ui).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    self.draw_buttons(ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.name.draw_as_opponent(ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.attributes.draw_as_opponent(ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.skills.draw_as_opponent(ui);
                });
                util::create_frame(ui).show(ui, |ui| {
                    self.weapon.draw_as_opponent(ui);
                });
            });
        });
    }

    fn draw_buttons(&mut self, ui: &mut egui::Ui) {
        let mut add_button = |text| -> egui::Response {
            let text = egui::RichText::new(text).size(24.0);
            let button = egui::Button::new(text).rounding(10.0);
            ui.add_sized(Self::BUTTON_SIZE, button)
        };

        let reset = "❌";
        let save = "💾";
        let open = "🗁";
        if add_button(save).clicked() {
            self.save().or_log_err("failed to save character");
        }
        if add_button(open).clicked() {
            self.load().or_log_err("failed to load character");
        }
        if add_button(reset).clicked() {
            *self = Default::default();
        }
    }

    fn load(&mut self) -> Result<()> {
        log::info!("loading char...");

        let future = async {
            let file = rfd::AsyncFileDialog::new().pick_file().await?;
            Some(file.read().await)
        };
        let Some(data) = async_std::task::block_on(future) else {
            log::debug!("file pick dialog was canceled");
            return Ok(());
        };

        let new_char = serde_json::from_slice(&data)
            .context("failed to convert character from JSON format")?;
        *self = new_char;

        log::info!("successfully loaded char");
        Ok(())
    }

    fn save(&self) -> Result<()> {
        log::info!("saving char...");

        async fn save_file(char_serialized: &[u8]) -> Result<()> {
            let Some(file) = rfd::AsyncFileDialog::new().save_file().await else {
                log::debug!("save file dialog was canceled");
                return Ok(());
            };
            file.write(char_serialized)
                .await
                .context("failed to write to file")?;
            Ok(())
        }

        let char_serialized = serde_json::to_vec_pretty(self)
            .context("failed to convert character to JSON format")?;
        async_std::task::block_on(save_file(&char_serialized))
            .context("failed to save char to file")?;

        log::info!("successfully saved char");
        Ok(())
    }
}
