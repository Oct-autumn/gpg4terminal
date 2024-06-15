use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::theme::THEME;

use super::{
    widget::menu_bar::{MenuBar, MenuBarStyle, MenuBarUiState, MenuTab, MenuTabUiState},
    UiState,
};

pub const MENU_TABS: [(&str, Option<&str>); 4] = [
    ("File", Some("F")),
    ("Tool", Some("T")),
    ("Setting", Some("S")),
    ("Help", Some("H")),
];

pub const FILE_TAB_ITEMS: [(&str, Option<&str>); 6] = [
    ("New KeyPair", Some("N")),
    ("Import", Some("I")),
    ("Search On Server", Some("S")),
    ("Decrypt/Verify", None),
    ("Encrypt/Sign", None),
    ("Quit", Some("Q")),
];

pub const TOOL_TAB_ITEMS: [(&str, Option<&str>); 3] = [
    ("Refresh", None),
    ("Decrypt/Verify", None),
    ("Generate KeyPair", None),
];

pub fn init_menu_bar_state() -> MenuBarUiState {
    let mut menu_tab_states = Vec::new();
    // todo
    for i in 0..5 {
        menu_tab_states.push(MenuTabUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })
    }

    MenuBarUiState {
        focus_on: 0,
        tab_state: menu_tab_states,
    }
}

pub fn render_menu_bar(ui_state: &UiState, area: Rect, buf: &mut Buffer) {
    let menu_bar_style = &MenuBarStyle::new(
        THEME.menu_bar.title_style,
        THEME.menu_bar.default_style,
        THEME.menu_bar.disabled_style,
        THEME.menu_bar.focused_style,
        THEME.menu_bar.selected_style,
    );

    let menu_items: Vec<MenuTab> = MENU_TABS
        .iter()
        .enumerate()
        .map(|(i, (label, hotkey))| {
            let state = ui_state.menu_bar_state.tab_state.get(i);
            assert!(state.is_some(), "Tab state not found!");
            MenuTab::new(
                label.to_string(),
                *hotkey,
                None,
                state.unwrap(),
                menu_bar_style,
            )
        })
        .collect();

    MenuBar::new(
        Some("GPG4Terminal".to_string()),
        menu_items,
        &ui_state.menu_bar_state,
        menu_bar_style,
    )
    .render(area, buf);
}
