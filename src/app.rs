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
        focus,
        menu_bar::{self, init_menu_bar_state, render_menu_bar},
        UiState,
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
                focus_on: focus::FocusOn::MainPanel,
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
            let result = result.unwrap();
            match result {
                (true, key_board_handler::EventResult::Quit) => {
                    self.running_state = AppState::Quitting;
                }
                (true, key_board_handler::EventResult::EnterMenuBar)
                | (true, key_board_handler::EventResult::QuitMenu)
                | (true, key_board_handler::EventResult::MenuPrevItem)
                | (true, key_board_handler::EventResult::MenuNextItem)
                | (true, key_board_handler::EventResult::MenuChoose)
                | (true, key_board_handler::EventResult::PrevMenuLevel) => {
                    menu_bar::handle_event(
                        &mut self.ui_state.focus_on,
                        self.ui_state.menu_bar_state.clone(),
                        result.1,
                    );
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
    let keys: Vec<(&str, &str)> = match ui_state.focus_on {
        focus::FocusOn::MainPanel => [
            ("Q", "Quit"),
            ("Tab", "Menu"),
            ("Up/Down", "Select"),
            ("Enter", "Choose"),
        ]
        .to_vec(),
        focus::FocusOn::MenuBar(focus::Menu::Tab) => [
            ("L/R", "Select"),
            ("Enter/Down", "Choose"),
            ("Tab/Esc", "Quit Menu"),
        ]
        .to_vec(),
        focus::FocusOn::MenuBar(focus::Menu::TabItem(0)) => [
            ("Up/Down", "Select"),
            ("Enter", "Choose"),
            ("Esc", "Back"),
            ("Tab", "Quit Menu"),
        ]
        .to_vec(),
        _ => vec![],
    };

    let mut spans: Vec<Span> = keys
        .iter()
        .flat_map(|(key, desc)| {
            let key = Span::styled(format!(" {key} "), THEME.hint_bar.key);
            let desc = Span::styled(format!(" {desc}  "), THEME.hint_bar.description);
            [key, desc]
        })
        .collect();
    spans.insert(0, Span::styled("  ", THEME.hint_bar.default_style));

    Line::from(spans)
        .left_aligned()
        .style(THEME.hint_bar.default_style)
        .render(area, buf);
}
