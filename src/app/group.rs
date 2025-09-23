use crate::{
    app::{character::Character, dnd, widgets},
    simulator::Simulator,
};

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Group {
    chars: Vec<Character>,
}

impl Group {
    pub fn into_vec(self) -> Vec<Character> {
        self.chars
    }

    #[must_use]
    pub fn draw(
        &mut self,
        simulator: &mut Simulator,
        drag_ctx: dnd::DragContext,
        ui: &mut egui::Ui,
    ) -> Option<GroupAction> {
        // user can only click on one thing each frame, so overwriting the
        // previous action should be ok
        let mut action = None;

        ui.group(|ui| {
            ui.vertical(|ui| {
                if let Some(button_action) = self.draw_menu_buttons(ui) {
                    action = Some(button_action);
                }

                ui.add_space(15.0);

                if let Some(char_action) = self.draw_chars(simulator, drag_ctx, ui) {
                    action = Some(char_action);
                }
            });
        });

        action
    }

    fn draw_chars(
        &mut self,
        simulator: &mut Simulator,
        drag_ctx: dnd::DragContext,
        ui: &mut egui::Ui,
    ) -> Option<GroupAction> {
        let mut action = None;

        if self.chars.is_empty() {
            drag_ctx.create_empty_drop_area(ui);
        }

        for (index, char) in self.chars.iter_mut().enumerate() {
            let index = CharIndex::from_usize(index);
            drag_ctx.create_drop_area(index, ui, |ui| {
                ui.horizontal(|ui| {
                    let drag_size = 15.0;
                    drag_ctx.draw_drag_button(drag_size, index, ui);

                    let char_button = char.draw_as_button(simulator, ui);
                    if char_button.clicked() {
                        action = Some(GroupAction::Select(index));
                    }

                    let rm_size = 15.0;
                    let rm_help = "Diesen Char von der Gruppe entfernen";
                    let rm_text = "❌";
                    let rm_button = widgets::create_menu_button(rm_text, rm_help, rm_size, ui);
                    if rm_button.clicked() {
                        action = Some(GroupAction::Delete(index));
                    }
                });
            });
        }

        action
    }

    fn draw_menu_buttons(&mut self, ui: &mut egui::Ui) -> Option<GroupAction> {
        let size = 40.0;
        let mut action = None;

        ui.horizontal(|ui| {
            let new_char_button = widgets::create_menu_button("➕", "Neuer Char", size, ui);
            if new_char_button.clicked() {
                action = Some(GroupAction::New);
            }

            let load_char_button = widgets::create_menu_button("🗁", "Char laden", size, ui);
            if load_char_button.clicked() {
                action = Some(GroupAction::Load);
            }

            let clear_button = widgets::create_menu_button("🗑", "Alle Chars löschen", size, ui);
            if clear_button.clicked() {
                action = Some(GroupAction::Clear);
            }
        });

        action
    }

    pub fn add_char(&mut self, character: Character) {
        self.chars.push(character);
    }

    pub fn insert_char(&mut self, index: CharIndex, character: Character) {
        self.chars.insert(index.into_usize(), character);
    }

    pub fn take_char(&mut self, index: CharIndex) -> Character {
        self.chars.remove(index.into_usize())
    }

    pub fn delete_char(&mut self, index: CharIndex) {
        self.chars.remove(index.into_usize());
    }

    pub fn clear(&mut self) {
        self.chars.clear();
    }

    pub fn get_char_mut(&mut self, index: CharIndex) -> Option<&mut Character> {
        self.chars.get_mut(index.into_usize())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GroupAction {
    New,
    Load,
    Clear,
    Select(CharIndex),
    Delete(CharIndex),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharIndex(usize);

impl CharIndex {
    pub fn increment(self) -> Self {
        Self(self.0 + 1)
    }

    pub fn decrement(self) -> Self {
        assert_ne!(self.0, 0, "can't decrement 0");
        Self(self.0 - 1)
    }

    fn from_usize(index: usize) -> Self {
        Self(index)
    }

    pub fn into_usize(self) -> usize {
        self.0
    }
}
