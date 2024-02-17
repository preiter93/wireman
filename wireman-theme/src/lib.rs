use once_cell::sync::OnceCell;

pub static THEME: OnceCell<Theme> = OnceCell::new();

#[derive(Debug)]
pub struct Theme {
    pub root: RootTheme,
}

#[derive(Debug)]
pub struct RootTheme {
    pub hide_footer_help: bool,
}

impl Theme {
    #[must_use]
    pub fn new(root: RootTheme) -> Self {
        Self { root }
    }

    #[must_use]
    pub fn global() -> &'static Theme {
        THEME.get().expect("Theme is not initialized")
    }
}
