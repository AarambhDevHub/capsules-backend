# Capsule Backend

The Backend for **Capsule** — a sleek, responsive interface built using **Axum (Rust)**. It connects to the Capsule Frontend (powered by Next.js) and currently works without authentication.

---

## 🚀 Tech Stack

- Axum
- Rust
- Tokio
- SQLx (for PostgreSQL)
- dotenvy

---

## 🛠️ Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL
- sqlx-cli for managing migrations



### Installation

Clone the repo and install dependencies:

```bash
git clone https://github.com/AarambhDevHub/capsules-backend.git
cd capsule-backend
cargo build
```
### Setup Environment Variables
Create a .env file in the root directory:
```bash
DATABASE_URL=postgres://postgres:yourpassword@localhost/capsule_db
```
### Run Database Migrations
Make sure your database is running, then run:
```bash
sqlx database create
sqlx migrate run
```

### Run the Development Server
```bash
cargo run
```

The API server will be live at:
👉 `http://localhost:8000`

## 💬 Join Our Community
📢 Discord: [Discord Link](https://discord.gg/cDAbFuAC)

## ☕ Support the Project
If you find this project helpful, consider buying me a coffee!
[Buy Me a Coffee](https://buymeacoffee.com/aarambhdevhub)
