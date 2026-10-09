<div align="center">
  <img src="https://tuquet.com/icons/random-user-generator.svg" width="76" height="76" alt="Specter Faker Logo" />
  <h1>Specter Faker (`specter faker`)</h1>
  <p><strong>Enterprise-Grade Synthetic Persona & Test Identity Generator in Rust</strong></p>

  <p>
    <a href="https://specter.tuquet.com/faker/"><img src="https://img.shields.io/badge/Docs-VitePress%20Hub-blue.svg" alt="Documentation Hub" /></a>
    <a href="https://github.com/tuquet/scoop-bucket"><img src="https://img.shields.io/badge/Scoop-specter-brightgreen.svg" alt="Scoop" /></a>
    <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-Clap-orange.svg" alt="Rust" /></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License" /></a>
  </p>

  <p>
    <strong><a href="https://specter.tuquet.com/faker/">📖 Đọc toàn bộ tài liệu kỹ thuật tại Documentation Hub &rarr;</a></strong>
  </p>
</div>

---

## 📌 Tổng Quan (Overview)

**Specter Faker** là công cụ sinh dữ liệu danh tính nhân vật giả lập (synthetic personas) chuẩn xác cao phục vụ kiểm thử QA, điền form tự động và nuôi tài khoản. Viết bằng Rust thuần túy với độ trễ sub-millisecond (0ms network round-trip), Faker hỗ trợ đầy đủ thuật toán sinh số Căn cước Công dân (CCCD) Việt Nam hợp lệ, địa chỉ hành chính gắn liền mã bưu chính và email/mật khẩu entropy cao.

* **Lưu trữ SSOT (Pillar 5)**: Cấu hình schema và override domain pool lưu tại `~/.specter/faker/faker.json`.
* **Air-gapped & Offline**: Không phụ thuộc vào API bên ngoài, đảm bảo dữ liệu sinh ra tức thì và bảo mật tuyệt đối.

## ⚡ Sử Dụng Nhanh (Quickstart)

```bash
# Sinh 5 danh tính Việt Nam ngẫu nhiên kèm CCCD hợp lệ
specter faker generate --count 5 --locale vi

# Xuất dữ liệu nhân vật ra file JSON
specter faker generate -n 10 --json > personas.json
```

## 📚 Tài Liệu Kỹ Thuật Tập Trung (SSOT)

Toàn bộ thuật toán sinh CCCD 12 số, cấu trúc cây địa chỉ hành chính, quy chuẩn `faker.json` và bảng tham số CLI được bảo trì duy nhất tại Documentation Hub:

👉 **[https://specter.tuquet.com/faker/](https://specter.tuquet.com/faker/)**
