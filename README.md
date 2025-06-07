# 🔐 fpass – CLI Password Manager

**fpass** is a ⚡ fast, 🔒 secure, and 🖥️ local command-line password manager written entirely in **Rust**.
Your data is **locally encrypted** and **never leaves your system**. ☁️❌ No cloud. No servers. No dependencies.

---

## 📁 Installation

You can compile `fpass` from source without using `cargo install`.

### 🧾 1. Clone the repository

```sh
git clone https://github.com/0l3d/fpass.git
cd fpass
```

### 🛠️ 2. Build the project

Make sure you have the Rust toolchain installed. Then run:

```sh
cargo build --release
```

This will generate the binary at:

```sh
target/release/fpass
```

### 🚚 3. (Optional) Move it to your PATH

```sh
sudo mv target/release/fpass /usr/local/bin/fpass
```

---

## 🧪 Usage

### 🔧 Initial Setup

```sh
fpass setup
```

This will initialize your encrypted password database at:

```plaintext
~/.local/share/fpass/db.json
```

---

## 🧭 Command Overview

```bash
fpass [command] [arguments]
```

### 📂 List all entries

```sh
fpass list
```

### 🔍 Find an entry by name

```sh
fpass find <entry-name>
```

### ➕ Add a new entry

```sh
fpass add
```

### 👁️ Show a password (hidden)

```sh
fpass show <id>
# Output: | <here your password> |
```

### 🔓 Show a password (revealed)

```sh
fpass shown <id>
```

### ❌ Delete an entry

```sh
fpass delete <id>
```

### 📝 Copy functionality (coming soon)

```sh
fpass copy <id> <password/email>
```

> ⚠️ Note: `copy` command is not yet implemented.

---

## 🔐 Security

* 🔒 Passwords are stored encrypted using **AES-256**.
* 🔑 Your vault password is never saved and is required for all operations.
* 📴 No internet connection is used. Everything runs **100% locally**.

---

## 🗂️ File Structure

All data is stored in:

```plaintext
~/.local/share/fpass/db.json
```

📌 This file is encrypted. Do not edit it manually.

---

## 🛠️ Dependencies

* 🦀 Requires Rust toolchain to build.
* 🧼 No runtime dependencies.

---

## 📄 License

📝 Licensed under the GNU General Public License v3.0 (GPL-3.0).You are free to use, modify, and distribute this software under the terms of the GPLv3.
See the LICENSE file for full details.

---

## ✨ Author

Created with ❤️ in Rust by **0l3d**
☕ Support me: [https://buymeacoffee.com/oled](https://buymeacoffee.com/oled)
