use crate::{
    app::{character::Character, widgets},
    simulator::Simulator,
};

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Group {
    chars: Vec<Character>,
}

impl Group {
    #[must_use]
    pub fn draw(&mut self, simulator: &mut Simulator, ui: &mut egui::Ui) -> Option<GroupAction> {
        // user can only click on one thing each frame, so overwriting the
        // previous action should be ok
        let mut action = None;

        ui.group(|ui| {
            ui.vertical(|ui| {
                if let Some(button_action) = self.draw_menu_buttons(ui) {
                    action = Some(button_action);
                }

                ui.add_space(15.0);

                if let Some(char_action) = self.draw_chars(simulator, ui) {
                    action = Some(char_action);
                }
            });
        });

        action
    }

    fn draw_chars(&mut self, simulator: &mut Simulator, ui: &mut egui::Ui) -> Option<GroupAction> {
        let mut action = None;

        for (index, char) in self.chars.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                let char_button = char.draw_as_button(simulator, ui);
                if char_button.clicked() {
                    action = Some(GroupAction::Select(CharIndex::from_usize(index)));
                }

                let rm_size = 15.0;
                let rm_help = "Diesen Char von der Gruppe entfernen";
                let rm_text = "❌";
                let rm_button = widgets::create_menu_button(rm_text, rm_help, rm_size, ui);
                if rm_button.clicked() {
                    action = Some(GroupAction::Delete(CharIndex::from_usize(index)));
                }
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

    pub fn delete_char(&mut self, index: CharIndex) {
        self.chars.remove(index.into_usize());
    }

    pub fn clear(&mut self) {
        self.chars.clear();
    }

    pub fn get_char_mut(&mut self, index: CharIndex) -> Option<&mut Character> {
        self.chars.get_mut(index.into_usize())
    }

    // TODO delete once the simulator can handle groups
    pub fn first(&self) -> Option<&Character> {
        self.chars.first()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharIndex(usize);

impl CharIndex {
    pub fn decrement(&mut self) {
        assert_ne!(self.0, 0, "can't decrement 0");
        self.0 -= 1;
    }

    fn from_usize(index: usize) -> Self {
        Self(index)
    }

    fn into_usize(self) -> usize {
        self.0
    }
}
