use std::{
    env::var,
    io::Write,
    path::{Path, PathBuf},
};

use crate::{Config, CONFIG_FNAME, DEFAULT_CONFIG_DIR, ENV_CONFIG_DIR};

pub fn install() {
    print_header("Install Wireman");

    // Determine config directory
    let mut config_dir = var(ENV_CONFIG_DIR).unwrap_or_else(|_| DEFAULT_CONFIG_DIR.to_string());

    let input = read_input(&format!(
        "Do you want to install Wireman to \x1b[1;33m{config_dir}\x1b[0m? [y/N] "
    ));
    if input.trim().to_lowercase() != "y" {
        let input = read_input("Install instead in: ");
        config_dir = input.trim().to_string();
    }

    let is_default_directory = expand_path(&config_dir) == expand_path(DEFAULT_CONFIG_DIR);

    // Create directory if missing
    if let Err(err) = create_directory_if_missing(expand_path(&config_dir)) {
        println!("\x1b[1;31mError:\x1b[0m Could not create {config_dir:?}: {err}. ABORT.");
        return;
    }

    // Write config
    match write_config_to_toml(expand_path(&config_dir)) {
        Ok(should_continue) if !should_continue => return,
        Err(err) => {
            println!("\x1b[1;31mError:\x1b[0m Could not write config: {err}. ABORT.");
            return;
        }
        _ => {}
    }

    println!();
    print_header("Further Information");
    println!();

    // Config file path
    println!("\x1b[1;34mWireman configuration file:\x1b[0m");
    println!();
    println!("    \x1b[0;37m{config_dir}/{CONFIG_FNAME}\x1b[0m");
    println!();

    // Shell config line if non-default
    if !is_default_directory {
        println!("\x1b[1;33mAdd this line to your shell configuration file:\x1b[0m");
        println!();
        println!("    \x1b[0;37mexport {ENV_CONFIG_DIR}={config_dir}\x1b[0m");
        println!();
    }

    // Include directories and proto files
    println!("\x1b[1;34mInclude directories and proto files:\x1b[0m");
    println!();
    println!("    \x1b[0;37mincludes = [");
    println!("        \"$HOME/my-project/services\",");
    println!("        \"$HOME/my-project/protos\",");
    println!("    ]");
    println!("    files = [");
    println!("        \"order/api.proto\",");
    println!("        \"price/api.proto\"");
    println!("    ]\x1b[0m");
    println!();

    // More info
    println!("\x1b[1;36mMore information:\x1b[0m");
    println!("    \x1b[0;34mhttps://github.com/preiter93/wireman?tab=readme-ov-file#setupconfiguration\x1b[0m");
    println!();
    println!("\x1b[1;32mSetup complete! You're ready to use Wireman.\x1b[0m");
}

fn print_header(text: &str) {
    // Bold + orange text
    println!("\x1b[1;33m{}\x1b[0m", text);
}

fn read_input(prompt: &str) -> String {
    print!("{prompt}: ");
    std::io::stdout().flush().unwrap();

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    input_string
}

pub(crate) fn expand_path(path: &str) -> String {
    enforce_absolute_path(
        shellexpand::tilde(
            &shellexpand::env(&expand_current_dir(path))
                .map_or(path.to_string(), |x| x.to_string()),
        )
        .as_ref(),
    )
}

pub(crate) fn expand_file(path: &str) -> String {
    shellexpand::tilde(
        &shellexpand::env(&expand_current_dir(path)).map_or(path.to_string(), |x| x.to_string()),
    )
    .to_string()
}

pub(crate) fn make_absolute_path(config_path: &str) -> String {
    let path = Path::new(config_path);

    if path.is_absolute()
        || config_path.starts_with('.')
        || config_path.starts_with('~')
        || config_path.starts_with('$')
    {
        config_path.to_string()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
            .to_string_lossy()
            .to_string()
    }
}

fn create_directory_if_missing<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

// Returns whether the installation should be continued (true) or not (false).
fn write_config_to_toml<P: AsRef<Path>>(path: P) -> std::io::Result<bool> {
    let file_path = path.as_ref().join(CONFIG_FNAME);
    if file_path.exists() {
        println!("It seems you already have a config file.");
        return Ok(true);
    }

    let mut config = Config::default();
    config.server.default_address = Some(String::from("http://localhost:50051"));
    config.server.default_auth_header = Some(String::new());

    let toml_str = toml::to_string(&config).expect("Failed to serialize config");
    std::fs::write(&file_path, toml_str)?;
    std::fs::create_dir_all(path)?;

    Ok(true)
}

fn enforce_absolute_path(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}

fn expand_current_dir(path: &str) -> String {
    if let Some(stripped) = path.strip_prefix('.') {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(stripped)
            .to_string_lossy()
            .to_string()
    } else {
        path.to_string()
    }
}

// let file_path = config_dir.join("wireman.toml");
// let Ok(config) = Config::load(&file_path.to_string_lossy().to_string()) else {
//     println!("Could not load config. ABORT.");
//     return;
// };

// if config.includes().is_empty() {
//     println!("No proto includes found in config. Would you like to add them now? (y/n)");
// }
//
// if config.files().is_empty() {
//     println!("No proto files found in config. Would you like to add them now? (y/n)");
// }
//
