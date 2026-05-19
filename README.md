# Tutorial 3 - WebChat using Yew

## Experiment 3.1: Original code

Project ini menggunakan dua aplikasi dari referensi YewChat:

- `SimpleWebsocketServer`: WebSocket server berbasis NodeJS/TypeScript yang berjalan pada port `8080`.
- `YewChat`: web client berbasis Rust Yew yang berjalan pada port `8000`.

Server dapat dijalankan dengan perintah berikut:

```powershell
cd SimpleWebsocketServer
npm install
npm start
```

Client dapat dijalankan dengan perintah berikut:

```powershell
cd YewChat
npm install
npm start
```

Setelah kedua aplikasi berjalan, buka browser ke:

```text
http://127.0.0.1:8000/
```

Untuk mencoba chat, buka dua tab browser atau dua window browser ke alamat yang sama.
Masukkan username yang berbeda pada setiap tab, lalu kirim pesan dari salah satu tab.
Pesan akan dikirim dari Yew client ke WebSocket server pada `ws://127.0.0.1:8080`.
Server kemudian membroadcast pesan tersebut ke semua client yang sedang terhubung.

### WebSocket Server

![WebSocket server running on port 8080](images/server-3.1.png)

Server berhasil berjalan pada port `8080`.
Pada screenshot, server menerima koneksi dari web client yang dibuka melalui browser.
Server ini bertugas menerima pesan dari satu client, menyimpan daftar user aktif, lalu mengirim update user dan pesan chat ke semua client yang terhubung.

### Client 1

![First YewChat client](images/client1-3.1.png)

### Client 2

![Second YewChat client](images/client2-3.1.png)

Pada pengujian ini, dua client dibuka melalui browser dengan username `awan1` dan `awan2`.
Kedua username muncul pada daftar `Users`, sehingga koneksi WebSocket dan proses registrasi user berhasil.
Pesan yang dikirim dari `awan1` muncul di halaman chat dan juga terlihat oleh `awan2`.
Sebaliknya, pesan yang dikirim oleh `awan2` juga muncul pada client lain.
Hal ini menunjukkan bahwa web client Yew berhasil terhubung ke WebSocket server dan server berhasil melakukan broadcast pesan antar client.
