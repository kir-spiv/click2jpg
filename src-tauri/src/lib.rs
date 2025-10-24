use std::path::PathBuf;

use tauri::Emitter;

#[tauri::command]
async fn convert_images_to_jpeg(
    app_handle: tauri::AppHandle,
    paths: Vec<String>,
    output_dir: String,
) -> Result<Vec<String>, String> {
    let output_path = PathBuf::from(&output_dir);
    if !output_path.exists() {
        std::fs::create_dir_all(&output_path)
            .map_err(|e| format!("Не удалось создать папку {}: {}", output_dir, e))?;
    }
    let mut output_paths = Vec::new();

    for (i, path_str) in paths.iter().enumerate() {
        let input_path = PathBuf::from(path_str);
        if !input_path.exists() {
            continue;
        }

        let img = image::open(&input_path)
            .map_err(|e| format!("Не удалось открыть {}: {}", path_str, e))?;
        let file_name = input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("image");
        let mut output_file = output_path.join(format!("{}.jpg", file_name));
        let mut counter = 1;
        let original_stem = file_name.to_string();
        while output_file.exists() {
            output_file = output_path.join(format!("{}_{}.jpg", original_stem, counter));
            counter += 1;
        }
        img.save_with_format(&output_file, image::ImageFormat::Jpeg)
            .map_err(|e| format!("Не удалось сохранить {}: {}", output_file.display(), e))?;
        output_paths.push(output_file.to_string_lossy().into_owned());
        let progress = format!("{}/{}", i + 1, paths.len());
        app_handle
            .emit("conversion_progress", &progress)
            .unwrap_or(());
    }

    Ok(output_paths)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    libheif_rs::integration::image::register_all_decoding_hooks();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![convert_images_to_jpeg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
