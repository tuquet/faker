mod generator;

use serde_json::{json, Value};
use std::fs::{self, OpenOptions};
use std::io::Write;

pub fn log_to_file(level: &str, msg: &str) {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let line = format!("[{}] [{}] {}\n", now, level, msg);

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("app.log") {
        let _ = file.write_all(line.as_bytes());
    }

    eprintln!("[{}] {}", level, msg);
}

#[tauri::command]
fn log_client_message(level: String, message: String) {
    log_to_file(&level, &message);
}

#[tauri::command]
async fn fetch_users(
    count: u32,
    gender: Option<String>,
    nat: Option<String>,
    _mode: Option<String>,
) -> Result<Value, String> {
    // KISS & YAGNI: Always use instant, offline-first Rust generator with rich Vietnamese & International dataset
    let mut data = generator::generate_local_users(count, gender.as_deref(), nat.as_deref());
    if let Some(obj) = data.as_object_mut() {
        obj.insert("source".to_string(), json!("Offline Rust Engine (0ms)"));
        obj.insert("offlineFallback".to_string(), json!(false));
    }
    Ok(data)
}

#[tauri::command]
async fn save_file_dialog(default_name: String, content: String, extension: String) -> Result<Option<String>, String> {
    let filter_name = match extension.as_str() {
        "csv" => "CSV Files (*.csv)",
        "json" => "JSON Files (*.json)",
        _ => "All Files (*.*)",
    };

    let path = tauri::async_runtime::spawn_blocking(move || {
        rfd::FileDialog::new()
            .set_file_name(&default_name)
            .add_filter(filter_name, &[&extension])
            .save_file()
    })
    .await
    .map_err(|e| e.to_string())?;

    if let Some(file_path) = path {
        fs::write(&file_path, content.as_bytes()).map_err(|e| e.to_string())?;
        return Ok(Some(file_path.to_string_lossy().to_string()));
    }

    Ok(None)
}

#[tauri::command]
async fn export_bundle_dialog(users: Vec<Value>) -> Result<Option<String>, String> {
    let folder = tauri::async_runtime::spawn_blocking(move || {
        rfd::FileDialog::new()
            .set_title("Chọn thư mục lưu gói dữ liệu hoàn chỉnh (Bundle)")
            .pick_folder()
    })
    .await
    .map_err(|e| e.to_string())?;

    if let Some(folder_path) = folder {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let target_dir = folder_path.join(format!("tuquet_users_{}", timestamp));
        let avatars_dir = target_dir.join("avatars");

        fs::create_dir_all(&avatars_dir).map_err(|e| e.to_string())?;

        let mut csv_rows = vec![
            "STT,Ho Ten,Chuc Danh,Gioi Tinh,Quoc Tich,Email,So Dien Thoai,Avatar File,Avatar URL,Dia Chi,Thanh Pho,Quoc Gia,CCCD/SSN,Username,Password".to_string()
        ];

        for (i, u) in users.iter().enumerate() {
            let username = u["login"]["username"].as_str().unwrap_or("user");
            let svg_filename = format!("{}.svg", username);
            let svg_file_path = avatars_dir.join(&svg_filename);

            if let Some(data_uri) = u["picture"]["data_uri"].as_str() {
                if let Some(svg_content) = data_uri.strip_prefix("data:image/svg+xml;utf8,") {
                    let _ = fs::write(&svg_file_path, svg_content);
                }
            }

            let full_name = format!("{} {}", u["name"]["first"].as_str().unwrap_or(""), u["name"]["last"].as_str().unwrap_or(""));
            let job = u["job"].as_str().unwrap_or("");
            let gender = u["gender"].as_str().unwrap_or("");
            let nat = u["nat"].as_str().unwrap_or("");
            let email = u["email"].as_str().unwrap_or("");
            let phone = u["phone"].as_str().unwrap_or("");
            let avatar_file = format!("./avatars/{}", svg_filename);
            let avatar_url = u["picture"]["large"].as_str().unwrap_or("");
            let street = format!("{} {}", u["location"]["street"]["number"], u["location"]["street"]["name"].as_str().unwrap_or(""));
            let city = u["location"]["city"].as_str().unwrap_or("");
            let country = u["location"]["country"].as_str().unwrap_or("");
            let id_val = u["id"]["value"].as_str().unwrap_or("");
            let password = u["login"]["password"].as_str().unwrap_or("");

            let escape = |s: &str| format!("\"{}\"", s.replace('"', "\"\""));

            csv_rows.push(format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                i + 1,
                escape(&full_name),
                escape(job),
                escape(gender),
                escape(nat),
                escape(email),
                escape(phone),
                escape(&avatar_file),
                escape(avatar_url),
                escape(&street),
                escape(city),
                escape(country),
                escape(id_val),
                escape(username),
                escape(password)
            ));
        }

        // Write users.csv
        let csv_path = target_dir.join("users.csv");
        fs::write(&csv_path, csv_rows.join("\r\n")).map_err(|e| e.to_string())?;

        // Write users.json
        let json_path = target_dir.join("users.json");
        let json_content = serde_json::to_string_pretty(&users).map_err(|e| e.to_string())?;
        fs::write(&json_path, json_content).map_err(|e| e.to_string())?;

        return Ok(Some(target_dir.to_string_lossy().to_string()));
    }

    Ok(None)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_users,
            save_file_dialog,
            export_bundle_dialog,
            log_client_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_users() {
        let res = fetch_users(5, Some("male".to_string()), Some("VN".to_string()), None).await;
        assert!(res.is_ok());
        let val = res.unwrap();
        let list = val["results"].as_array().expect("results should be an array");
        assert_eq!(list.len(), 5);
    }
}
