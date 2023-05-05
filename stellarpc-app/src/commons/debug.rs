use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;

#[allow(dead_code)]
pub fn log_to_file<T: Debug>(data: T) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("debug.log")
        .unwrap();

    if let Err(e) = writeln!(file, "{:?}", data) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
