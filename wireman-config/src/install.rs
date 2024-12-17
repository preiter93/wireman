use std::{env::var, io::Write, path::Path};

use crate::{Config, CONFIG_FNAME, DEFAULT_CONFIG_DIR, ENV_CONFIG_DIR};

pub fn install() {
    println!("****** Install Wireman ******");
    println!();

    let mut config_dir = if let Ok(env_config_dir) = var(ENV_CONFIG_DIR) {
        env_config_dir
    } else {
        DEFAULT_CONFIG_DIR.to_string()
    };

    let mut non_default_directory = false;
    let input = read_input(&format!("Install wireman to {config_dir}? [y/n]"));
    if input.trim().to_lowercase() != "y" {
        let input = read_input("Install instead in");
        config_dir = input.trim().to_string();
        if expand_path(&config_dir) != expand_path(DEFAULT_CONFIG_DIR) {
            non_default_directory = true;
        }
    }

    if let Err(err) = create_directory_if_missing(expand_path(&config_dir)) {
        println!("Could not create {config_dir:?}: {err}. ABORT.");
        return;
    }

    match write_config_to_toml(expand_path(&config_dir)) {
        Ok(should_continue) => {
            if !should_continue {
                return;
            }
        }
        Err(err) => {
            println!("Could not write config: {err}. ABORT.");
            return;
        }
    }

    println!();
    println!("****** Further Information ******");
    println!();
    if non_default_directory {
        println!("*** Set the {ENV_CONFIG_DIR} environment variable ***");
        println!();
        println!(
        "   Add the following line to your shell configuration file (e.g., `.bashrc`, `.zshrc`):"
    );
        println!();
        println!("   export {ENV_CONFIG_DIR}=${config_dir}");
        println!();
    }
    println!("*** Include directories and proto files ***");
    println!();
    println!("The Wireman configuration file is found here: ");
    println!("   {config_dir}/{CONFIG_FNAME}");
    println!();
    println!("Specify the include directories and proto files.");
    println!("Here’s an example structure:");
    println!();
    println!("   includes = [");
    println!("       \"$HOME/my-project/services\",");
    println!("       \"$HOME/my-project/protos\",");
    println!("   ]");
    println!("   files = [");
    println!("       \"order/api.proto\",");
    println!("       \"price/api.proto\"");
    println!("   ]");
    println!();
    println!();
    println!("For more information, visit the Wireman configuration guide:");
    println!("🔗 https://github.com/preiter93/wireman?tab=readme-ov-file#configuration");
    println!();
    println!("✅ Once you've completed these steps, you're ready to use Wireman!");
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
        shellexpand::tilde(&shellexpand::env(path).map_or(path.to_string(), |x| x.to_string()))
            .as_ref(),
    )
}

pub(crate) fn expand_file(path: &str) -> String {
    shellexpand::tilde(&shellexpand::env(path).map_or(path.to_string(), |x| x.to_string()))
        .to_string()
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
        let input = read_input("A config exists already. Do you want to overwrite it? [y/n]");
        return Ok(input.trim() == "y");
    }

    let mut config = Config::default();
    config.server.default_address = Some(String::new());
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
