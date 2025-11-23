pub mod character;
pub mod gradient;
pub mod group;

mod dnd;
mod io;
mod widgets;

use egui::{Align, Layout};

use io::{IoResponse, IoThread};

use crate::{
    app::{
        character::Character,
        dnd::DragOperation,
        group::{CharIndex, Group, GroupAction},
        io::IoRequest,
    },
    simulator::Simulator,
};

pub const EDITOR_WIDTH: f32 = 650.0;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct App {
    chars_left: Group,
    chars_right: Group,

    #[serde(skip)]
    selection: Option<CharSelection>,
    #[serde(skip)]
    simulator: Simulator,
    #[serde(skip)]
    io: IoThread,
    #[serde(skip)]
    dnd: dnd::DragHandler,
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
        Self::default()
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

        for io_response in self.io.poll_iter() {
            match io_response {
                IoResponse::CharLoaded(group_id, new_char) => {
                    let group = match group_id {
                        GroupId::Left => &mut self.chars_left,
                        GroupId::Right => &mut self.chars_right,
                    };
                    group.add_char(new_char);
                    log::info!("character successfully loaded");
                }
            }
        }

        if let Some(dnd) = self.dnd.operation() {
            self.handle_dnd(dnd);
        }

        self.simulator.update(
            self.chars_left.clone().into_vec(),
            self.chars_right.clone().into_vec(),
        );

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("DSA Hybrid Char Editor");
                });
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    Self::quit_button(ui, ctx);
                    self.help_button(ui);
                });
            });
            ui.add_space(2.0);
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(8.0);
                self.progress_bar(ui);
                egui::widgets::global_theme_preference_buttons(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .show(ctx, |ui| {
                self.draw_group(GroupId::Left, ui);
            });

        egui::SidePanel::right("right_panel")
            .resizable(false)
            .show(ctx, |ui| {
                self.draw_group(GroupId::Right, ui);
            });

        // The central panel the region left after adding other panels - has to come last
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                self.simulator.report().draw(ui);
                ui.add_space(8.0);
                self.draw_char_editor(ui);
            });
        });
    }
}

impl App {
    fn handle_dnd(&mut self, dnd: DragOperation) {
        log::debug!("drag and drop detected: {dnd:?}");

        let DragOperation { from, to } = dnd;

        let same_group = from.group == to.group;
        let same_index = Some(from.index) == to.index();

        if same_group && same_index {
            return;
        }

        let from_group = match from.group {
            GroupId::Left => &mut self.chars_left,
            GroupId::Right => &mut self.chars_right,
        };
        let moved_char = from_group.take_char(from.index);
        log::debug!("moving char: {}", moved_char.name.as_str());

        let to_group = match to.group {
            GroupId::Left => &mut self.chars_left,
            GroupId::Right => &mut self.chars_right,
        };

        let to_index = to.index().unwrap_or_default();
        let moved_down = same_group && to_index > from.index;
        let to_index = if moved_down {
            // we removed a char, so index has to be adjusted
            to_index.decrement()
        } else {
            to_index
        };

        to_group.insert_char(to_index, moved_char);

        // adjust selection
        if let Some(selection) = self.selection.as_mut() {
            Self::adjust_selection_after_dnd(selection, from.group, to.group, from.index, to_index);
        }
    }

    fn adjust_selection_after_dnd(
        selection: &mut CharSelection,
        from_group: GroupId,
        to_group: GroupId,
        from_index: CharIndex,
        to_index: CharIndex,
    ) {
        let move_selection =
            selection.group_id() == from_group && selection.char_index() == from_index;
        if move_selection {
            *selection = CharSelection::new(to_group, to_index);
            return;
        }

        let decrement = selection.group_id() == from_group && selection.char_index() > from_index;
        let increment = selection.group_id() == to_group && selection.char_index() >= to_index;
        match (increment, decrement) {
            (true, true) => (),
            (true, false) => selection.increment_index(),
            (false, true) => selection.decrement_index(),
            (false, false) => (),
        }
    }

    fn draw_char_editor(&mut self, ui: &mut egui::Ui) {
        let Some(selection) = self.selection else {
            Character::draw_help(ui);
            return;
        };

        // unwrap is fine, selection should always be valid and exist
        let char = match selection {
            CharSelection::Left(i) => self.chars_left.get_char_mut(i).unwrap(),
            CharSelection::Right(i) => self.chars_right.get_char_mut(i).unwrap(),
        };

        char.draw_editor(selection, &mut self.simulator, &self.io, ui);
    }

    fn draw_group(&mut self, group_id: GroupId, ui: &mut egui::Ui) {
        let group = match group_id {
            GroupId::Left => &mut self.chars_left,
            GroupId::Right => &mut self.chars_right,
        };
        let draw_ctx = self.dnd.create_context(group_id);

        ui.add_space(4.0);
        let Some(action) = group.draw(draw_ctx, ui) else {
            return;
        };

        match action {
            GroupAction::New => group.add_char(Character::default()),
            GroupAction::Load => self.io.request(IoRequest::Load(group_id)),
            GroupAction::Clear => group.clear(),
            GroupAction::Select(i) => self.selection = Some(CharSelection::new(group_id, i)),
            GroupAction::Copy(i) => group.copy_char(i),
            GroupAction::Delete(i) => group.delete_char(i),
        }

        self.adjust_selection_index(group_id, action);
    }

    fn adjust_selection_index(&mut self, affected_group: GroupId, action: GroupAction) {
        // TODO: bei drag & drop!
        let selected = match (&mut self.selection, affected_group) {
            (Some(CharSelection::Left(i)), GroupId::Left) => i,
            (Some(CharSelection::Right(i)), GroupId::Right) => i,
            _ => return,
        };

        match action {
            GroupAction::Clear => self.selection = None,
            GroupAction::Delete(i) if i == *selected => self.selection = None,
            GroupAction::Delete(i) if i < *selected => *selected = selected.decrement(),
            _ => (),
        }
    }

    fn progress_bar(&mut self, ui: &mut egui::Ui) {
        let progress = self.simulator.progress();
        if progress >= 100 {
            return;
        }

        let progress: f32 = f32::from(progress) / 100_f32;
        let progress_bar = egui::widgets::ProgressBar::new(progress).show_percentage();
        ui.add(progress_bar);
    }

    fn help_button(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
            let text = egui::RichText::new("❓").size(24.0);
            let button = egui::Button::new(text).corner_radius(5.0);
            let response = ui.add_sized([32.0, 32.0], button).on_hover_ui(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Hilfe anzeigen");
                });
            });
            if response.clicked() {
                log::info!("help button clicked");
                self.selection = None;
            }
        });
    }

    fn quit_button(ui: &mut egui::Ui, ctx: &egui::Context) {
        let is_web = cfg!(target_arch = "wasm32"); // no File->Quit on web pages
        if is_web {
            return;
        }

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
    }
}

/// Contains information from which team a char is selected, and which index in the team
#[derive(Debug, Clone, Copy)]
pub enum CharSelection {
    Left(CharIndex),
    Right(CharIndex),
}

impl CharSelection {
    fn new(id: GroupId, index: CharIndex) -> Self {
        match id {
            GroupId::Left => Self::Left(index),
            GroupId::Right => Self::Right(index),
        }
    }

    fn group_id(self) -> GroupId {
        match self {
            Self::Left(_) => GroupId::Left,
            Self::Right(_) => GroupId::Right,
        }
    }

    fn char_index(self) -> CharIndex {
        match self {
            Self::Left(char_index) => char_index,
            Self::Right(char_index) => char_index,
        }
    }

    fn increment_index(&mut self) {
        *self = match self {
            Self::Left(char_index) => Self::Left(char_index.increment()),
            Self::Right(char_index) => Self::Right(char_index.increment()),
        };
    }

    fn decrement_index(&mut self) {
        *self = match self {
            Self::Left(char_index) => Self::Left(char_index.decrement()),
            Self::Right(char_index) => Self::Right(char_index.decrement()),
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GroupId {
    Left,
    Right,
}
