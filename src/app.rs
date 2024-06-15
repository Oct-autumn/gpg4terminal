use std::io::Result;

use ratatui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
    Terminal,
};

use crate::{
    event_handler::key_board_handler,
    theme::THEME,
    ui::{
        menu_bar::{init_menu_bar_state, render_menu_bar},
        FocusOn, UiState,
    },
};

pub struct App {
    running_state: AppState,
    ui_state: UiState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running_state: Default::default(),
            // Initialize the UiState
            ui_state: UiState {
                focus_on: FocusOn::MainPanel,
                menu_bar_state: init_menu_bar_state(),
            },
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

impl App {
    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while self.running_state == AppState::Running {
            self.draw(terminal)?;
            self.handle_events();
        }
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) {
        let result = key_board_handler::handle_events(&self.ui_state);
        if result.is_ok() {
            match result.unwrap() {
                (true, key_board_handler::EventResult::Quit) => {
                    self.running_state = AppState::Quitting;
                }
                (true, key_board_handler::EventResult::EnterMenuBar) => {
                    self.ui_state.focus_on = FocusOn::MenuTab;
                    // Reset focus on the first tab
                    self.ui_state.menu_bar_state.focus_on = 0;
                    self.ui_state.menu_bar_state.tab_state[0].is_focused = true;
                }
                (true, key_board_handler::EventResult::QuitMenu) => {
                    self.ui_state.focus_on = FocusOn::MainPanel;
                    // Recursively reset selection and focus state
                    for tab in &mut self.ui_state.menu_bar_state.tab_state {
                        tab.is_focused = false;
                        tab.is_selected = false;
                        if let Some(sub_items) = &mut tab.sub_item_state {
                            for item in sub_items {
                                item.is_focused = false;
                                item.is_selected = false;
                            }
                        }
                    }
                }
                (true, key_board_handler::EventResult::MenuPrevItem) => {
                    if self.ui_state.focus_on == FocusOn::MenuTab {
                        // Move focus to prev tab
                        let current_tab = self.ui_state.menu_bar_state.focus_on;
                        let prev_tab = if current_tab != 0 {
                            current_tab - 1
                        } else {
                            current_tab
                        };
                        self.ui_state.menu_bar_state.focus_on = prev_tab;

                        self.ui_state.menu_bar_state.tab_state[current_tab].is_focused = false;
                        self.ui_state.menu_bar_state.tab_state[prev_tab].is_focused = true;
                    } else {
                        let current_tab = self.ui_state.menu_bar_state.focus_on;
                        // Move focus to prev tab item
                        let current_item =
                            self.ui_state.menu_bar_state.tab_state[current_tab].focus_on;
                        let prev_item = if current_item != 0 {
                            current_item - 1
                        } else {
                            current_item
                        };
                        self.ui_state.menu_bar_state.tab_state[current_tab].focus_on = prev_item;

                        self.ui_state.menu_bar_state.tab_state[current_tab]
                            .sub_item_state
                            .as_mut()
                            .unwrap()[current_item]
                            .is_focused = false;
                        self.ui_state.menu_bar_state.tab_state[current_tab]
                            .sub_item_state
                            .as_mut()
                            .unwrap()[prev_item]
                            .is_focused = true;
                    }
                }
                (true, key_board_handler::EventResult::MenuNextItem) => {
                    if self.ui_state.focus_on == FocusOn::MenuTab {
                        // Move focus to next tab
                        let current_tab = self.ui_state.menu_bar_state.focus_on;
                        let next_tab =
                            if current_tab != self.ui_state.menu_bar_state.tab_state.len() - 1 {
                                current_tab + 1
                            } else {
                                current_tab
                            };
                        self.ui_state.menu_bar_state.focus_on = next_tab;

                        self.ui_state.menu_bar_state.tab_state[current_tab].is_focused = false;
                        self.ui_state.menu_bar_state.tab_state[next_tab].is_focused = true;
                    } else {
                        // Move focus to prev tab item
                        let current_tab = self.ui_state.menu_bar_state.focus_on;
                        let current_item =
                            self.ui_state.menu_bar_state.tab_state[current_tab].focus_on;
                        let next_item = if current_item
                            != self.ui_state.menu_bar_state.tab_state[current_tab]
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
                        self.ui_state.menu_bar_state.tab_state[current_tab].focus_on = next_item;

                        self.ui_state.menu_bar_state.tab_state[current_tab]
                            .sub_item_state
                            .as_mut()
                            .unwrap()[current_item]
                            .is_focused = false;
                        self.ui_state.menu_bar_state.tab_state[current_tab]
                            .sub_item_state
                            .as_mut()
                            .unwrap()[next_item]
                            .is_focused = true;
                    }
                }
                (true, key_board_handler::EventResult::MenuChoose) => {
                    if self.ui_state.focus_on == FocusOn::MenuTab {
                        // Select the focused tab and move focus to the first tab item
                        let current_tab = self.ui_state.menu_bar_state.focus_on;
                        self.ui_state.menu_bar_state.tab_state[current_tab].is_selected = true;
                        if self.ui_state.menu_bar_state.tab_state
                            [self.ui_state.menu_bar_state.focus_on]
                            .sub_item_state
                            .is_some()
                        {
                            self.ui_state.focus_on = FocusOn::MenuTabItem;
                            self.ui_state.menu_bar_state.tab_state[current_tab].focus_on = 0;
                            self.ui_state.menu_bar_state.tab_state[current_tab]
                                .sub_item_state
                                .as_mut()
                                .unwrap()[0]
                                .is_focused = true;
                        } else {
                        }
                    }
                }
                (true, key_board_handler::EventResult::PrevMenuLevel) => {
                    // De-Select the focused tab and move focus to it
                    let current_tab = self.ui_state.menu_bar_state.focus_on;
                    self.ui_state.menu_bar_state.tab_state[current_tab].is_selected = false;
                    self.ui_state.focus_on = FocusOn::MenuTab;
                }
                _ => (),
            }
        } else {
            // Handle Error
            todo!();
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ],
        )
        .split(area);
        render_menu_bar(&self.ui_state, main_layout[0], buf);
        render_bottom_bar(&self.ui_state, main_layout[2], buf);
    }
}

fn render_bottom_bar(ui_state: &UiState, area: Rect, buf: &mut Buffer) {
    let keys: Vec<(&str, &str)>;
    if ui_state.focus_on == crate::ui::FocusOn::MenuTab
        || ui_state.focus_on == crate::ui::FocusOn::MenuTabItem
    {
        // In Menu
        keys = [
            ("Tab/Esc", "Back/Quit Menu"),
            ("Up/Down/Left/Right", "Select"),
            ("Enter", "Choose"),
        ]
        .to_vec();
    } else {
        // In Main
        keys = [
            ("Q", "Quit"),
            ("Tab", "Menu"),
            ("Up/Down", "Select"),
            ("Enter", "Choose"),
        ]
        .to_vec();
    }
    let spans: Vec<_> = keys
        .iter()
        .flat_map(|(key, desc)| {
            let key = Span::styled(format!(" {key} "), THEME.hint_bar.key);
            let desc = Span::styled(format!(" {desc}  "), THEME.hint_bar.description);
            [key, desc]
        })
        .collect();
    Line::from(spans)
        .left_aligned()
        .style(THEME.hint_bar.default_style)
        .render(area, buf);
}
