# 👤 Random User Generator - Local Suite

Ứng dụng Web App cục bộ (Localhost) và API sinh dữ liệu người dùng ảo (Họ tên, Email, Số điện thoại, Địa chỉ, Avatar, Tài khoản/Mật khẩu) phục vụ test web, tạo mock data, seeding database hoặc làm tài khoản ảo.

Dự án tích hợp trực tiếp **API RandomUser.me** (Online) và **Local Faker Generator** (Offline), đảm bảo hoạt động liên tục ngay cả khi mất mạng internet hoặc bị chặn IP.

---

## 🌟 Tính năng nổi bật

1. **Giao diện hiện đại & tiện dụng (Tailwind CSS)**:
   - Chế độ **Thẻ Profile (Cards)**: Hiển thị avatar, đầy đủ thông tin chi tiết.
   - Chế độ **Bảng dữ liệu (Table)**: Xem hàng loạt, sắp xếp, lọc nhanh.
   - Chế độ **Raw JSON**: Copy trực tiếp payload cho Developer.
2. **Sao chép 1-Click**:
   - Nhấp vào bất kỳ trường nào (Email, Mật khẩu, SĐT, Địa chỉ, JSON) là tự động copy vào Clipboard.
3. **Bộ lọc đa dạng**:
   - Số lượng (1 - 500 profiles).
   - Giới tính (Nam / Nữ / Ngẫu nhiên).
   - Quốc gia / Ngôn ngữ (Việt Nam `VN`, Mỹ `US`, Anh `GB`, Pháp `FR`, Đức `DE`, Nhật `JP`, v.v.).
4. **Offline Fallback (Hoạt động không cần mạng)**:
   - Nếu không có internet hoặc RandomUser.me phản hồi chậm, hệ thống tự động chuyển sang bộ sinh dữ liệu cục bộ bằng `@faker-js/faker`.
5. **Xuất file nhanh**:
   - Xuất ra file **Excel / CSV** chuẩn UTF-8 (mở bằng Microsoft Excel không bị lỗi font tiếng Việt).
   - Xuất ra file **JSON**.
6. **Tích hợp API cục bộ cho Developer**:
   - Bạn có thể gọi API trực tiếp từ backend hoặc postman: `GET http://localhost:3000/api/users?results=20&nat=vn`.

---

## 🚀 Cách chạy ứng dụng

### Cách 1: Nhanh nhất trên Windows (1-Click)
* Nhấp đúp chuột vào file **`start.bat`**.
* Trình duyệt sẽ tự động mở trang web tại địa chỉ: `http://localhost:3000`.

---

### Cách 2: Chạy qua dòng lệnh (Terminal / PowerShell)

```bash
# 1. Di chuyển vào thư mục dự án
cd D:\Repository\tuquet\random-user-generator

# 2. Cài đặt thư viện (chỉ cần chạy lần đầu)
npm install

# 3. Khởi động ứng dụng
npm start
```

*Mở trình duyệt truy cập: **`http://localhost:3000`***

---

## 📡 Tài liệu API Endpoint (Local REST API)

Nếu bạn muốn tích hợp vào tool hoặc web khác, bạn có thể gọi thẳng vào Localhost:

### 1. Lấy danh sách Users ngẫu nhiên
* **URL**: `GET /api/users`
* **Tham số (Query parameters)**:
  * `results` (number): Số lượng users cần sinh (mặc định: `10`, tối đa `5000`).
  * `gender` (string): `male` hoặc `female`.
  * `nat` (string): Mã quốc gia (`vn`, `us`, `gb`, `fr`, `de`, `jp`...).
  * `mode` (string):
    * `auto` (mặc định): Thử API trước, nếu lỗi chuyển offline.
    * `local`: Tạo offline ngay lập tức siêu tốc.
    * `api`: Bắt buộc dùng API randomuser.me.

**Ví dụ cURL:**
```bash
curl "http://localhost:3000/api/users?results=5&nat=vn"
```

### 2. Xuất dữ liệu ra file CSV
* **URL**: `POST /api/export/csv`
* **Body (JSON)**:
  ```json
  {
    "users": [ ... ]
  }
  ```
* **Trả về**: File CSV đính kèm tải xuống (`attachment`).

---

## 📁 Cấu trúc thư mục

```text
random-user-generator/
├── public/                 # Giao diện người dùng Web App
│   ├── index.html          # File HTML chính (Tailwind CSS, Font Awesome)
│   ├── app.js              # Xử lý tương tác, filter, copy, xuất file
│   └── style.css           # Hiệu ứng và tinh chỉnh CSS
├── src/                    # Backend Node.js
│   ├── server.js           # Express Server & REST API endpoints
│   ├── localGenerator.js   # Module sinh dữ liệu offline (Faker)
│   └── exporter.js         # Tiện ích chuyển đổi sang CSV/Excel UTF-8
├── start.bat               # File khởi động 1-click cho Windows
├── install.bat             # File cài đặt thư viện 1-click
├── package.json            # Cấu hình dự án và dependencies
└── README.md               # Tài liệu hướng dẫn
```
