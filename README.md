# shared-models
Publish mẫu

--------- Các bước thực hiện:

cd shared-models

# Bump version nếu cần
cargo package         # Tạo file .crate

git add .
git commit -m "init shared-models"
git tag v0.1.0
git push origin main --tags

Sau đó vào GitHub:
→ Releases → Create release → tag v0.1.0 → upload file target/package/shared-models-0.1.0.crate.

6. Mỗi lần cập nhật product.rs?
	1.	Sửa shared-models/src/models/product.rs.
	2.	Bump version trong Cargo.toml, ví dụ 0.1.1.
	3.	Tạo tag mới + release mới:

    git tag v0.1.1
    git push origin main --tags

=> GitHub Action sẽ tự upload .crate → các service dùng được version mới ngay.


name: Publish crate
on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write  # 👈 Cho phép ghi release (bắt buộc cho action-gh-release)

jobs:
  build-and-release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
          
      - name: Build crate (optional but useful)
        run: cargo build

      - name: Package crate
        run: cargo package

      - name: Upload .crate file to GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            target/package/*.crate