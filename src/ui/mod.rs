pub mod menu_bar;
pub mod widget;

use widget::menu_bar::MenuBarUiState;

#[derive(Clone, PartialEq, Eq)]
pub enum FocusOn {
    MainPanel,
    MenuTab,
    MenuTabItem,
}

#[derive(Clone, PartialEq, Eq)]
pub struct UiState {
    pub focus_on: FocusOn,
    pub menu_bar_state: MenuBarUiState,
}
