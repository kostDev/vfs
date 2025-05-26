# 🧠 vfs-easy

- 🇺🇦 [Читати українською](./README_UA.md)

> A lightweight virtual file system with CLI, daemon and Unix socket API  
> 🦀 Built with Rust for cybertools, hacking fun, and real system extensibility

---

## 🚀 Features

- 🗃️ Create and manage `.vfs` volume files
- 🔄 Load and save volumes on demand
- 📁 Add local files to the VFS
- 📜 Display file contents via `cat`
- 📂 List all files in the VFS
- 📡 Communicate between CLI and daemon via Unix socket (`/tmp/vfs.sock`)
- ✅ Works on Linux and macOS

---

## 📁 Project Structure

```
vfs-easy/
├── src/
│   ├── main.rs        # CLI interface (vfs)
│   ├── vfsd.rs        # Daemon logic (vfsd)
│   ├── vfs.rs         # Core virtual file system logic
│   ├── ipc.rs         # Unix socket communication
│   ├── proto.rs       # Command/request protocol structs
│   ├── cli.rs         # CLI command parsing
├── .fs/               # (Optional) directory to store VFS files
├── Cargo.toml
└── README.md
```

---

## ⚙️ Build

```bash
# Build the CLI binary or makefile command
cargo build --bin vfs
build-vfs
# Build the daemon  or makefile command
cargo build --bin vfsd
build-vfsd
```

---

## 🧪 Example Usage

### 🔌 Start the daemon

```bash
./target/debug/vfsd
```

> Starts listening on `/tmp/vfs.sock` for client requests

---

### 💻 CLI Commands

```bash
# Create a new VFS volume
vfs create .fs/mydata.vfs

# Load an existing volume
vfs load .fs/mydata.vfs

# Add a file into the VFS
vfs add ./test.txt

# Read a file's content from the VFS
vfs cat test.txt

# List all files in the VFS
vfs ls

# Save the current in-memory VFS to disk
vfs save .fs/mydata.vfs
```

---

## 🛠️ Dependencies

- [Rust](https://www.rust-lang.org/)
- [bincode](https://crates.io/crates/bincode)
- [chrono](https://crates.io/crates/chrono)
- [serde](https://crates.io/crates/serde)

---

## 💡 Future Ideas

- 🔐 Encrypted volumes
- 🌐 TUI or WebSocket interface
- 🧠 FUSE-mountable volumes
- 📦 Tag system, archive support, in-RAM phantom files

---

## 🧬 Author

This project was built as an experiment in building minimal virtual file systems and powerful CLI experiences in Rust.
