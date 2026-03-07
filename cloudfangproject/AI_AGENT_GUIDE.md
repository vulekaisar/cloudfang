# Hướng Dẫn Môi Trường Phát Triển AI Agent với Rust

Môi trường này đã được thiết lập để bạn bắt đầu phát triển các ứng dụng Agentic AI hiệu năng cao bằng ngôn ngữ Rust.

## 1. Thành phần đã cài đặt

- **Ngôn ngữ**: Rust (phiên bản 1.92.0)
- **Thư viện chính (`Cargo.toml`)**:
    - `async-openai`: Client mạnh mẽ để kết nối với OpenAI (GPT-4, GPT-3.5).
    - `tokio`: Runtime bất đồng bộ chuẩn công nghiệp của Rust.
    - `serde` & `serde_json`: Xử lý dữ liệu JSON.
    - `dotenvy`: Quản lý biến môi trường bảo mật.

## 2. Bắt đầu nhanh

### Bước 1: Cấu hình API Key

Bạn cần có một API Key từ OpenAI (hoặc các nhà cung cấp tương thích).
1. Đổi tên file `.env.example` thành `.env`:
   ```bash
   mv .env.example .env
   ```
2. Mở file `.env` và dán API key của bạn vào:
   ```env
   OPENAI_API_KEY=sk-proj-xxxxxxxxx
   ```

### Bước 2: Chạy thử Agent cơ bản

Mã nguồn mẫu nằm trong `src/main.rs`. Đây là một CLI Chatbot đơn giản.

Chạy lệnh sau để biên dịch và chạy:

```bash
cargo run
```

Sau khi biên dịch xong, bạn có thể chat với Agent ngay trong terminal.

## 3. Phát triển nâng cao (Agentic AI)

Để phát triển các Agent thông minh hơn (biết dùng công cụ, xử lý bộ nhớ), bạn có hai hướng đi chính trong Rust:

### Cách 1: Tự xây dựng (Build from scratch)
Sử dụng `async-openai` (đã cài sẵn) và tính năng **Function Calling**.
- Bạn định nghĩa các hàm (Tools) trong Rust.
- Mô tả chúng cho LLM thông qua Schema.
- Xử lý vòng lặp: LLM trả về yêu cầu gọi hàm -> Rust thực thi hàm -> Gửi kết quả lại cho LLM.

### Cách 2: Sử dụng Framework chuyên dụng (Khuyên dùng: Rig)
**Rig** (`rig-core`) là một thư viện Rust mới và mạnh mẽ để xây dựng Agent, tương tự như LangChain hay CrewAI bên Python nhưng tối ưu cho Rust.

Để sử dụng Rig, hãy mở `Cargo.toml` và bỏ comment dòng:
```toml
# rig-core = "0.28.0" (hoặc phiên bản mới nhất bạn tìm thấy)
```
Hoặc chạy:
```bash
cargo add rig-core
```

_Rig cung cấp các abstraction cao cấp như `Agent`, `VectorStore`, và `Tool` giúp code gọn gàng hơn._

## 4. Tài nguyên tham khảo

- **Rust Book (Tiếng Việt)**: Tài liệu học Rust cơ bản.
- **Async-OpenAI Document**: [https://docs.rs/async-openai](https://docs.rs/async-openai)
- **Rig Framework**: [https://github.com/0xPlaygrounds/rig](https://github.com/0xPlaygrounds/rig) - Nơi tốt nhất để xem các ví dụ về Agent phức tạp.

---
*Chúc bạn xây dựng được những AI Agent mạnh mẽ với Rust!*
