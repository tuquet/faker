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
    mode: Option<String>,
    avatar_style: Option<String>,
) -> Result<Value, String> {
    let mode_str = mode.as_deref().unwrap_or("local");
    let style = avatar_style.as_deref().unwrap_or("real");

    if mode_str == "api" {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(8))
            .build()
            .map_err(|e| e.to_string())?;

        let mut url = format!("https://randomuser.me/api/?results={}", count);
        if let Some(ref g) = gender {
            if g != "all" {
                url.push_str(&format!("&gender={}", g));
            }
        }
        if let Some(ref n) = nat {
            if n != "all" && n != "vn" {
                url.push_str(&format!("&nat={}", n));
            }
        }

        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                if let Ok(mut data) = resp.json::<Value>().await {
                    if style == "svg" {
                        if let Some(results) = data["results"].as_array_mut() {
                            for u in results.iter_mut() {
                                let uuid = u["login"]["uuid"].as_str().unwrap_or("seed");
                                let svg_url = format!(
                                    "https://api.dicebear.com/7.x/avataaars/svg?seed={}&backgroundColor=b6e3f4,c0aede,d1d4f9,ffd5dc,ffdfbf",
                                    uuid
                                );
                                u["picture"]["large"] = json!(svg_url);
                                u["picture"]["medium"] = json!(svg_url);
                                u["picture"]["thumbnail"] = json!(svg_url);
                            }
                        }
                    }
                    if let Some(obj) = data.as_object_mut() {
                        obj.insert("source".to_string(), json!("RandomUser.me Web API"));
                        obj.insert("offlineFallback".to_string(), json!(false));
                    }
                    return Ok(data);
                }
            }
        }
    }

    // Default: Offline Rust Generator
    let mut data = generator::generate_local_users(count, gender.as_deref(), nat.as_deref(), Some(style));
    if let Some(obj) = data.as_object_mut() {
        let source_name = if mode_str == "api" {
            "Offline Rust Core (API Fallback)"
        } else {
            "Offline Rust Core (0ms)"
        };
        obj.insert("source".to_string(), json!(source_name));
        obj.insert("offlineFallback".to_string(), json!(mode_str == "api"));
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
async fn download_single_avatar(url: String, default_name: String) -> Result<Option<String>, String> {
    let is_svg = url.contains(".svg") || url.contains("dicebear");
    let ext = if is_svg { "svg" } else { "jpg" };
    let filename = format!("{}.{}", default_name, ext);
    let filter_name = if is_svg { "SVG Vector Image (*.svg)" } else { "JPEG Image (*.jpg)" };

    let path = tauri::async_runtime::spawn_blocking(move || {
        rfd::FileDialog::new()
            .set_file_name(&filename)
            .add_filter(filter_name, &[ext])
            .save_file()
    })
    .await
    .map_err(|e| e.to_string())?;

    if let Some(file_path) = path {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
        let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
        fs::write(&file_path, bytes).map_err(|e| e.to_string())?;
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

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        let mut csv_rows = vec![
            "STT,Ho Ten,Chuc Danh,Gioi Tinh,Quoc Tich,Email,So Dien Thoai,Avatar File,Avatar URL,Dia Chi,Thanh Pho,Quoc Gia,CCCD/SSN,Username,Password".to_string()
        ];

        for (i, u) in users.iter().enumerate() {
            let username = u["login"]["username"].as_str().unwrap_or("user");
            let img_url = u["picture"]["large"].as_str().unwrap_or("");
            let is_svg = img_url.contains(".svg") || img_url.contains("dicebear");
            let ext = if is_svg { "svg" } else { "jpg" };
            let filename = format!("{}.{}", username, ext);
            let file_path = avatars_dir.join(&filename);

            // Fetch actual real image bytes via reqwest
            let mut downloaded = false;
            if img_url.starts_with("http") {
                if let Ok(resp) = client.get(img_url).send().await {
                    if resp.status().is_success() {
                        if let Ok(bytes) = resp.bytes().await {
                            if fs::write(&file_path, bytes).is_ok() {
                                downloaded = true;
                            }
                        }
                    }
                }
            }

            // Fallback if network failed or data URI available
            if !downloaded {
                if let Some(data_uri) = u["picture"]["data_uri"].as_str() {
                    if let Some(svg_content) = data_uri.strip_prefix("data:image/svg+xml;utf8,") {
                        let _ = fs::write(&file_path, svg_content);
                    }
                }
            }

            let full_name = format!("{} {}", u["name"]["first"].as_str().unwrap_or(""), u["name"]["last"].as_str().unwrap_or(""));
            let job = u["job"].as_str().unwrap_or("");
            let gender = u["gender"].as_str().unwrap_or("");
            let nat = u["nat"].as_str().unwrap_or("");
            let email = u["email"].as_str().unwrap_or("");
            let phone = u["phone"].as_str().unwrap_or("");
            let avatar_file = format!("./avatars/{}", filename);
            let avatar_url = img_url;
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
            download_single_avatar,
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
        let res = fetch_users(5, Some("male".to_string()), Some("VN".to_string()), None, None).await;
        assert!(res.is_ok());
        let val = res.unwrap();
        let list = val["results"].as_array().expect("results should be an array");
        assert_eq!(list.len(), 5);
    }
}
