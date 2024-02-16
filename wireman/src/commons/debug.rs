use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;

use config::DEBUG;

#[allow(dead_code)]
pub fn log<T: Debug>(data: T) {
    if DEBUG {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("debug.log")
            .unwrap();

        if let Err(e) = writeln!(file, "{data:?}") {
            eprintln!("Couldn't write to file: {e}");
        }
    }
}
