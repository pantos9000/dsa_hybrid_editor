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
