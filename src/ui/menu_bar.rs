use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::theme::THEME;

use super::{
    widget::menu_bar::{
        MenuBar, MenuBarStyle, MenuBarUiState, MenuTab, MenuTabItem, MenuTabItemUiState,
        MenuTabUiState,
    },
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

pub const TOOL_TAB_ITEMS: [(&str, Option<&str>); 2] = [
    ("Refresh OpenPGP Cert", Some("R")),
    ("Restart Backend Process", None),
];

pub const SETTING_TAB_ITEMS: [(&str, Option<&str>); 2] =
    [("General", Some("G")), ("Server", Some("S"))];

pub const HELP_TAB_ITEMS: [(&str, Option<&str>); 2] =
    [("Check Update", Some("C")), ("About", Some("A"))];

pub fn init_menu_bar_state() -> MenuBarUiState {
    let mut menu_tab_states = Vec::new();

    // File Tab
    {
        let mut sub_item_states = Vec::new();
        // New KeyPair
        sub_item_states.push(MenuTabItemUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        });

        // Import
        sub_item_states.push(MenuTabItemUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        });

        // Search On Server
        sub_item_states.push(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        });

        // Decrypt/Verify
        sub_item_states.push(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        });

        // Encrypt/Sign
        sub_item_states.push(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        });

        // Quit
        sub_item_states.push(MenuTabItemUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        });

        menu_tab_states.push(MenuTabUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: Some(sub_item_states),
        });
    }

    // Tool Tab
    {
        let mut sub_item_states = Vec::new();

        // Refresh OpenPGP Cert
        sub_item_states.push(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        });

        // Restart Backend Process
        sub_item_states.push(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        });

        menu_tab_states.push(MenuTabUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: Some(sub_item_states),
        });
    }

    // Setting Tab
    {
        let mut sub_item_states = Vec::new();
        // General
        sub_item_states.push(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        });

        // Server
        sub_item_states.push(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        });

        menu_tab_states.push(MenuTabUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: Some(sub_item_states),
        });
    }

    // Help Tab
    {
        let mut sub_item_states = Vec::new();
        // Check Update
        sub_item_states.push(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        });

        // About
        sub_item_states.push(MenuTabItemUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        });

        menu_tab_states.push(MenuTabUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: Some(sub_item_states),
        });
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

    let mut menu_items: Vec<MenuTab> = Vec::new();

    // File Tab
    {
        let mut tab_items: Vec<MenuTabItem> = Vec::new();

        for (label, hotkey) in FILE_TAB_ITEMS.iter() {
            tab_items.push(MenuTabItem::new(
                label.to_string(),
                *hotkey,
                None,
                &ui_state.menu_bar_state.tab_state[0]
                    .sub_item_state
                    .as_ref()
                    .unwrap()[tab_items.len()],
                menu_bar_style,
            ));
        }

        menu_items.push(MenuTab::new(
            MENU_TABS[0].0.to_string(),
            MENU_TABS[0].1,
            Some(tab_items),
            &ui_state.menu_bar_state.tab_state[0],
            menu_bar_style,
        ));
    }

    // Tool Tab
    {
        let mut tab_items: Vec<MenuTabItem> = Vec::new();

        for (label, hotkey) in TOOL_TAB_ITEMS.iter() {
            tab_items.push(MenuTabItem::new(
                label.to_string(),
                *hotkey,
                None,
                &ui_state.menu_bar_state.tab_state[1]
                    .sub_item_state
                    .as_ref()
                    .unwrap()[tab_items.len()],
                menu_bar_style,
            ));
        }

        menu_items.push(MenuTab::new(
            MENU_TABS[1].0.to_string(),
            MENU_TABS[1].1,
            Some(tab_items),
            &ui_state.menu_bar_state.tab_state[1],
            menu_bar_style,
        ));
    }

    // Setting Tab
    {
        let mut tab_items: Vec<MenuTabItem> = Vec::new();

        for (label, hotkey) in SETTING_TAB_ITEMS.iter() {
            tab_items.push(MenuTabItem::new(
                label.to_string(),
                *hotkey,
                None,
                &ui_state.menu_bar_state.tab_state[2]
                    .sub_item_state
                    .as_ref()
                    .unwrap()[tab_items.len()],
                menu_bar_style,
            ));
        }

        menu_items.push(MenuTab::new(
            MENU_TABS[2].0.to_string(),
            MENU_TABS[2].1,
            Some(tab_items),
            &ui_state.menu_bar_state.tab_state[2],
            menu_bar_style,
        ));
    }

    // Help Tab
    {
        let mut tab_items: Vec<MenuTabItem> = Vec::new();

        for (label, hotkey) in HELP_TAB_ITEMS.iter() {
            tab_items.push(MenuTabItem::new(
                label.to_string(),
                *hotkey,
                None,
                &ui_state.menu_bar_state.tab_state[3]
                    .sub_item_state
                    .as_ref()
                    .unwrap()[tab_items.len()],
                menu_bar_style,
            ));
        }

        menu_items.push(MenuTab::new(
            MENU_TABS[3].0.to_string(),
            MENU_TABS[3].1,
            Some(tab_items),
            &ui_state.menu_bar_state.tab_state[3],
            menu_bar_style,
        ));
    }

    MenuBar::new(
        Some("GPG4Terminal".to_string()),
        menu_items,
        &ui_state.menu_bar_state,
        menu_bar_style,
    )
    .render(area, buf);
}
