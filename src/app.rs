use crate::character::Character;
use crate::simulator::Simulator;
// use crate::simulator::Simulator;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct App {
    char: Character,
    opponent: Character,

    #[serde(skip)]
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

        self.simulator
            .update_characters(self.char.clone(), self.opponent.clone());

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

            egui::containers::Resize::default()
                .auto_sized()
                .show(ui, |ui| {
                    egui::Grid::new("CharCols")
                        .num_columns(3)
                        .spacing([10.0, 4.0])
                        .striped(false)
                        .show(ui, |ui| {
                            self.char.draw(&self.simulator, ui);
                            self.opponent.draw_as_opponent(ui);
                            ui.end_row();
                        });
                });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.progress_bar(ui);
            egui::warn_if_debug_build(ui);
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

    fn menu_bar(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let is_web = cfg!(target_arch = "wasm32"); // no File->Quit on web pages

        ui.menu_button("File", |ui| {
            if !is_web && ui.button("Quit").clicked() {
                log::info!("quit button clicked, exiting...");
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
        ui.add_space(10.0);

        ui.menu_button("Appearance", |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        ui.add_space(10.0);
    }
}
