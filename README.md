# 🦀 Random User Generator - Tauri Desktop Suite

Ứng dụng **Desktop Native (Windows)** siêu nhẹ, khởi động tức thì, tiêu thụ cực ít tài nguyên để sinh dữ liệu người dùng ảo (Họ tên, Email, Số điện thoại, Địa chỉ, Avatar, Tài khoản/Mật khẩu) phục vụ test web, tạo mock data, seeding database.

Ứng dụng được xây dựng trên nền tảng **Tauri v2 + Rust Core** kết hợp giao diện **Tailwind CSS**, thay thế hoàn toàn việc phải mở trình duyệt web localhost cồng kềnh.

---

## ⚡ Ưu điểm vượt trội của bản Tauri Desktop

* 🪶 **Siêu nhẹ & Tiết kiệm RAM**: Chiếm chỉ ~**30MB RAM** (so với 300–500MB của Electron/Chrome).
* 🖥️ **Cửa sổ phần mềm Native**: Hoạt động như một app Windows thực thụ (`.exe`), không cần mở trình duyệt, không lo xung đột Port.
* 💾 **Hộp thoại lưu file Windows Native**: Khi bấm xuất CSV/JSON, xuất hiện hộp thoại **Save File As...** của Windows để bạn chọn vị trí lưu file trực tiếp.
* 🛡️ **Bảo mật & Không CORS**: Rust Core gọi API trực tiếp, không bị chặn CORS hay giới hạn của trình duyệt.
* 🔌 **Offline Engine bằng Rust**: Tích hợp sẵn bộ sinh dữ liệu ngẫu nhiên bằng Rust nội bộ (có tên & địa chỉ Việt Nam + Quốc tế), hoạt động mượt mà ngay cả khi ngắt kết nối mạng.

---

## 🚀 Cách chạy ứng dụng

### 1. Khởi động chế độ Desktop (Nhanh nhất)
Nhấp đúp chuột vào file:
👉 **`run-desktop.bat`** *(hoặc chạy `npm run desktop` trong terminal)*.
> Cửa sổ phần mềm sẽ xuất hiện trực tiếp trên màn hình.

---

### 2. Đóng gói thành file `.EXE` độc lập (Portable / Installer)
Nhấp đúp chuột vào file:
👉 **`build-exe.bat`** *(hoặc chạy `npm run build:exe`)*.
> File `.exe` cài đặt sẽ được tạo ra tại: `src-tauri\target\release\`. Bạn có thể copy file này sang máy khác sử dụng trực tiếp mà không cần cài thêm Node.js hay Rust.

---

### 3. Chế độ Web Server (Tùy chọn phụ)
Nếu vẫn muốn chạy dưới dạng Web Localhost trên trình duyệt:
* Chạy file **`start.bat`** (Mở tại `http://localhost:3500`).

---

## 📁 Cấu trúc dự án (Tauri Architecture)

```text
random-user-generator/
├── src-tauri/                 # RUST CORE (Tauri Backend)
│   ├── Cargo.toml             # Khai báo crate: tauri, reqwest, rfd, serde
│   ├── tauri.conf.json        # Cấu hình cửa sổ, quyền hạn (capabilities)
│   ├── build.rs               # Script build của Tauri
│   ├── icons/                 # Bộ icon chuẩn đa nền tảng
│   └── src/
│       ├── main.rs            # Entry point Windows subsystem
│       ├── lib.rs             # Tauri Commands (fetch_users, save_file_dialog)
│       └── generator.rs       # Rust Offline Mock Data Generator (VN & Quốc tế)
├── public/                    # FRONTEND (Giao diện người dùng)
│   ├── index.html             # Giao diện Cards / Table / JSON (Tailwind)
│   ├── app.js                 # Cầu nối Tauri IPC (`window.__TAURI__.core`)
│   └── style.css              # Custom styling & hiệu ứng
├── run-desktop.bat            # 1-Click mở ứng dụng Desktop (Tauri dev)
├── build-exe.bat              # 1-Click build file .exe độc lập
├── start.bat                  # Chạy dạng web server localhost
├── package.json               # Cấu hình npm & tauri scripts
└── README.md                  # Tài liệu hướng dẫn
```

---

## 🛠️ Danh sách Tauri Commands (Rust IPC)

Frontend giao tiếp với Rust Core qua các lệnh:
* `fetch_users(count, gender, nat, mode)`: Lấy dữ liệu từ `randomuser.me` hoặc tự động sinh offline bằng Rust nếu mất mạng.
* `save_file_dialog(default_name, content, extension)`: Mở hộp thoại Windows Explorer để chọn nơi lưu file `.csv` hoặc `.json`.
