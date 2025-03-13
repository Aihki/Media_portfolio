import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path';

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue({
      template: {
        compilerOptions: {
          isCustomElement: (tag) => tag.includes('img-comparison-slider')
        }
      }
    })
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    }
  },
 /*  server: {
    headers: {
      'Access-Control-Allow-Origin': '*',
      'Content-Type': 'application/javascript',
    },
    fs: {
      strict: false
    }
  }, */
  build: {
    rollupOptions: {
      output: {
        format: 'es'
      }
    }
  }
});