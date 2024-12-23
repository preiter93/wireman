use wireman_theme::{Config, Theme};

fn main() {
    let file = String::from("assets/default.toml");
    Theme::init(&Config::new(Some(file)));

    let theme = Theme::global();
    println!("Theme: {:?}", theme.base);
}
