use egui::{Align, Layout};

use crate::character::Character;
use crate::io::IoThread;
use crate::simulator::Simulator;
use crate::util;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct App {
    char: Character,
    opponent: Character,

    #[serde(skip)]
    simulator: Simulator,

    #[serde(skip)]
    io: IoThread,
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

        for response in self.io.poll_iter() {
            match response {
                crate::io::IoResponse::CharLoaded(character) => {
                    self.char = character;
                    log::info!("character successfully loaded")
                }
                crate::io::IoResponse::OpponentLoaded(character) => {
                    self.opponent = character;
                    log::info!("opponent successfully loaded");
                }
            }
        }

        self.simulator
            .update_characters(self.char.clone(), self.opponent.clone());

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("DSA Hybrid Char Editor");
                Self::quit_button(ui, ctx);
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.progress_bar(ui);
            egui::widgets::global_theme_preference_buttons(ui);
            egui::warn_if_debug_build(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding other panels - has to come last

            self.menu_buttons(ui);
            ui.separator();
            egui::containers::ScrollArea::both().show(ui, |ui| {
                egui::Grid::new("CharCols")
                    .num_columns(3)
                    .spacing([10.0, 4.0])
                    .striped(false)
                    .show(ui, |ui| {
                        self.char.draw(&self.simulator, &self.io, ui);
                        self.opponent.draw_as_opponent(&self.io, ui);
                        ui.end_row();
                    });
            });
        });
    }
}

impl App {
    fn progress_bar(&mut self, ui: &mut egui::Ui) {
        let progress = self.simulator.progress();
        if progress >= 100 {
            return;
        }

        let progress: f32 = f32::from(progress) / 100_f32;
        let progress_bar = egui::widgets::ProgressBar::new(progress).show_percentage();
        ui.add(progress_bar);
    }

    fn menu_buttons(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            let copy_char_button = util::create_menu_button("➡", "Copy char to opponent", 40.0, ui);
            if copy_char_button.clicked() {
                self.opponent = self.char.clone();
            }

            let switch_button = util::create_menu_button("↔", "Switch chars", 40.0, ui);
            if switch_button.clicked() {
                std::mem::swap(&mut self.char, &mut self.opponent);
            }

            let copy_opponent_button =
                util::create_menu_button("⬅", "Copy opponent to char", 40.0, ui);
            if copy_opponent_button.clicked() {
                self.char = self.opponent.clone();
            }
        });
    }

    fn quit_button(ui: &mut egui::Ui, ctx: &egui::Context) {
        let is_web = cfg!(target_arch = "wasm32"); // no File->Quit on web pages
        if is_web {
            return;
        }

        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
            let text = egui::RichText::new("❌").size(24.0);
            let button = egui::Button::new(text).corner_radius(5.0);
            let response = ui.add_sized([32.0, 32.0], button).on_hover_ui(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Quit");
                });
            });
            if response.clicked() {
                log::info!("quit button clicked, exiting...");
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
