use std::{cell::RefCell, rc::Rc};

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::{event_handler::key_board_handler, theme::THEME};

use super::{
    focus,
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

pub fn init_menu_bar_state() -> Rc<RefCell<MenuBarUiState>> {
    let mut menu_tab_states = vec![];

    // File Tab
    {
        let mut sub_item_states = vec![];
        // New KeyPair
        sub_item_states.push(Rc::new(RefCell::new(MenuTabItemUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })));

        // Import
        sub_item_states.push(Rc::new(RefCell::new(MenuTabItemUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })));

        // Search On Server
        sub_item_states.push(Rc::new(RefCell::new(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })));

        // Decrypt/Verify
        sub_item_states.push(Rc::new(RefCell::new(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })));

        // Encrypt/Sign
        sub_item_states.push(Rc::new(RefCell::new(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })));

        // Quit
        sub_item_states.push(Rc::new(RefCell::new(MenuTabItemUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })));

        menu_tab_states.push(Rc::new(RefCell::new(MenuTabUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: Some(sub_item_states),
        })));
    }

    // Tool Tab
    {
        let mut sub_item_states = Vec::new();

        // Refresh OpenPGP Cert
        sub_item_states.push(Rc::new(RefCell::new(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })));

        // Restart Backend Process
        sub_item_states.push(Rc::new(RefCell::new(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })));

        menu_tab_states.push(Rc::new(RefCell::new(MenuTabUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: Some(sub_item_states),
        })));
    }

    // Setting Tab
    {
        let mut sub_item_states = Vec::new();
        // General
        sub_item_states.push(Rc::new(RefCell::new(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })));

        // Server
        sub_item_states.push(Rc::new(RefCell::new(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })));

        menu_tab_states.push(Rc::new(RefCell::new(MenuTabUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: Some(sub_item_states),
        })));
    }

    // Help Tab
    {
        let mut sub_item_states = Vec::new();
        // Check Update
        sub_item_states.push(Rc::new(RefCell::new(MenuTabItemUiState {
            is_disabled: true,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })));

        // About
        sub_item_states.push(Rc::new(RefCell::new(MenuTabItemUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: None,
        })));

        menu_tab_states.push(Rc::new(RefCell::new(MenuTabUiState {
            is_disabled: false,
            is_selected: false,
            is_focused: false,
            focus_on: 0,
            sub_item_state: Some(sub_item_states),
        })));
    }

    Rc::new(RefCell::new(MenuBarUiState {
        focus_on: 0,
        tab_state: menu_tab_states,
    }))
}

pub fn render_menu_bar(ui_state: &UiState, area: Rect, buf: &mut Buffer) {
    let menu_bar_state = ui_state.menu_bar_state.borrow();
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
                menu_bar_state.tab_state[0]
                    .borrow()
                    .sub_item_state
                    .as_ref()
                    .unwrap()[tab_items.len()]
                .clone(),
                menu_bar_style,
            ));
        }

        menu_items.push(MenuTab::new(
            MENU_TABS[0].0.to_string(),
            MENU_TABS[0].1,
            Some(tab_items),
            menu_bar_state.tab_state[0].clone(),
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
                menu_bar_state.tab_state[1]
                    .borrow()
                    .sub_item_state
                    .as_ref()
                    .unwrap()[tab_items.len()]
                .clone(),
                menu_bar_style,
            ));
        }

        menu_items.push(MenuTab::new(
            MENU_TABS[1].0.to_string(),
            MENU_TABS[1].1,
            Some(tab_items),
            menu_bar_state.tab_state[1].clone(),
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
                menu_bar_state.tab_state[2]
                    .borrow()
                    .sub_item_state
                    .as_ref()
                    .unwrap()[tab_items.len()]
                .clone(),
                menu_bar_style,
            ));
        }

        menu_items.push(MenuTab::new(
            MENU_TABS[2].0.to_string(),
            MENU_TABS[2].1,
            Some(tab_items),
            menu_bar_state.tab_state[2].clone(),
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
                menu_bar_state.tab_state[3]
                    .borrow()
                    .sub_item_state
                    .as_ref()
                    .unwrap()[tab_items.len()]
                .clone(),
                menu_bar_style,
            ));
        }

        menu_items.push(MenuTab::new(
            MENU_TABS[3].0.to_string(),
            MENU_TABS[3].1,
            Some(tab_items),
            menu_bar_state.tab_state[3].clone(),
            menu_bar_style,
        ));
    }

    MenuBar::new(
        Some("GPG4Terminal".to_string()),
        menu_items,
        ui_state.menu_bar_state.clone(),
        menu_bar_style,
    )
    .render(area, buf);
}

pub fn handle_event(
    focus_on: &mut focus::FocusOn,
    menu_bar_state: Rc<RefCell<MenuBarUiState>>,
    event: key_board_handler::EventResult,
) {
    let mut menu_bar_state = menu_bar_state.borrow_mut();
    match event {
        key_board_handler::EventResult::EnterMenuBar => {
            *focus_on = focus::FocusOn::MenuBar(focus::Menu::Tab);
            // Reset focus on the first tab
            menu_bar_state.focus_on = 0;
            menu_bar_state.tab_state[0]
                .try_borrow_mut()
                .unwrap()
                .is_focused = true;
        }
        key_board_handler::EventResult::QuitMenu => {
            match focus_on {
                focus::FocusOn::MenuBar(focus::Menu::Tab) => {
                    let current_tab = menu_bar_state.focus_on;
                    menu_bar_state.tab_state[current_tab]
                        .try_borrow_mut()
                        .unwrap()
                        .is_focused = false;
                }
                focus::FocusOn::MenuBar(focus::Menu::TabItem(_)) => {
                    let mut stack = Vec::new();

                    // push in the first focused tab item
                    {
                        let current_tab_id = menu_bar_state.focus_on;
                        let current_tab = menu_bar_state.tab_state[current_tab_id].clone();

                        let current_item_id = current_tab.borrow().focus_on;
                        let current_item = current_tab
                            .try_borrow()
                            .unwrap()
                            .sub_item_state
                            .as_ref()
                            .unwrap()[current_item_id]
                            .clone();

                        stack.push(current_item);
                    }

                    // push in the leftover focused tab items
                    let mut item_point = stack.last().unwrap().clone();
                    while item_point.borrow().is_focused
                        && item_point.borrow().sub_item_state.is_some()
                    {
                        let current_item_id = item_point.borrow().focus_on;
                        let current_item = item_point
                            .try_borrow()
                            .unwrap()
                            .sub_item_state
                            .as_ref()
                            .unwrap()[current_item_id]
                            .clone();
                        item_point = current_item.clone();
                        stack.push(current_item);
                    }

                    // De-Select the focused tab items
                    while !stack.is_empty() {
                        let current_item_ptr = stack.pop().unwrap();
                        let mut current_item = current_item_ptr.borrow_mut();
                        current_item.is_selected = false;
                        current_item.is_focused = false;
                    }

                    // De-Select the focused tab
                    let current_tab_id = menu_bar_state.focus_on;
                    let mut current_tab = menu_bar_state.tab_state[current_tab_id]
                        .try_borrow_mut()
                        .unwrap();
                    current_tab.is_selected = false;
                    current_tab.is_focused = false;
                }
                _ => unreachable!(),
            }
            *focus_on = focus::FocusOn::MainPanel;
        }
        key_board_handler::EventResult::MenuPrevItem => {
            match focus_on {
                focus::FocusOn::MenuBar(focus::Menu::Tab) => {
                    // Move focus to prev tab
                    let current_tab = menu_bar_state.focus_on;
                    let prev_tab = if current_tab != 0 {
                        current_tab - 1
                    } else {
                        current_tab
                    };
                    menu_bar_state.focus_on = prev_tab;

                    menu_bar_state.tab_state[current_tab]
                        .borrow_mut()
                        .is_focused = false;
                    menu_bar_state.tab_state[prev_tab].borrow_mut().is_focused = true;
                }
                focus::FocusOn::MenuBar(focus::Menu::TabItem(_)) => {
                    let current_tab = menu_bar_state.focus_on;
                    // Move focus to prev tab item
                    let current_item = menu_bar_state.tab_state[current_tab].borrow().focus_on;
                    let prev_item = if current_item != 0 {
                        current_item - 1
                    } else {
                        current_item
                    };
                    menu_bar_state.tab_state[current_tab].borrow_mut().focus_on = prev_item;

                    let current_tab = menu_bar_state.tab_state[current_tab].borrow();
                    current_tab.sub_item_state.as_ref().unwrap()[current_item]
                        .borrow_mut()
                        .is_focused = false;
                    current_tab.sub_item_state.as_ref().unwrap()[prev_item]
                        .borrow_mut()
                        .is_focused = true;
                }
                _ => unreachable!(),
            }
        }
        key_board_handler::EventResult::MenuNextItem => {
            match focus_on {
                focus::FocusOn::MenuBar(focus::Menu::Tab) => {
                    // Move focus to next tab
                    let current_tab = menu_bar_state.focus_on;
                    let next_tab = if current_tab != menu_bar_state.tab_state.len() - 1 {
                        current_tab + 1
                    } else {
                        current_tab
                    };
                    menu_bar_state.focus_on = next_tab;

                    menu_bar_state.tab_state[current_tab]
                        .borrow_mut()
                        .is_focused = false;
                    menu_bar_state.tab_state[next_tab].borrow_mut().is_focused = true;
                }
                focus::FocusOn::MenuBar(focus::Menu::TabItem(_)) => {
                    // Move focus to next tab item
                    let current_tab = menu_bar_state.focus_on;
                    let current_item = menu_bar_state.tab_state[current_tab].borrow().focus_on;
                    let next_item = if current_item
                        != menu_bar_state.tab_state[current_tab]
                            .borrow()
                            .sub_item_state
                            .as_ref()
                            .unwrap()
                            .len()
                            - 1
                    {
                        current_item + 1
                    } else {
                        current_item
                    };
                    menu_bar_state.tab_state[current_tab].borrow_mut().focus_on = next_item;

                    let current_tab = menu_bar_state.tab_state[current_tab].borrow();
                    current_tab.sub_item_state.as_ref().unwrap()[current_item]
                        .borrow_mut()
                        .is_focused = false;
                    current_tab.sub_item_state.as_ref().unwrap()[next_item]
                        .borrow_mut()
                        .is_focused = true;
                }
                _ => unreachable!(),
            }
        }
        key_board_handler::EventResult::MenuChoose => {
            match focus_on {
                focus::FocusOn::MenuBar(focus::Menu::Tab) => {
                    // Select the focused tab
                    let current_tab = menu_bar_state.focus_on;
                    menu_bar_state.tab_state[current_tab]
                        .borrow_mut()
                        .is_selected = true;

                    if menu_bar_state.tab_state[menu_bar_state.focus_on]
                        .borrow()
                        .sub_item_state
                        .is_some()
                    {
                        // if the tab has sub-items
                        // move focus to the first sub tab item
                        *focus_on = focus::FocusOn::MenuBar(focus::Menu::TabItem(0));
                        menu_bar_state.tab_state[current_tab].borrow_mut().focus_on = 0;
                        menu_bar_state.tab_state[current_tab]
                            .borrow()
                            .sub_item_state
                            .as_ref()
                            .unwrap()[0]
                            .borrow_mut()
                            .is_focused = true;
                    } else {
                        // [ ] If the tab has no sub-items
                        // choose the tab
                        // and execute the method
                        // and quit the menu
                    }
                }
                focus::FocusOn::MenuBar(focus::Menu::TabItem(_)) => {
                    // [ ] If the focus is on the tab item
                    // if the tab item has sub-items ...
                    //  select the focused tab item and move focus to the first sub tab item
                    // if the tab item has no sub-items ...
                    //  choose the tab item
                    //  and execute the method
                    //  and quit the menu
                }
                _ => unreachable!(),
            }
        }
        key_board_handler::EventResult::PrevMenuLevel => {
            match *focus_on {
                focus::FocusOn::MenuBar(focus::Menu::TabItem(0)) => {
                    // De-Select the focused tab item and move focus to it
                    let current_tab = menu_bar_state.focus_on;
                    menu_bar_state.tab_state[current_tab]
                        .borrow_mut()
                        .is_selected = false;
                    *focus_on = focus::FocusOn::MenuBar(focus::Menu::Tab);
                }
                focus::FocusOn::MenuBar(focus::Menu::TabItem(_level)) => {
                    // [ ] If the focus is on the other tab_item_level
                    // quit to the previous tab_item_level
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
