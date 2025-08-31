# fpass – CLI Password Manager

**fpass** is a fast, secure, and local command-line password manager written entirely in **Rust**.
Your data is **locally encrypted** and **never leaves your system**. No cloud. No servers. No dependencies.

---

## Installation

## Install with cargo

```
cargo install fpass
```

### Build from scratch

#### 1. Clone the repository

```sh
git clone https://github.com/0l3d/fpass.git
cd fpass
```

#### 2. Build the project

Make sure you have the Rust toolchain installed. Then run:

```sh
cargo build --release
```

This will generate the binary at:

```sh
target/release/fpass
```

---

## Usage

### Initial Setup

```sh
fpass setup
```

This will initialize your encrypted password database at:

```plaintext
~/.local/share/fpass/db.json
```

---

## Command Overview

```bash
fpass -h
```

---

## Security

- Passwords are stored encrypted using **AES-256**.
- Your vault password is never saved and is required for all operations.
- No internet connection is used. Everything runs **100% locally**.

---

## Dependencies

- Requires Rust toolchain to build.
- No runtime dependencies.

---

## License

Licensed under the GNU General Public License v3.0 (GPL-3.0).You are free to use, modify, and distribute this software under the terms of the GPLv3.
See the LICENSE file for full details.

---

## Author

Created with in Rust by **0l3d**  
Support me: [https://buymeacoffee.com/oled](https://buymeacoffee.com/oled)
