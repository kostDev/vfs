# 🧠 vfs-easy

> Проста віртуальна файлова система з CLI, демоном та Unix socket API  
> 🦀 Побудовано з любов’ю на Rust для кібервикористання, з розширюваністю в основі

---

## 🚀 Можливості

- 🗃️ Створення `.vfs` файлів (volume)
- 🔄 Завантаження та збереження VFS
- 📁 Додавання файлів до VFS
- 📜 Перегляд вмісту (`cat`)
- 📂 Лістинг (`ls`)
- 📡 Комунікація між CLI та демоном через Unix socket (`/tmp/vfs.sock`)
- ✅ Крос-платформено (Linux/macOS)

---

## 📁 Структура проєкту

```
vfs-easy/
├── src/
│   ├── main.rs        # CLI інтерфейс (vfs)
│   ├── vfsd.rs        # Демон (vfsd)
│   ├── vfs.rs         # Ядро віртуальної файлової системи
│   ├── ipc.rs         # Зв'язок по сокету з демоном
│   ├── proto.rs       # Структури команд і відповідей
│   ├── cli.rs         # CLI логіка
├── .fs/               # (Опційно) директорія для VFS-файлів
├── Cargo.toml
└── README.md
```

---

## ⚙️ Компіляція

```bash
# Зібрати CLI
cargo build --bin vfs

# Зібрати демон
cargo build --bin vfsd
```

---

## 🧪 Приклад використання

### 🔌 Запустити демон

```bash
./target/debug/vfsd
```

> Відкриває сокет `/tmp/vfs.sock` і чекає на команди

---

### 💻 Команди CLI

```bash
# Створити новий volume
vfs create .fs/mydata.vfs

# Завантажити volume у демона
vfs load .fs/mydata.vfs

# Додати файл до VFS
vfs add ./test.txt

# Переглянути вміст файлу у VFS
vfs cat test.txt

# Показати список файлів у VFS
vfs ls

# Зберегти volume назад у файл
vfs save .fs/mydata.vfs
```

---

## 🛠️ Залежності

- [Rust](https://www.rust-lang.org/)
- [bincode](https://crates.io/crates/bincode)
- [chrono](https://crates.io/crates/chrono)
- [serde](https://crates.io/crates/serde)

---

## 💡 Ідеї на майбутнє

- 🔐 Шифрування volume-файлів
- 🌐 WebSocket API / TUI
- 🧠 VFS як FUSE-монтуємий пристрій
- 📦 Архівація, tag-система, фантом-файли в RAM

---

## 🧬 Автор

Проєкт створено для експериментів із VFS на Rust у рамках дослідження системного дизайну та CLI UX.
