import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // @marktext/file-icons 是 CommonJS 产物（却声明了 module 字段），
  // dev 下强制交给 esbuild 预打包成 ESM（提供 default 导出），
  // build 下用 commonjsOptions 做 CJS → ESM 互操作。
  optimizeDeps: {
    include: ["@marktext/file-icons"],
  },
  build: {
    commonjsOptions: {
      include: [/file-icons/, /node_modules/],
    },
  },
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
