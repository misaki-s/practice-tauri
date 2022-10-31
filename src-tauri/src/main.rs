#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("rust@greet {}",name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn simple_command(filepath: &str) -> String {
    println!("rust@simple_command {}",filepath);
    use std::process::Command;
    let win_cmd = format!("start {}", filepath);
    let output = if cfg!(target_os = "windows") {
        // windows用のコマンド
        Command::new("cmd")
                .args(["/C", &win_cmd])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
    // let hello = output.stdout;
    format!("simple_command: {}", filepath)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            simple_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
