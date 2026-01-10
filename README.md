# React + Vite + Rust (Actix) Template

A modern full-stack template featuring React + Vite on the frontend and Rust (Actix-web) on the backend.

---

## What's Included

- React + Vite frontend
- Rust backend using Actix-web
- Fast, type-safe API server
- Monorepo-style structure
- CORS and JSON handling configured
- One-command development setup

---

## Prerequisites

Make sure you have the following installed:

- Node.js (v18+ recommended)
- Rust & Cargo  

---

## Getting Started

#### 1. [Create a new repo from this template](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template).

#### 2. Install root dependencies:

This installs tooling used to run both client and server:

```bash
npm install
```

#### 3. Install project dependencies:
Install frontend dependencies and prepare the backend:

```bash
npm run install:all
```

Alternatively, you can install them independently:

- Frontend (React + Vite):
  ```bash
  npm run install:client
  ```
  Or
  ```bash
  cd client
  npm install
  ```
- Backend (Actix-web):
  ```bash
  npm run install:server
  ```
  Or
  ```bash
  cd server
  cargo build
  ```

#### 4. Run the scripts in development mode:

The PORTS are:

- 9000 for the client
- 9001 for the server

To run both the frontend and backend concurrently in development mode:

```bash
npm run dev
```

Alternatively, you can run them independently:

- Frontend (React + Vite):
  ```bash
  npm run dev:client
  ```
- Backend (Actix-web):
  ```bash
  npm run dev:server
  ```

---

### API Status Check

The Actix server includes a basic health endpoint:

- `GET /api/status` → `{ "message":"Server is running!", "status":"ok" }`
- `GET /api/hello` → `{ "message": "Hello World!" }`

---

### Environment Variables

Create a `.env` file to configure environment variables. Copy the `.env.example` and rename it to `.env`.

| Key  | Description                        | Value |
| ---- | ---------------------------------- | ----- |
| PORT | Port number for the Actix server   | 9001  |

---