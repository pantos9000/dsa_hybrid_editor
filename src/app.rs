use crate::character::Character;
use crate::simulator::Simulator;

pub trait Drawable {
    fn draw_ui(&mut self, ui: &mut egui::Ui);
    fn draw_gradients(&self, ui: &mut egui::Ui, simulator: &Simulator);
}

pub fn create_grid(name: &'static str) -> egui::Grid {
    egui::Grid::new(name)
        .num_columns(2)
        .min_col_width(120.0)
        .spacing([0.0, 4.0])
        .striped(true)
}

pub fn create_frame(ui: &egui::Ui) -> egui::Frame {
    egui::Frame::default()
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .rounding(ui.visuals().widgets.noninteractive.rounding)
        .inner_margin(10.0)
        .outer_margin(5.0)
        .fill(egui::Color32::TRANSPARENT)
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
                menu_bar(ui, ctx);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("DSA Hybrid Char Editor");
            ui.separator();

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    self.char.draw_ui(ui);
                });
                ui.vertical(|ui| {
                    self.char.draw_gradients(ui, &self.simulator);
                });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
                ui.separator();
            });
        });
    }
}

fn menu_bar(ui: &mut egui::Ui, ctx: &egui::Context) {
    // NOTE: no File->Quit on web pages!
    let is_web = cfg!(target_arch = "wasm32");
    if !is_web {
        ui.menu_button("File", |ui| {
            if ui.button("Quit").clicked() {
                log::info!("quit button clicked, exiting...");
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
        ui.add_space(16.0);
    }

    egui::widgets::global_dark_light_mode_buttons(ui);
}
