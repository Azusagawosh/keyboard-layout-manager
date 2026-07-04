# ⌨️ Keyboard Layout Sync

Синхронизация раскладки клавиатуры между двумя компьютерами через локальную сеть.  
Клиент-серверное приложение на Rust.

**Проблема:** При использовании Deskflow раскладка на клиенте (Linux) и сервере (Windows) может расходиться.  
**Решение:** Программа автоматически отправляет раскладку активного окна с клиента на сервер.

---

## 🚀 Быстрый старт

### Windows (сервер)

```powershell
cargo run --release -- server --port 8080

cargo run --release -- client --server-ip 192.168.1.XXX --port 8080

keyboard-sync/
├── src/
│   ├── client/      # Клиент (отслеживает раскладку)
│   ├── server/      # Сервер (принимает команды)
│   ├── common/      # Общие структуры (команды)
│   ├── network/     # TCP-соединение
│   └── platform/    # WinAPI / X11
├── Cargo.toml
└── README.md

cargo build --release

📦 Зависимости
    tokio — асинхронный рантайм
    serde / serde_json — сериализация команд
    windows-sys — Windows API
    x11 — Linux X11

📌 Лицензия
MIT
