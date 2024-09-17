mod rmbg;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::ImageFormat::Png;
use rmbg::Rmbg;
use std::io::Cursor;
use tauri::{path::BaseDirectory, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn remove_bg_from_select(handle: tauri::AppHandle, path: String) -> String {
    let resource_path = handle
        .path()
        .resolve("resources/rmbg.onnx", BaseDirectory::Resource)
        .unwrap();

    let rmbg = Rmbg::new(resource_path).unwrap();

    // Load an image
    let original_img = image::open(path).unwrap();

    // Remove the background
    let img_without_bg = rmbg.remove_background(&original_img).unwrap();

    let mut image_data: Vec<u8> = Vec::new();
    img_without_bg
        .write_to(&mut Cursor::new(&mut image_data), Png)
        .unwrap();
    // turn to bytes
    let b64 = STANDARD.encode(image_data);
    return b64;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![remove_bg_from_select])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
