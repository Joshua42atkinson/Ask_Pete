/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./frontend/src/**/*.rs", "./frontend/index.html"],
  theme: {
    extend: {
      colors: {
        "industrial-slate": "#1B1B1E", // Main Background
        "industrial-surface": "#2A2A2E", // Panel Backgrounds
        "purdue-gold": "#CFB991", // Base Gold (Metallic)
        "purdue-prime": "#FFD700", // Highlight Gold (Bright)
        "purdue-black": "#000000",
        "blueprint-blue": "#1B2A41",
      },
      animation: {
        blob: "blob 7s infinite",
        "fade-in": "fadeIn 0.5s ease-out forwards",
      },
      keyframes: {
        blob: {
          "0%": { transform: "translate(0px, 0px) scale(1)" },
          "33%": { transform: "translate(30px, -50px) scale(1.1)" },
          "66%": { transform: "translate(-20px, 20px) scale(0.9)" },
          "100%": { transform: "translate(0px, 0px) scale(1)" },
        },
        fadeIn: {
          "0%": { opacity: "0", transform: "translateY(10px)" },
          "100%": { opacity: "1", transform: "translateY(0)" },
        },
      },
      backgroundImage: {
        "blueprint-grid": "linear-gradient(to right, #334155 1px, transparent 1px), linear-gradient(to bottom, #334155 1px, transparent 1px)",
      },
      backgroundSize: {
        "blueprint-grid": "40px 40px",
      },
    },
  },
  plugins: [],
}
