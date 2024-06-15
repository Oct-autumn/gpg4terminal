use std::alloc::System;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

/// Style for the MenuBar
pub struct MenuBarStyle {
    title_style: Style,
    default_style: Style,
    disabled_style: Style,
    focused_style: Style,
    selected_style: Style,
}

impl MenuBarStyle {
    pub fn default() -> Self {
        Self {
            title_style: Style::default()
                .bg(Color::Black)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            default_style: Style::default().bg(Color::Black).fg(Color::Gray),
            disabled_style: Style::default().bg(Color::Black).fg(Color::DarkGray),
            focused_style: Style::default().bg(Color::Gray).fg(Color::DarkGray),
            selected_style: Style::default().bg(Color::DarkGray).fg(Color::Gray),
        }
    }
    pub fn new(
        title_style: Style,
        default_style: Style,
        disabled_style: Style,
        focused_style: Style,
        selected_style: Style,
    ) -> Self {
        Self {
            title_style,
            default_style,
            disabled_style,
            focused_style,
            selected_style,
        }
    }
}

/// MenuBar widget
pub struct MenuBar<'a> {
    /// Title of the MenuBar. Usually will be the name of the application
    title: Option<String>,
    /// List of MenuTabs
    menu_tabs: Vec<MenuTab<'a>>,
    state: &'a MenuBarUiState,
    menu_bar_style: &'a MenuBarStyle,
}

/// MenuTab widget
pub struct MenuTab<'a> {
    label: String,
    hotkey: Option<&'a str>,
    menu_tab_items: Option<Vec<MenuTabItem<'a>>>,
    state: &'a MenuTabUiState,
    menu_bar_style: &'a MenuBarStyle,
}

/// MenuTabItem widget
pub struct MenuTabItem<'a> {
    label: String,
    hotkey: Option<&'a str>,
    sub_menu_tab_items: Option<Vec<MenuTabItem<'a>>>,
    state: &'a MenuTabItemUiState,
    menu_bar_style: &'a MenuBarStyle,
}

/// MenuBar UI State
///
/// The root of the state tree for the MenuBar widget
#[derive(Clone, PartialEq, Eq)]
pub struct MenuBarUiState {
    pub focus_on: usize,
    pub tab_state: Vec<MenuTabUiState>,
}

/// MenuTab UI State
///
/// The second level of the state tree for the MenuBar widget
#[derive(Clone, PartialEq, Eq)]
pub struct MenuTabUiState {
    pub is_focused: bool,
    pub focus_on: usize,
    pub is_disabled: bool,
    pub is_selected: bool,
    pub sub_item_state: Option<Vec<MenuTabItemUiState>>,
}

/// MenuTabItem UI State
///
/// The third+ level of the state tree for the MenuBar widget
#[derive(Clone, PartialEq, Eq)]
pub struct MenuTabItemUiState {
    pub is_focused: bool,
    pub focus_on: usize,
    pub is_disabled: bool,
    pub is_selected: bool,
    pub sub_item_state: Option<Vec<MenuTabItemUiState>>,
}

impl<'a> MenuBar<'a> {
    pub fn new(
        title: Option<String>,
        menu_tabs: Vec<MenuTab<'a>>,
        state: &'a MenuBarUiState,
        menu_bar_style: &'a MenuBarStyle,
    ) -> Self {
        assert!(
            menu_tabs.len() > 0,
            "Menu bar must have at least one menu tab"
        );
        Self {
            title,
            menu_tabs,
            menu_bar_style,
            state,
        }
    }
}

impl<'a> Widget for &MenuBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render the menu bar
        //  Calculate the layout for the menu bar
        let mut constraints = vec![];
        // If there is a title
        if self.title.is_some() {
            constraints.push(Constraint::Length(
                self.title.clone().unwrap().len() as u16 + 2,
            ));
        }

        for (i, menu_tab) in self.menu_tabs.iter().enumerate() {
            let len: u16;
            if menu_tab.hotkey.is_some() {
                let hotkey = menu_tab.hotkey.unwrap();
                len = (menu_tab.label.len() + hotkey.len()) as u16 + 4;
            } else {
                len = menu_tab.label.len() as u16 + 2;
            }
            constraints.push(Constraint::Length(len));
        }
        constraints.push(Constraint::Min(0));

        let layout = Layout::new(Direction::Horizontal, constraints).split(area);

        // If there is a title
        Span::styled(
            format!(" {title} ─", title = self.title.clone().unwrap()),
            self.menu_bar_style.title_style,
        )
        .render(layout[0], buf);

        // Always render the menu tabs
        //  Render the menu tabs
        for (i, menu_tab) in self.menu_tabs.iter().enumerate() {
            menu_tab.render(layout[i + 1], buf);
        }

        // Fill the left over space
        Block::default()
            .borders(Borders::TOP)
            .style(self.menu_bar_style.default_style)
            .render(layout[layout.len() - 1], buf);
    }
}

impl<'a> MenuTab<'a> {
    pub fn new(
        label: String,
        hotkey: Option<&'a str>,
        menu_tab_items: Option<Vec<MenuTabItem<'a>>>,
        state: &'a MenuTabUiState,
        menu_bar_style: &'a MenuBarStyle,
    ) -> Self {
        Self {
            label,
            hotkey,
            menu_tab_items,
            state,
            menu_bar_style,
        }
    }
}

impl<'a> Widget for &MenuTab<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render the menu tab

        // Always render the menu tab self
        // Define the label
        let hotkey = match self.hotkey {
            Some(ch) => format!("({})", ch),
            None => "".to_string(),
        };
        let label = format!(" {label}{hotkey} ", label = self.label, hotkey = hotkey);
        // Define the style
        let style: Style;
        if self.state.is_focused {
            style = self.menu_bar_style.focused_style;
        } else if self.state.is_selected {
            style = self.menu_bar_style.selected_style;
        } else if self.state.is_disabled {
            style = self.menu_bar_style.disabled_style;
        } else {
            style = self.menu_bar_style.default_style;
        }

        Span::styled(label, style).render(area, buf);

        // Render the selected menu tab item
    }
}
