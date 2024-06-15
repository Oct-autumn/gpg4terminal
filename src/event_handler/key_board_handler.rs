use std::io::Result;

use crossterm::event::{
    self, Event, KeyCode, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};

use crate::ui::UiState;

/*
 枚举 捕获的事件
*/
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    #[default]
    None,
    EnterMenuBar,
    MenuUp,
    MenuDown,
    MenuLeft,
    MenuRight,
    QuitMenu,
    PreMenu,
    Quit,
}

pub fn handle_events(ui_state: &UiState) -> Result<(bool, EventResult)> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                if ui_state.focus_on == crate::ui::FocusOn::MenuTab
                    || ui_state.focus_on == crate::ui::FocusOn::MenuTabItem
                {
                    // Handle Menu Navigation Events (When in Menu)
                    if key.code == KeyCode::Up {
                        return Ok((true, EventResult::MenuUp));
                    }
                    if key.code == KeyCode::Down {
                        return Ok((true, EventResult::MenuDown));
                    }
                    if key.code == KeyCode::Left {
                        return Ok((true, EventResult::MenuLeft));
                    }
                    if key.code == KeyCode::Right {
                        return Ok((true, EventResult::MenuRight));
                    }
                    // L-Alt can quit menu at any time
                    // Esc can quit menu when focus is on MenuBar
                    if key.code == KeyCode::Tab
                        || (ui_state.focus_on == crate::ui::FocusOn::MenuTab
                            && key.code == KeyCode::Esc)
                    {
                        return Ok((true, EventResult::QuitMenu));
                    }
                    // Esc can go back to previous menu when focus is on MenuTab or
                    if ui_state.focus_on == crate::ui::FocusOn::MenuTabItem
                        && key.code == KeyCode::Esc
                    {
                        // Handle Quit Events (When in Menu)
                        return Ok((true, EventResult::PreMenu));
                    }
                } else {
                    // Handle Quit Events (When not in Menu)
                    if key.code == KeyCode::Char('q') {
                        return Ok((true, EventResult::Quit));
                    }
                    // Handle Menu Events (Always)
                    if key.code == KeyCode::Tab {
                        return Ok((true, EventResult::EnterMenuBar));
                    }
                }
            }
        }
    }
    Ok((false, EventResult::None))
}
