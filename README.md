<div align="center">
  <img src="https://tuquet.github.io/icons/random-user-generator.svg" width="76" height="76" alt="Random User Generator Logo" />
  <h1>Random User Generator</h1>
  <p><strong>Ultra-Fast Native Desktop &amp; Offline Mock User Suite in Tauri v2 &amp; Vue 3</strong></p>

  <p>
    <a href="https://v2.tauri.app/"><img src="https://img.shields.io/badge/Tauri-v2-blue.svg?logo=tauri" alt="Tauri v2" /></a>
    <a href="https://vuejs.org/"><img src="https://img.shields.io/badge/Vue-3.5-emerald.svg?logo=vuedotjs" alt="Vue 3" /></a>
    <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-Core-orange.svg?logo=rust" alt="Rust" /></a>
    <a href="https://github.com/tuquet/lib"><img src="https://img.shields.io/badge/Design%20System-Tuquet%20Lib%20UI-cyan.svg" alt="Tuquet Lib UI" /></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License" /></a>
  </p>
</div>

---

> Ứng dụng Desktop Native siêu nhẹ, sinh dữ liệu hồ sơ người dùng ảo (Họ tên, Email, Số điện thoại, Địa chỉ, Avatar, Tài khoản/Mật khẩu) phục vụ kiểm thử hệ thống, seeding database và mock API. Tích hợp trực tiếp **Vue 3**, **Tuquet Lib UI Design System**, và **Rust Offline Engine** vận hành ngay cả khi ngắt kết nối mạng.

---

## ✨ Điểm Nổi Bật

* 🪶 **Siêu Nhẹ & Tối Ưu RAM**: Tiêu thụ chỉ ~**30MB RAM** với nhân Tauri v2 (nhẹ hơn 10x so với giải pháp Electron).
* 🎨 **Chuẩn Hệ Sinh Thái Tuquet**: Giao diện xây dựng trên nền tảng **Vue 3 + TypeScript + Tailwind CSS** và các nguyên lý component từ `@tuquet/lib` (`vue-ui`, `vue-table`).
* 📊 **Chế Độ Xem Đa Năng**:
  * **Card View**: Lưới thẻ trực quan với avatar, gắn thẻ quốc tịch, thông tin đăng nhập và copy 1-click.
  * **Data Table View**: Bảng dữ liệu mật độ cao (High-Density) có tìm kiếm lọc tức thì, sắp xếp cột và phân trang.
* ⚡ **Động Cơ Kép (Hybrid Engine)**:
  * Trực tuyến: Đồng bộ thời gian thực từ `randomuser.me`.
  * Ngoại tuyến: Tự động fallback sang **Rust Offline Generator** (hỗ trợ đầy đủ bộ từ điển tên tiếng Việt và các quốc gia).
* 💾 **Xuất Dữ Liệu Native**: Xuất file CSV và JSON trực tiếp qua hộp thoại lưu file của hệ điều hành.

---

## 📁 Cấu Trúc Dự Án

```text
random-user-generator/
├── src-tauri/                 # RUST CORE (Tauri Backend)
│   ├── Cargo.toml             # Crate: tauri v2, reqwest, rfd, serde, tokio
│   ├── tauri.conf.json        # Cấu hình cửa sổ, CSP bảo mật, dist mapping
│   └── src/
│       ├── main.rs            # Entrypoint native desktop
│       ├── lib.rs             # Tauri IPC Handlers (fetch_users, save_file_dialog)
│       └── generator.rs       # Bộ sinh dữ liệu Rust Offline
├── src/                       # VUE 3 FRONTEND
│   ├── components/            # Vue Components
│   │   ├── ui/                # Tuquet UI Primitives (Button, Badge, Card, Input)
│   │   ├── HeaderBar.vue      # Thanh điều hướng & trạng thái
│   │   ├── FilterControls.vue # Bộ lọc số lượng, giới tính, quốc tịch
│   │   ├── UserCardGrid.vue   # Chế độ hiển thị thẻ
│   │   ├── UserDataTable.vue  # Chế độ bảng dữ liệu (Data Table)
│   │   └── EventLogDrawer.vue # Bảng nhật ký sự kiện hệ thống
│   ├── services/
│   │   └── tauri.ts           # Cầu nối IPC an toàn (Tauri + Web Fallback)
│   ├── types/
│   │   └── user.ts            # TypeScript interfaces
│   ├── App.vue                # Giao diện điều phối chính
│   └── main.ts                # Bootstrap ứng dụng Vue
├── vite.config.ts             # Vite bundler
├── tailwind.config.ts         # Bảng màu chuẩn Tuquet
└── package.json               # Quản lý bởi pnpm
```

---

## 🚀 Hướng Dẫn Phát Triển

### 1. Cài đặt dependencies (pnpm)
```bash
pnpm install
```

### 2. Khởi chạy chế độ Web Dev
```bash
pnpm dev
# Mở tại http://localhost:5173
```

### 3. Khởi chạy chế độ Desktop (Tauri Dev)
```bash
pnpm desktop
```

### 4. Đóng gói ứng dụng Desktop (.EXE)
```bash
pnpm build:desktop
# File cài đặt hoặc portable exe được sinh ra tại src-tauri/target/release/
```

---

## 📜 Giấy Phép

Phát hành theo giấy phép **MIT**. Bản quyền thuộc về **Tuquet Team**.
