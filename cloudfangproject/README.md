# Hướng Dẫn Toàn Tập: Môi Trường Rust & AI Agent

Chào mừng bạn đến với môi trường phát triển hiệu năng cao dành cho **Rust** và **Agentic AI**.  
Tài liệu này bao gồm hướng dẫn cho cả người mới bắt đầu với Rust và các bước để xây dựng một AI Agent hoàn chỉnh.

## 1. Tổng Quan Hệ Thống

Môi trường này đã được cấu hình sẵn với các công cụ mạnh mẽ nhất hiện nay:
- **Rust**: 1.92.0 (Ngôn ngữ lập trình hệ thống an toàn và nhanh chóng).
- **AI Stack**:
  - `async-openai`: Client kết nối LLM (GPT-4, Claude...) mạnh mẽ, ổn định.
  - `tokio`: Runtime xử lý bất đồng bộ (async) chuẩn công nghiệp.
  - `rig-core` (Tùy chọn): Framework chuyên dụng để xây dựng Agent phức tạp (tương tự LangChain/CrewAI).

## 2. Cài Đặt & Cấu Hình

### Bước 1: Kiểm tra công cụ
Đảm bảo bạn đã cài đặt Rust. Kiểm tra bằng lệnh:
```bash
rustc --version
cargo --version
```
*Kết quả mong đợi: Phiên bản ~1.92.0*

### Bước 2: Cấu hình API Key
Để Agent hoạt động, bạn cần cung cấp "bộ não" cho nó (API Key).
1. Đổi tên file cấu hình mẫu:
   ```bash
   mv .env.example .env
   ```
2. Chỉnh sửa file `.env` và điền API Key của bạn:
   ```env
   OPENAI_API_KEY=sk-proj-xxxxxxxxxxxxxxxxxxxx
   ```

## 3. Chạy Thử AI Agent (Workthrough)

Dự án đi kèm với một **CLI Chatbot** mẫu nằm trong `src/main.rs`.

### Cách chạy:
Mở terminal tại thư mục dự án và chạy:
```bash
cargo run
```
*(Lần đầu chạy sẽ tốn chút thời gian để tải và biên dịch thư viện)*

### Tương tác mẫu:
```text
🤖 Rust AI Agent initialized (Model: gpt-3.5-turbo)
Type 'quit' or 'exit' to stop.
--------------------------------------------------
> Xin chào, bạn là ai?
Thinking...
🤖: Xin chào! Tôi là một trợ lý ảo AI được viết bằng Rust. Tôi có thể giúp gì cho bạn?
--------------------------------------------------
```

## 4. Cấu Trúc Dự Án

- **`src/main.rs`**: Mã nguồn chính của Agent. Nơi bạn định nghĩa luồng logic.
- **`Cargo.toml`**: Quản lý thư viện (dependencies).
- **`.env`**: Nơi chứa mật khẩu/API Key (không bao giờ được commit lên Git!).
- **`target/`**: Thư mục chứa file thực thi sau khi biên dịch.

## 5. Phát Triển Nâng Cao (Advanced AI)

Để xây dựng Agent "thông minh" hơn (biết dùng Google Search, tra cứu Database, đọc file...), bạn nên sử dụng **Rig Framework**.

### Kích hoạt Rig:
Mở file `Cargo.toml` và bỏ comment dòng sau (hoặc thêm vào):
```toml
rig-core = "0.6.0" # Hoặc phiên bản mới nhất
```

### Ví dụ ý tưởng với Rig:
Bạn có thể tạo một Agent có khả năng tự suy luận:
```rust
// Pseudo-code ví dụ
let agent = AgentBuilder::new(model)
    .attach_tool(GoogleSearchTool::new())
    .attach_tool(CalculatorTool::new())
    .build();

agent.prompt("Tìm giá Bitcoin hiện tại và tính xem tôi mua được bao nhiêu với $100?");
```

## 6. Các Lệnh Rust Thường Dùng

- `cargo check`: Kiểm tra lỗi code nhanh (không biên dịch ra file chạy).
- `cargo build --release`: Biên dịch bản chính thức (tối ưu hóa tốc độ tối đa).
- `cargo fmt`: Tự động format code cho đẹp chuẩn Rust.

---
*Created by Antigravity Team for High-Performance AI Development.*
