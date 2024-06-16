pub mod menu_bar;
pub mod widget;

use std::{cell::RefCell, rc::Rc};

use widget::menu_bar::MenuBarUiState;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FocusOn {
    MainPanel,
    MenuTab,
    MenuTabItem(u16),
}

#[derive(Clone, PartialEq, Eq)]
pub struct UiState {
    pub focus_on: FocusOn,
    pub menu_bar_state: Rc<RefCell<MenuBarUiState>>,
}
