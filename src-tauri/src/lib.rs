mod generator;

use serde_json::{json, Value};
use std::fs;
use std::time::Duration;

#[tauri::command]
async fn fetch_users(
    count: u32,
    gender: Option<String>,
    nat: Option<String>,
    mode: Option<String>,
) -> Result<Value, String> {
    let mode_str = mode.unwrap_or_else(|| "auto".to_string());
    let nat_str = nat.clone().unwrap_or_default();
    let is_vn = nat_str.to_lowercase().contains("vn");

    // If local mode requested or Vietnam requested (better offline names)
    if mode_str == "local" || (is_vn && mode_str != "api") {
        let mut data = generator::generate_local_users(
            count,
            gender.as_deref(),
            nat.as_deref(),
        );
        if let Some(obj) = data.as_object_mut() {
            obj.insert("source".to_string(), json!("local-rust-offline"));
            obj.insert("offlineFallback".to_string(), json!(false));
        }
        return Ok(data);
    }

    // Attempt online API fetch
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let mut url = format!("https://randomuser.me/api/?results={}", count);
    if let Some(g) = &gender {
        if !g.is_empty() {
            url.push_str(&format!("&gender={}", g));
        }
    }
    if let Some(n) = &nat {
        if !n.is_empty() && n != "all" {
            url.push_str(&format!("&nat={}", n));
        }
    }

    match client.get(&url).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                if let Ok(mut json_data) = resp.json::<Value>().await {
                    if let Some(obj) = json_data.as_object_mut() {
                        obj.insert("source".to_string(), json!("randomuser.me"));
                        obj.insert("offlineFallback".to_string(), json!(false));
                    }
                    return Ok(json_data);
                }
            }
            // If response not ok, fallback
            let mut fallback = generator::generate_local_users(count, gender.as_deref(), nat.as_deref());
            if let Some(obj) = fallback.as_object_mut() {
                obj.insert("source".to_string(), json!("local-rust-fallback"));
                obj.insert("offlineFallback".to_string(), json!(true));
            }
            Ok(fallback)
        }
        Err(err) => {
            eprintln!("[Tauri API Warning] Request failed: {}. Using offline generator.", err);
            let mut fallback = generator::generate_local_users(count, gender.as_deref(), nat.as_deref());
            if let Some(obj) = fallback.as_object_mut() {
                obj.insert("source".to_string(), json!("local-rust-fallback"));
                obj.insert("offlineFallback".to_string(), json!(true));
                obj.insert("originalError".to_string(), json!(err.to_string()));
            }
            Ok(fallback)
        }
    }
}

#[tauri::command]
async fn save_file_dialog(default_name: String, content: String, extension: String) -> Result<Option<String>, String> {
    let filter_name = match extension.as_str() {
        "csv" => "CSV Files (*.csv)",
        "json" => "JSON Files (*.json)",
        _ => "All Files (*.*)"
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_users,
            save_file_dialog
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
