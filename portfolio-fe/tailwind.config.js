/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      animation: {
        'typewriter': 'typing 3s ease-out forwards',
        'caret': 'blink 1s steps(2) infinite',
        'spin-slow': 'spin 20s linear infinite', 
      },
      keyframes: {
        typing: {
          '0%': { width: '0' },
          '100%': { width: '100%' }
        },
        blink: {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0' }
        },
        spin: {
          from: {
            transform: 'rotate(0deg)'
          },
          to: {  
            transform: 'rotate(360deg)'
          }
        }
      }
    },
  },
  plugins: [],
}

