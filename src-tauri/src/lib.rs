mod generator;

use serde_json::{json, Value};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::Duration;

pub fn log_to_file(level: &str, msg: &str) {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let line = format!("[{}] [{}] {}\n", now, level, msg);
    
    // Log to app.log next to the executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(dir) = exe_path.parent() {
            let log_path = dir.join("app.log");
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
                let _ = file.write_all(line.as_bytes());
            }
        }
    }
    
    // Also log to current directory app.log
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
    mode: Option<String>,
) -> Result<Value, String> {
    log_to_file("INFO", &format!("fetch_users called: count={}, gender={:?}, nat={:?}, mode={:?}", count, gender, nat, mode));
    let mode_str = mode.unwrap_or_else(|| "auto".to_string());
    let nat_str = nat.clone().unwrap_or_default();
    let is_vn = nat_str.to_lowercase().contains("vn");

    // If local mode requested or Vietnam requested (better offline names)
    if mode_str == "local" || (is_vn && mode_str != "api") {
        log_to_file("INFO", "Using local offline Rust generator (local/VN mode)");
        let mut data = generator::generate_local_users(
            count,
            gender.as_deref(),
            nat.as_deref(),
        );
        if let Some(obj) = data.as_object_mut() {
            obj.insert("source".to_string(), json!("local-rust-offline"));
            obj.insert("offlineFallback".to_string(), json!(false));
        }
        log_to_file("SUCCESS", &format!("Local Rust generator produced {} profiles", count));
        return Ok(data);
    }

    // Attempt online API fetch
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(4))
        .build() {
            Ok(c) => c,
            Err(e) => {
                log_to_file("WARN", &format!("Client builder failed: {}. Falling back to offline.", e));
                let mut fallback = generator::generate_local_users(count, gender.as_deref(), nat.as_deref());
                if let Some(obj) = fallback.as_object_mut() {
                    obj.insert("source".to_string(), json!("local-rust-fallback"));
                    obj.insert("offlineFallback".to_string(), json!(true));
                }
                return Ok(fallback);
            }
        };

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

    log_to_file("INFO", &format!("Requesting online API: {}", url));

    match client.get(&url).send().await {
        Ok(resp) => {
            let status = resp.status();
            log_to_file("INFO", &format!("API responded with status: {}", status));
            if status.is_success() {
                if let Ok(mut json_data) = resp.json::<Value>().await {
                    if let Some(obj) = json_data.as_object_mut() {
                        obj.insert("source".to_string(), json!("randomuser.me"));
                        obj.insert("offlineFallback".to_string(), json!(false));
                    }
                    log_to_file("SUCCESS", "API fetch succeeded and parsed successfully");
                    return Ok(json_data);
                }
            }
            log_to_file("WARN", "API response was not successful, falling back to local generator");
            let mut fallback = generator::generate_local_users(count, gender.as_deref(), nat.as_deref());
            if let Some(obj) = fallback.as_object_mut() {
                obj.insert("source".to_string(), json!("local-rust-fallback"));
                obj.insert("offlineFallback".to_string(), json!(true));
            }
            Ok(fallback)
        }
        Err(err) => {
            log_to_file("WARN", &format!("API request failed: {}. Using offline generator.", err));
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
    log_to_file("INFO", &format!("save_file_dialog called for: {}", default_name));
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
        let res_path = file_path.to_string_lossy().to_string();
        log_to_file("SUCCESS", &format!("Saved file successfully to: {}", res_path));
        return Ok(Some(res_path));
    }

    log_to_file("INFO", "User cancelled save file dialog");
    Ok(None)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    log_to_file("STARTUP", "Random User Generator Tauri application started");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_users,
            save_file_dialog,
            log_client_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_users_local_mode() {
        let res = fetch_users(5, Some("male".to_string()), Some("VN".to_string()), Some("local".to_string())).await;
        assert!(res.is_ok(), "fetch_users local mode should return Ok");
        let val = res.unwrap();
        let list = val["results"].as_array().expect("results should be an array");
        assert_eq!(list.len(), 5);
        assert_eq!(val["source"], "local-rust-offline");
    }

    #[tokio::test]
    async fn test_fetch_users_auto_fallback() {
        // Test auto mode returns users whether online or fallback
        let res = fetch_users(3, None, None, Some("auto".to_string())).await;
        assert!(res.is_ok(), "fetch_users auto mode should return Ok");
        let val = res.unwrap();
        let list = val["results"].as_array().expect("results should be an array");
        assert_eq!(list.len(), 3);
    }
}
