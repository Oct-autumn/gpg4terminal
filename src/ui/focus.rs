#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FocusOn {
    MenuBar(Menu), // MenuBar
    MainPanel,     // MainPanel
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Tab,          // MenuTab
    TabItem(u16), // MenuTabItem(Level)
}
