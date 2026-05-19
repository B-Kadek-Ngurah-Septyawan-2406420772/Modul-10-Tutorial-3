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

## Experiment 3.2: Be Creative!

Pada eksperimen ini, saya mengubah tampilan web client YewChat agar terasa lebih modern dan informatif.
Login page diubah menjadi halaman masuk bertema `Orbit Room` dengan visual gelap, accent cyan, dan call-to-action yang lebih jelas.
Chat page juga diubah dengan sidebar gelap untuk daftar user aktif, indikator online, status koneksi, jumlah pesan, dan message bubble yang membedakan pesan milik user sendiri dengan pesan dari user lain.
Jika belum ada pesan, halaman chat menampilkan empty state sehingga area chat tidak terlihat kosong tanpa konteks.
Input pesan juga dibuat lebih jelas dengan placeholder yang memberi tahu bahwa pengguna bisa mengirim teks biasa atau link `.gif`.

Perubahan ini tetap mempertahankan fungsi utama aplikasi.
User masih login menggunakan username, client masih terhubung ke WebSocket server pada `ws://127.0.0.1:8080`, dan pesan tetap dibroadcast ke semua browser yang sedang terhubung.
Kreativitas difokuskan pada pengalaman pengguna: status online lebih mudah dibaca, pengguna dapat melihat dirinya sendiri dengan label `You`, dan pesan yang dikirim sendiri tampil di sisi kanan dengan warna berbeda.

### Login Page 3.2

![Creative login page](images/login-3.2.png)

### Chat Page 3.2

![Creative chat page](images/chat-3.2.png)

### Broadcast Test 3.2

![Creative broadcast test with two clients](images/broadcast-3.2.png)

Pada screenshot broadcast, dua browser dibuka berdampingan dengan user `Awan1` dan `Awan2`.
Masing-masing browser menampilkan label `You` pada user yang sedang aktif di browser tersebut.
Pesan dari `Awan1` dan `Awan2` tetap terkirim melalui WebSocket server dan muncul di client lain, sehingga perubahan visual tidak merusak fungsi broadcast chat.

## Bonus: Rust WebSocket server for YewChat

Pada bagian bonus ini, saya membuat server WebSocket baru menggunakan Rust di folder `rust_websocket_server`.
Server ini menggantikan server JavaScript/TypeScript dari `SimpleWebsocketServer`, tetapi tetap memakai protokol pesan yang sama dengan YewChat.
Karena YewChat mengirim dan menerima data dalam bentuk JSON, server Rust juga membaca field `messageType`, `data`, dan `dataArray` dengan format camelCase.

Server Rust dapat dijalankan dengan perintah berikut:

```powershell
cd rust_websocket_server
cargo run
```

Frontend YewChat tetap dijalankan seperti sebelumnya:

```powershell
cd YewChat
npm start
```

Server Rust berjalan pada port yang sama, yaitu `8080`, sehingga client tetap dapat menggunakan URL WebSocket:

```text
ws://127.0.0.1:8080
```

Ketika client mengirim pesan `register`, server Rust menyimpan pasangan alamat koneksi dan username.
Setelah itu, server mengirim broadcast daftar user aktif dengan format:

```json
{"messageType":"users","data":null,"dataArray":["Awan1","Awan2"]}
```

Ketika client mengirim pesan `message`, server Rust mencari username pengirim, membuat payload pesan berisi `from`, `message`, dan `time`, lalu membroadcast pesan tersebut ke semua client.
Format pesan yang dikirim server adalah:

```json
{"messageType":"message","data":"{\"from\":\"Awan1\",\"message\":\"hello from rust\",\"time\":1779184279234}","dataArray":null}
```

Hasil uji server Rust:

```text
Rust YewChat WebSocket server listening on port 8080
web client connected from 127.0.0.1:57550
127.0.0.1:57550 registered as Awan1
web client connected from 127.0.0.1:57551
127.0.0.1:57551 registered as Awan2
from Awan1: hello from rust
127.0.0.1:57550 disconnected
127.0.0.1:57551 disconnected
```

Hasil uji WebSocket client:

```text
c1 {"messageType":"users","data":null,"dataArray":["Awan1"]}
c2 {"messageType":"users","data":null,"dataArray":["Awan1","Awan2"]}
c1 {"messageType":"users","data":null,"dataArray":["Awan1","Awan2"]}
c1 {"messageType":"message","data":"{\"from\":\"Awan1\",\"message\":\"hello from rust\",\"time\":1779184279234}","dataArray":null}
c2 {"messageType":"message","data":"{\"from\":\"Awan1\",\"message\":\"hello from rust\",\"time\":1779184279234}","dataArray":null}
```

Menurut saya, versi JavaScript lebih cepat dibuat karena library `ws` sederhana dan format JSON langsung cocok dengan ekosistem web.
Namun, versi Rust lebih saya sukai untuk server yang akan dikembangkan lebih serius karena tipe datanya lebih eksplisit, error handling lebih terstruktur, dan ownership membantu mengurangi kesalahan pada shared state.
Untuk tugas ini, Rust server berhasil menjadi pengganti server JavaScript karena YewChat tetap bisa menerima daftar user dan pesan broadcast dengan format yang sama.
