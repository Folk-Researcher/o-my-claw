/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: '#5B8CFF',
        secondary: '#FF6B9C',
        background: '#0B1020',
        accent: '#F5D76E',
      },
    },
  },
  plugins: [],
}