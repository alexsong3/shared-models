# shared-models
Publish mẫu

-- Bước thực hiện:

cd shared-models

# Bump version nếu cần
cargo package         # Tạo file .crate

git add .
git commit -m "init shared-models"
git tag v0.1.0
git push origin main --tags

Sau đó vào GitHub:
→ Releases → Create release → tag v0.1.0 → upload file target/package/shared-models-0.1.0.crate.