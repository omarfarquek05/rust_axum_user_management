# User Management API

A production-ready **User Management REST API** built with **Rust**, **Axum**, **SQLx**, and **Neon PostgreSQL**. This project demonstrates clean architecture, asynchronous programming, database migrations, validation, and scalable backend development using the Rust ecosystem.

---

## 🚀 Tech Stack

| Technology | Purpose |
|------------|---------|
| Rust | Programming Language |
| Axum | Web Framework |
| Tokio | Async Runtime |
| SQLx | Async PostgreSQL Driver |
| Neon PostgreSQL | Cloud Database |
| Serde | JSON Serialization |
| dotenvy | Environment Variables |
| UUID | Unique ID Generation |
| Tower HTTP | Middleware |
| Tracing | Logging |

---

## 📁 Project Structure

```text
user-management/
│
├── src/
│   ├── main.rs
│   ├── db.rs
│   ├── handlers/
│   ├── models/
│   ├── routes/
│   ├── dto/
│   ├── services/
│   ├── repository/
│   ├── errors/
│   └── utils/
│
├── migrations/
│
├── .env.example
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

# Features

- RESTful API
- Create User
- Get All Users
- Get User by ID
- Update User
- Delete User
- PostgreSQL Integration
- SQLx Migrations
- Async/Await
- Environment Configuration
- Structured Error Handling
- JSON Responses
- Input Validation
- Clean Folder Structure

---

# Installation

## Clone Repository

```bash
git clone https://github.com/your-username/user-management.git

cd user-management
```

---

## Install Dependencies

```bash
cargo build
```

---

## Create Environment File

Create a `.env` file.

Example:

```env
DATABASE_URL=postgres://username:password@host/database
```

For Neon PostgreSQL:

```env
DATABASE_URL=postgres://USER:PASSWORD@HOST.neon.tech/DBNAME?sslmode=require
```

---

# Database Migration

Create migration

```bash
sqlx migrate add create_users
```

Run migration

```bash
sqlx migrate run
```

Revert migration

```bash
sqlx migrate revert
```

---

# Run the Project

```bash
cargo run
```

Server starts at

```
http://localhost:3000
```

---

# API Endpoints

## Health Check

```
GET /
```

Response

```json
{
  "message": "Server is running"
}
```

---

## Create User

```
POST /users
```

Request

```json
{
  "username": "john",
  "email": "john@example.com"
}
```

Response

```json
{
  "id": 1,
  "username": "john",
  "email": "john@example.com"
}
```

---

## Get All Users

```
GET /users
```

---

## Get User By ID

```
GET /users/:id
```

---

## Update User

```
PUT /users/:id
```

---

## Delete User

```
DELETE /users/:id
```

---

# Build

Debug

```bash
cargo build
```

Release

```bash
cargo build --release
```

---

# Testing

```bash
cargo test
```

---

# Useful Cargo Commands

```bash
cargo fmt
cargo clippy
cargo check
cargo clean
cargo run
```

---

# SQLx Commands

Generate Migration

```bash
sqlx migrate add migration_name
```

Run Migration

```bash
sqlx migrate run
```

Check Database

```bash
cargo sqlx prepare
```

---

# Environment Variables

| Variable | Description |
|----------|-------------|
| DATABASE_URL | PostgreSQL Connection String |
| RUST_LOG | Logging Level |

---

# Future Improvements

- JWT Authentication
- Refresh Token
- Role Based Access Control (RBAC)
- Email Verification
- Password Reset
- Pagination
- Filtering
- Searching
- Sorting
- Docker Support
- Docker Compose
- Swagger / OpenAPI
- Redis Cache
- Unit Tests
- Integration Tests
- CI/CD Pipeline

---

# Contributing

1. Fork the repository.
2. Create a new feature branch.
3. Commit your changes.
4. Push your branch.
5. Open a Pull Request.

---

# License

This project is licensed under the MIT License.

---

# Author

**Your Name**

GitHub: https://github.com/your-username

LinkedIn: https://linkedin.com/in/your-profile
