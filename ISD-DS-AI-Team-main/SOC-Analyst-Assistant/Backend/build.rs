use std::process::Command;

fn main() {
    // Get the current time
    let output = Command::new("date")
        .arg("+%Y-%m-%d %H:%M:%S")
        .output()
        .expect("Failed to get compile time");
    let compile_time = String::from_utf8_lossy(&output.stdout);

    // Set the environment variable
    println!("cargo:rustc-env=COMPILE_TIME={}", compile_time.trim());
}
