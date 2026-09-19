# Hướng dẫn cài đặt Rust và Cargo cho dự án `learn-rust`

Dự án **learn-rust** (phiên bản 0.1.0, Rust edition 2024) sử dụng thư viện `dotenvy` để quản lý biến môi trường. Dưới đây là hướng dẫn chi tiết cách thiết lập môi trường Rust, Cargo trên các hệ điều hành và cách làm việc với dự án này.

### 🍎 macOS (Sử dụng Homebrew)

Bạn có thể chọn 1 trong 2 cách cài đặt qua Homebrew bên dưới:

#### Cách 1: Cài đặt qua `rustup-init` (Khuyên dùng để dễ quản lý version)
   ```bash
   # 1. Cài đặt rustup-init qua brew
   brew install rustup-init

   # 2. Chạy rustup-init để thiết lập toolchain
   rustup-init

   # 3. Cập nhật biến môi trường
   source $HOME/.cargo/env
   ```

#### Cách 2: Cài đặt trực tiếp gói rust
   ```bash
   brew install rust
   ```

### 🐧 Linux (Fedora)
Fedora sử dụng `dnf` làm trình quản lý gói. Trước khi cài Rust, bạn nên cài đặt bộ biên dịch C cơ bản để Rust có thể link các thư viện.
1. Mở Terminal và chạy:
   ```bash
   sudo dnf install gcc -y
   ```
2. Cài đặt Rust qua `rustup`:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Khởi động lại terminal hoặc cập nhật biến môi trường:
   ```bash
   source $HOME/.cargo/env
   ```

### 🪟 Windows
1. Truy cập trang chủ [https://rustup.rs/](https://rustup.rs/) và tải tệp **`rustup-init.exe`** (hoặc bản 64-bit tương ứng).
2. Khi chạy tệp cài đặt, Rust sẽ yêu cầu bạn cài đặt **Visual Studio C++ Build tools** (nếu máy tính chưa có). Nhấn vào link tải build tools mà installer cung cấp, chọn gói *"Desktop development with C++"* và cài đặt.
3. Sau khi có C++ Build tools, quay lại cửa sổ `rustup-init`, nhấn `1` và `Enter` để tiến hành cài đặt mặc định.

**Kiểm tra cài đặt (Dành cho mọi HĐH):**
Sau khi cài đặt xong, hãy xác minh lại bằng hai lệnh sau:
```bash
rustc --version
cargo --version
```

---

## 2. Quản lý dự án `learn-rust`

File `Cargo.toml` của bạn đã khai báo rõ thông tin dự án và dependency:
```toml
[package]
name = "learn-rust"
version = "0.1.0"
edition = "2024"

[dependencies]
dotenvy = "0.15.7"
```

### 🎯 Hướng dẫn tạo thư mục `target`
Thư mục `target` là nơi Cargo lưu trữ toàn bộ các tệp tin đã được biên dịch (compiled artifacts) như file thực thi `.exe`, mã máy trung gian, v.v.
Bạn **không cần và không nên tạo thư mục này bằng tay**. Để Cargo tự động tạo thư mục `target`, bạn chỉ cần chạy lệnh build tại thư mục chứa file `Cargo.toml`:

```bash
cargo build
```
*(Nếu muốn tạo file thực thi tối ưu để đem đi chạy thật, hãy dùng lệnh: `cargo build --release`)*

Sau khi chạy lệnh trên, Cargo sẽ tải `dotenvy`, biên dịch dự án và tự sinh ra thư mục `target` để lưu kết quả.

### 📦 Hướng dẫn tạo thư mục `vendor`
Thư mục `vendor` được sử dụng khi bạn muốn tải mã nguồn của toàn bộ thư viện bên thứ 3 (như `dotenvy`) về lưu trữ cục bộ trong dự án. Việc này cực kỳ hữu ích nếu bạn cần **build offline** (không có internet) hoặc lưu trữ cố định phiên bản các thư viện.

1. Chạy lệnh sau để Cargo kéo tất cả mã nguồn các dependencies về thư mục `vendor`:
   ```bash
   cargo vendor
   ```
   Lúc này thư mục `vendor/` đã được tạo ra.

   **suggest mới:**
   Nên thay đổi biến môi trường để cho máy đỡ nặng
   ```bash
   CARGO_HOME=.cargo_local cargo check
   ```

2. Để yêu cầu Cargo sử dụng các thư viện ở trong thư mục `vendor` (thay vì tải trực tiếp từ internet qua crates.io), bạn cần tạo thư mục `.cargo` và thêm file config:
   * **macOS / Linux:**
     ```bash
     mkdir -p .cargo
     nano .cargo/config.toml
     ```
   * **Windows (PowerShell):**
     ```powershell
     New-Item -ItemType Directory -Force -Path .cargo
     New-Item -ItemType File -Force -Path .cargo/config.toml
     ```

3. Dán đoạn nội dung sau vào file `.cargo/config.toml`:
   ```toml
   [source.crates-io]
   replace-with = "vendored-sources"

   [source.vendored-sources]
   directory = "vendor"
   ```

Bây giờ dự án của bạn đã sẵn sàng để phát triển và hoàn toàn có thể build offline với source code của `dotenvy` đã nằm an toàn trong thư mục `vendor`!