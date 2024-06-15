use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub content: Style,
    pub menu_bar: MenuBar,
    pub hint_bar: HintBar,
    pub borders: Style,
}

pub struct MenuBar {
    pub title_style: Style,
    pub default_style: Style,
    pub disabled_style: Style,
    pub focused_style: Style,
    pub selected_style: Style,
}

pub struct HintBar {
    pub default_style: Style,
    pub key: Style,
    pub description: Style,
}

pub const THEME: Theme = Theme {
    content: Style::new().bg(DARK_PURPLE).fg(LIGHT_GRAY),
    menu_bar: MenuBar {
        title_style: Style::new()
            .fg(WHITE)
            .bg(DARK_PURPLE)
            .add_modifier(Modifier::BOLD),
        default_style: Style::new().fg(MID_GRAY).bg(DARK_PURPLE),
        disabled_style: Style::new().fg(DARK_GRAY).bg(DARK_PURPLE),
        focused_style: Style::new().fg(LIGHT_GRAY).bg(MID_PURPLE),
        selected_style: Style::new()
            .fg(LIGHT_GRAY)
            .bg(MID_PURPLE)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED),
    },
    borders: Style::new().fg(LIGHT_GRAY),
    hint_bar: HintBar {
        default_style: Style::new().fg(MID_GRAY).bg(DARK_PURPLE),
        key: Style::new().fg(DARK_PURPLE).bg(MID_GRAY),
        description: Style::new().fg(MID_GRAY).bg(DARK_PURPLE),
    },
};

const DARK_PURPLE: Color = Color::Rgb(51, 0, 51);
const MID_PURPLE: Color = Color::Rgb(102, 80, 102);
const LIGHT_PURPLE: Color = Color::Rgb(185, 121, 192);
const LIGHT_BLUE: Color = Color::Rgb(64, 96, 192);
const LIGHT_YELLOW: Color = Color::Rgb(192, 192, 96);
const LIGHT_GREEN: Color = Color::Rgb(64, 192, 96);
const LIGHT_RED: Color = Color::Rgb(192, 96, 96);
const RED: Color = Color::Rgb(215, 0, 0);
const BLACK: Color = Color::Rgb(8, 8, 8); // not really black, often #080808
const DARK_GRAY: Color = Color::Rgb(68, 68, 68);
const MID_GRAY: Color = Color::Rgb(128, 128, 128);
const LIGHT_GRAY: Color = Color::Rgb(188, 188, 188);
const WHITE: Color = Color::Rgb(238, 238, 238); // not really white, often #eeeeee
