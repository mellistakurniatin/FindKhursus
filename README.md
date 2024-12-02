# FindKhursus
# Aplikasi Find Kursus

Aplikasi untuk mencari dan menjual kursus secara daring.

## Fitur
- **Penyedia Kursus:** Tambah kursus baru.
- **Pencari Kursus:** Jelajahi kursus berdasarkan kategori, lihat detail, dan beli kursus.

## Cara Menjalankan

### Backend
1. Masuk ke direktori `backend/`.
2. Bangun dan jalankan backend:
    ```bash
    cargo build --release
    cargo run
    ```

### Frontend
1. Masuk ke direktori `frontend/`.
2. Bangun frontend dengan WebAssembly:
    ```bash
    wasm-pack build --target web
    ```
3. Jalankan server lokal untuk mengakses aplikasi:
    ```bash
    python3 -m http.server 8080
    ```

Akses aplikasi di `http://localhost:8080`.
