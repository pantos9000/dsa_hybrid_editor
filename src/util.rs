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
