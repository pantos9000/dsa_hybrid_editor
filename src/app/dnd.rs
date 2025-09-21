use std::{cell::RefCell, rc::Rc};

use egui::Frame;

use crate::app::{self, group::CharIndex};

#[derive(Debug, Default, Clone)]
pub struct DragHandler {
    inner: Rc<RefCell<DragHandlerInner>>,
}

impl DragHandler {
    pub fn operation(&self) -> Option<DragOperation> {
        let mut inner = self.inner.borrow_mut();
        inner.try_reset_ongoing_drag();
        inner.take_operation()
    }

    pub fn create_context(&self, group: app::GroupId) -> DragContext {
        DragContext::new(&self.inner, group)
    }
}

#[derive(Debug, Default, Clone)]
struct DragHandlerInner {
    operation: Option<DragOperation>,
    dragged: Option<DragLocation>,
    hovered: Option<DropLocation>,
    drag_ongoing: bool,
}

impl DragHandlerInner {
    fn take_operation(&mut self) -> Option<DragOperation> {
        self.operation.take()
    }

    fn try_reset_ongoing_drag(&mut self) {
        if self.drag_ongoing {
            self.drag_ongoing = false;
            return;
        }
        self.dragged = None;
        self.hovered = None;
    }

    fn set_operation(&mut self) {
        self.drag_ongoing = false;
        let Some(from) = self.dragged.take() else {
            return;
        };
        let Some(to) = self.hovered.take() else {
            return;
        };
        self.operation = Some(DragOperation { from, to });
    }

    fn set_dragged(&mut self, dragged: DragLocation) {
        self.drag_ongoing = true;
        self.dragged = Some(dragged);
    }

    fn set_hovered(&mut self, hovered: DropLocation) {
        self.drag_ongoing = true;
        self.hovered = Some(hovered);
    }
}

#[derive(Debug, Clone)]
pub struct DragContext {
    handler: Rc<RefCell<DragHandlerInner>>,
    group: app::GroupId,
}

impl DragContext {
    fn new(handler: &Rc<RefCell<DragHandlerInner>>, group: app::GroupId) -> Self {
        Self {
            handler: Rc::clone(handler),
            group,
        }
    }

    pub fn create_empty_drop_area(&self, ui: &mut egui::Ui) {
        fn no_content(ui: &mut egui::Ui) {
            // When changing sizes, also adjust this
            // it should roughly be the size of the frame of a char
            ui.set_min_width(145.0);
            ui.set_min_height(24.0);
        }
        self.create_drop_area_inner(None, ui, no_content);
    }

    pub fn create_drop_area(
        &self,
        index: CharIndex,
        ui: &mut egui::Ui,
        content: impl FnOnce(&mut egui::Ui),
    ) {
        self.create_drop_area_inner(Some(index), ui, content);
    }

    fn create_drop_area_inner(
        &self,
        index: Option<CharIndex>,
        ui: &mut egui::Ui,
        content: impl FnOnce(&mut egui::Ui),
    ) {
        // workaround, see https://github.com/emilk/egui/issues/5695
        ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
        let frame = Frame::default().inner_margin(4.0);

        let (inner_response, payload) = ui.dnd_drop_zone::<(), ()>(frame, content);
        if payload.is_some() {
            self.handler.borrow_mut().set_operation();
        }
        let response = inner_response.response;
        if let Some(hover_state) = DropLocation::new(&response, self.group, index, ui) {
            self.handler.borrow_mut().set_hovered(hover_state);
        }
    }

    pub fn drag_symbol(&self, location: DragLocation) -> &'static str {
        let handler = self.handler.borrow();

        if handler.dragged == Some(location) {
            return "O";
        }

        if let Some(hovered) = handler.hovered
            && hovered.group == location.group
        {
            match hovered.index {
                Some(DropIndex::Above(i)) if i == location.index => return "↗",
                Some(DropIndex::Below(i)) if i == location.index => return "↘",
                _ => (),
            }
        }

        "☰"
    }

    pub fn draw_drag_button(&self, size: f32, index: CharIndex, ui: &mut egui::Ui) {
        let location = DragLocation {
            group: self.group,
            index,
        };

        let text = self.drag_symbol(location);
        let help = "Drag&Drop Char";
        let button = app::widgets::create_menu_button(text, help, size, ui);
        let id = egui::Id::new(("drag_char", location));
        let button = ui.interact(button.rect, id, egui::Sense::drag());

        // if the button is dragged, store the payload
        button.dnd_set_drag_payload(());
        if button.dragged() {
            self.handler.borrow_mut().set_dragged(location);
        }
    }
}

#[derive(Debug, Clone)]
pub struct DragOperation {
    pub from: DragLocation,
    pub to: DropLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DragLocation {
    pub group: app::GroupId,
    pub index: CharIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropLocation {
    pub group: app::GroupId,
    index: Option<DropIndex>,
}

impl DropLocation {
    fn new(
        response: &egui::Response,
        group: app::GroupId,
        index: Option<CharIndex>,
        ui: &egui::Ui,
    ) -> Option<Self> {
        if response.dragged() {
            return None;
        }
        response.dnd_hover_payload::<()>()?;
        let mouse_pos = RelMousePos::new(response, ui)?;
        let index = index.map(|i| DropIndex::new(i, mouse_pos));
        let location = Self { group, index };
        Some(location)
    }

    pub fn index(self) -> Option<CharIndex> {
        let index = match self.index? {
            DropIndex::Above(i) => i,
            DropIndex::Below(i) => i.increment(),
        };
        Some(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DropIndex {
    Above(CharIndex),
    Below(CharIndex),
}

impl DropIndex {
    fn new(index: CharIndex, mouse_pos: RelMousePos) -> Self {
        match mouse_pos {
            RelMousePos::Above => Self::Above(index),
            RelMousePos::Below => Self::Below(index),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RelMousePos {
    Above,
    Below,
}

impl RelMousePos {
    fn new(response: &egui::Response, ui: &egui::Ui) -> Option<Self> {
        let mouse_pos = ui.input(|i| i.pointer.interact_pos())?.y;
        let item_pos = response.rect.center().y;
        let pos = if mouse_pos < item_pos {
            Self::Above
        } else {
            Self::Below
        };
        Some(pos)
    }
}
