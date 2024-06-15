use std::io::Result;

use crossterm::event::{self, Event, KeyCode};

use crate::ui::UiState;

/*
 枚举 捕获的事件
*/
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    #[default]
    None,
    EnterMenuBar,
    QuitMenu,
    MenuNextItem,
    MenuPrevItem,
    MenuChoose,
    PrevMenuLevel,
    Quit,
}

pub fn handle_events(ui_state: &UiState) -> Result<(bool, EventResult)> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match ui_state.focus_on {
                    crate::ui::FocusOn::MainPanel => {
                        // `Q` can quit program when in MainPanel
                        if key.code == KeyCode::Char('q') {
                            return Ok((true, EventResult::Quit));
                        }
                        // Tab can enter the menu bar when in MainPanel
                        if key.code == KeyCode::Tab
                            && ui_state.focus_on != crate::ui::FocusOn::MenuTab
                            && ui_state.focus_on != crate::ui::FocusOn::MenuTabItem
                        {
                            return Ok((true, EventResult::EnterMenuBar));
                        }
                    }
                    crate::ui::FocusOn::MenuTab => {
                        // `L/R` can select the next/previous menu tab when focus is on MenuTab
                        if key.code == KeyCode::Left {
                            return Ok((true, EventResult::MenuPrevItem));
                        }
                        if key.code == KeyCode::Right {
                            return Ok((true, EventResult::MenuNextItem));
                        }
                        // `Enter` can choose menu item when focus is on MenuTab
                        // `Down` can choose the menu tab when focus is on MenuTab
                        if key.code == KeyCode::Enter || key.code == KeyCode::Down {
                            return Ok((true, EventResult::MenuChoose));
                        }
                        // `Tab` can quit menu when focus is on MenuTab
                        // `Esc` can quit menu when focus is on MenuTab
                        if key.code == KeyCode::Tab || key.code == KeyCode::Esc {
                            return Ok((true, EventResult::QuitMenu));
                        }
                    }
                    crate::ui::FocusOn::MenuTabItem => {
                        // `Esc` can go back to previous menu when focus is on MenuTabItem
                        if key.code == KeyCode::Esc {
                            return Ok((true, EventResult::PrevMenuLevel));
                        }
                        // `Up/Down` can select the previous/next menu item when focus is on MenuTabItem
                        // `Up` can go back to the menu tab when focus is on MenuTabItem and the first item is focused
                        if key.code == KeyCode::Up {
                            if ui_state.menu_bar_state.tab_state[ui_state.menu_bar_state.focus_on]
                                .sub_item_state
                                .is_some()
                                && ui_state.menu_bar_state.tab_state
                                    [ui_state.menu_bar_state.focus_on]
                                    .sub_item_state
                                    .as_ref()
                                    .unwrap()[0]
                                    .is_focused
                            {
                                return Ok((true, EventResult::PrevMenuLevel));
                            }
                            return Ok((true, EventResult::MenuPrevItem));
                        }
                        if key.code == KeyCode::Down {
                            return Ok((true, EventResult::MenuNextItem));
                        }
                        // `Enter` can choose menu item when focus is on MenuTabItem
                        if key.code == KeyCode::Enter {
                            return Ok((true, EventResult::MenuChoose));
                        }
                        // `Tab` can quit menu when focus is on MenuTabItem
                        if key.code == KeyCode::Tab {
                            return Ok((true, EventResult::QuitMenu));
                        }
                    }
                }
            }
        }
    }
    Ok((false, EventResult::None))
}
