/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        workspace: {
          950: '#0a0c0e',
          900: '#111317',
          850: '#16191f',
          800: '#1e222a',
          700: '#2b313c',
          600: '#3d4452',
          muted: '#8b949e',
          amber: {
            DEFAULT: '#f59e0b',
            glow: '#fbbf24',
          },
          blue: {
            DEFAULT: '#38bdf8',
            glow: '#60a5fa',
          },
        },
      },
      boxShadow: {
        'glow-amber': '0 0 35px -5px rgba(245, 158, 11, 0.25)',
        'glow-blue': '0 0 35px -5px rgba(56, 189, 248, 0.25)',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
    },
  },
  plugins: [],
};
