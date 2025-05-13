use chrono::{DateTime, Local};

fn main() {
    let time: DateTime<Local> = Local::now();
    println!("cargo:rustc-env=COMPILE_TIME={}", time.format("%Y-%m-%d %H:%M:%S"));

    tauri_build::build()
}
