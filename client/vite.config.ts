import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

const BACKEND_PORT = Number(process.env.PORT) || 9001;

export default defineConfig({
  plugins: [react()],
  server: {
    port: 9000,
    proxy: {
      '/api': {
        target: `http://127.0.0.1:${BACKEND_PORT}`,
        changeOrigin: true,
      },
    },
  },
});
