# Double Exponential Smoothing (Desktop)
## Implementasi metode Double Exponential Smoothing pada penjualan barang

Dibuat menggunakan bahasa pemrograman *Rust* untuk _backend proccessing_ dan menggunakan Framework Vue.js untuk frontend.

## Kebutuhan (_Requirement_)
### Linux
#### System Dependencies
```bash
$ sudo apt update && sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev appmenu-gtk3-module libgtk-3-dev squashfs-tools
```
#### Node.js Runtime and Package Manager
```bash
$ curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.2/install.sh | bash
$ nvm install node --latest-npm && nvm use {{version}}
$ npm install -g yarn
```
#### Rustc and Cargo Package Manager
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ cargo install diesel_cli --no-default-features --features sqlite
$ cargo install tauri-bundler --force
```
### MacOS
#### System Dependencies
```bash
$ brew install gcc
$ sudo xcode-select --install
```
#### Node.js Runtime and Package Manager
```bash
$ curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.2/install.sh | bash
$ nvm install node --latest-npm && nvm use {{version}}
$ npm install -g yarn
```
#### Rustc and Cargo Package Manager
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ cargo install diesel_cli --no-default-features --features sqlite
$ cargo install tauri-bundler --force
```
### Windows
#### System Dependencies
Pertama, kita harus [mengunduh](https://aka.ms/buildtools) dan menginstal Visual Studio MSBuild Tools dan C ++ build tools.
#### Node.js Runtime and Package Manager
Selanjutnya, direkomendasikan mengunduh [nvm-windows](https://github.com/coreybutler/nvm-windows#installation--upgrades) untuk mengelola runtime Node.js.

Kemudian jalankan perintah berikut dari PowerShell Administratif dan tekan Y saat diminta:

```powershell
nvm install latest && nvm use {{version}}
npm install -g yarn
```

#### Rustc and Cargo Package Manager
Sekarang kita perlu menginstal Rust. Cara termudah untuk melakukannya adalah dengan menggunakan rustup, penginstal resmi.
- [64-bit download link](https://win.rustup.rs/x86_64)
- [32-bit download link](https://win.rustup.rs/i686)

Unduh dan instal varian yang tepat untuk arsitektur komputer kita. Setelah selesai, buka PowerShell dan masukkan:
```powershell
cargo install diesel_cli --no-default-features --features sqlite
cargo install tauri-bundler --force
```
#### Enable Loopback
Microsoft menonaktifkan inteface loopback, kita harus mengizinkannya jika kita ingin menggunakan dev-server. Buka PowerShell administratif dan masukkan:

```powershell
CheckNetIsolation.exe LoopbackExempt -a -n="Microsoft.Win32WebViewHost_cw5n1h2txyewy"
```

*Note:*
Setelah itu direkomendasikan untuk me-restart komputer kita setelah menjalankan perintah ini, jadi jika tidak berhasil, coba itu!
### WebView2 (Chromium)
Terakhir kita perlu mengunduh Windows Edge Webview, kita dapat mengunduh Edge WebView2 [di sini](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).

----------

## Menjalankan Program (_Running Program_)
Setelah kebutuhan sistem terpenuhi selanjutnya kita lakukan langkah berikut:
```bash
$ git clone https://github.com/fatkhur1960/des-desktop.git && cd des-desktop
$ yarn install 
$ cp .env.example .env && source .env 
$ diesel setup && diesel migration run 
$ yarn tauri:serve 
```
