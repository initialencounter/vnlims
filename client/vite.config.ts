import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { viteSingleFile } from 'vite-plugin-singlefile'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), viteSingleFile()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  build: {
    outDir: '../server-rs/api/static',
  },
  server: {
    proxy: {
      '^/(get-login-status|search-property|save-config|get-config|reload-config|upload|upload-selected|get-project-info|get-attachment-info|upload-llm-files|ping|get-captcha|login|reload-clipkeeper-config|set-clipboard-text|search-file)': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true,
      },
    }
  },
  publicDir: './src/public',
})
