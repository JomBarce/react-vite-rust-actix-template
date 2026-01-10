# Backend — Rust + Actix-web

This folder contains the **Actix-web backend** for the full-stack template.

---

## Tech Stack

- Rust
- Actix-web
- Actix-cors
- dotenv
- MongoDB
- Serde + Serde JSON
- Futures-util

---

## Getting Started

#### Install project dependencies:
From the project root:
```bash
npm run install:server
```
Or you can build directly inside the server folder:
```bash
cd server
cargo build
```

#### Run the scripts in development mode:
From the project root:
```bash
npm run dev:server
```
Or directly inside the server folder:
```bash
cargo run
```
The server will run at:
```bash
http://127.0.0.1:9001
```

---

### API Endpoints

The backend exposes a simple API:

| Method  | Endpoint              | Response                                             |
| ------- | --------------------- | ---------------------------------------------------- |
| GET     | `/api/hello`          | `{ "message": "Hello World!" }`                      |
| GET     | `/api/status`         | `{ "message":"Server is running!", "status":"ok" }`  |


---

### Environment Variables

Create a `.env` file in the `server/` folder based on `.env.example`.

| Key       | Description                        | Value |
| --------- | ---------------------------------- | ----- |
| PORT      | Port number for the Actix server   | 9001  |
| MONGO_URI | MongoDB connection URI             |

---

## Production Workflow

Once you're ready to deploy or build the project for production:

```bash
cargo build --release
```