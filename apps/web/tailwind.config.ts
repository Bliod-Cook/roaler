import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        paper: "oklch(0.97 0.015 80)",
        ink: "oklch(0.27 0.03 55)",
        accent: "oklch(0.6 0.14 42)",
        accentSoft: "oklch(0.9 0.05 45)",
        borderWarm: "oklch(0.84 0.02 70)"
      },
      fontFamily: {
        display: ["Fraunces", "Georgia", "serif"],
        body: ["Instrument Sans", "Segoe UI", "sans-serif"]
      },
      boxShadow: {
        editorial: "0 12px 40px rgba(67, 43, 24, 0.08)"
      }
    }
  },
  plugins: [],
};

export default config;

