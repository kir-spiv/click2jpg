use std::path::PathBuf;

#[tauri::command]
async fn convert_images_to_jpeg(paths: Vec<String>) -> Result<Vec<String>, String> {
    let mut output_paths = Vec::new();

    for path_str in paths {
        let input_path = PathBuf::from(path_str);
        if !input_path.exists() {
            continue;
        }

        let img = match image::open(&input_path) {
            Ok(img) => img,
            Err(e) => {
                return Err(format!(
                    "Не удалось открыть {}: {}",
                    input_path.display(),
                    e
                ))
            }
        };

        let mut output_path = input_path.clone();
        output_path.set_extension("jpg");

        let mut counter = 1;
        let original_stem = output_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        while output_path.exists() {
            let mut new_stem = original_stem.clone();
            new_stem.push_str(&format!("_{}", counter));
            output_path.set_file_name(new_stem);
            output_path.set_extension("jpg");
            counter += 1;
        }
        if let Err(e) = img.save_with_format(&output_path, image::ImageFormat::Jpeg) {
            return Err(format!(
                "Не удалось сохранить {}: {}",
                output_path.display(),
                e
            ));
        }

        output_paths.push(output_path.to_string_lossy().into_owned());
    }

    Ok(output_paths)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![convert_images_to_jpeg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
