use wireman_theme::{Config, Theme};

fn main() {
    let file = String::from("assets/default.toml");
    Theme::init(&Config::new(Some(file)));

    let theme = Theme::global();
    println!("Theme base: {:?}", theme.base);
    println!(
        "Border type focused: {:?}",
        theme.border.border_type_focused
    );
    println!(
        "Border type unfocused: {:?}",
        theme.border.border_type_unfocused
    );
}
