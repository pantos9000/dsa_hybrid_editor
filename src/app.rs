use anyhow::{Context, Result};

use crate::character::Character;
use crate::simulator::Simulator;

pub trait Drawable {
    fn draw(&mut self, ui: &mut egui::Ui);
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct App {
    char: Character,
    #[serde(skip)] // don't save combat simulator state
    simulator: Simulator,
}

impl App {
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = creation_context.storage {
            log::info!("found previous state, restoring...");
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        log::info!("creating new app context");
        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        self.simulator.update_character(&self.char);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                self.menu_bar(ui, ctx);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("DSA Hybrid Char Editor");
            ui.separator();

            ui.vertical(|ui| {
                self.char.draw(ui);
                // TODO: show opponent name
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
                ui.separator();
            });
        });
    }
}

impl App {
    fn menu_bar(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let is_web = cfg!(target_arch = "wasm32"); // no File->Quit on web pages

        ui.menu_button("File", |ui| {
            if ui.button("Load char from file...").clicked() {
                if let Err(err) = self.load_char() {
                    log::error!("failed to load char: {err}");
                }
                ui.close_menu();
            }
            if ui.button("Save char to file...").clicked() {
                if let Err(err) = self.save_char() {
                    log::error!("failed to save char: {err}");
                }
                ui.close_menu();
            }
            if !is_web {
                ui.separator();
                if ui.button("Quit").clicked() {
                    log::info!("quit button clicked, exiting...");
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        });
        ui.add_space(16.0);
        // TODO: load opponent

        ui.menu_button("Theme", |ui| {
            egui::widgets::global_theme_preference_buttons(ui);
        });
    }

    fn load_char(&mut self) -> Result<()> {
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
        self.char = new_char;

        log::info!("successfully loaded char");
        Ok(())
    }

    fn save_char(&self) -> Result<()> {
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

        let char_serialized = serde_json::to_vec_pretty(&self.char)
            .context("failed to convert character to JSON format")?;
        async_std::task::block_on(save_file(&char_serialized))
            .context("failed to save char to file")?;

        log::info!("successfully saved char");
        Ok(())
    }
}
