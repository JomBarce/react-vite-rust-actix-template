# Frontend — React + Vite + Typescript

This folder contains the **React + Vite frontend** for the full-stack template.

---

## Tech Stack

- React
- Vite
- TypeScript
- ESLint + Prettier

---

## Getting Started

#### Install project dependencies:
From the project root:
```bash
npm run install:client
```
Or directly inside the client folder:
```bash
cd client
npm install
```

#### Run the scripts in development mode:
From the project root:
```bash
npm run dev:client
```
Or directly inside the client folder:
```bash
npm run dev
```
By default, the dev server runs on:
```bash
http://localhost:9000
```

---

### Linting & Formatting

ESLint and Prettier are configured for code quality and formatting consistency.

To run linting:

```bash
npm run lint
```

To run formatting:

```bash
npm run format
```

---

### Environment Variables

Create a `.env` file to configure environment variables. Copy the `.env.example` and rename it to `.env`.

| Key  | Description                        | Value |
| ---- | ---------------------------------- | ----- |
| PORT | Port number for the Actix server   | 9001  |

---

## Production Workflow

Once you're ready to deploy or build the project for production:

```bash
npm run build
```

Preview the build locally:

```bash
npm run preview
```