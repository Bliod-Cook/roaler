import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        app: "hsl(var(--app-bg) / <alpha-value>)",
        panel: "hsl(var(--panel) / <alpha-value>)",
        "panel-strong": "hsl(var(--panel-strong) / <alpha-value>)",
        "panel-soft": "hsl(var(--panel-soft) / <alpha-value>)",
        line: "hsl(var(--line) / <alpha-value>)",
        text: "hsl(var(--text) / <alpha-value>)",
        "text-secondary": "hsl(var(--text-secondary) / <alpha-value>)",
        "text-tertiary": "hsl(var(--text-tertiary) / <alpha-value>)",
        accent: "hsl(var(--accent) / <alpha-value>)",
        "accent-soft": "hsl(var(--accent-soft) / <alpha-value>)",
        success: "hsl(var(--success) / <alpha-value>)",
        danger: "hsl(var(--danger) / <alpha-value>)",
        warning: "hsl(var(--warning) / <alpha-value>)"
      },
      fontFamily: {
        display: ["Space Grotesk", "Segoe UI", "sans-serif"],
        body: ["Manrope", "Segoe UI", "sans-serif"]
      },
      boxShadow: {
        panel: "0 24px 60px rgba(11, 18, 35, 0.08)",
        floating: "0 18px 46px rgba(11, 18, 35, 0.12)"
      },
      borderRadius: {
        panel: "1.5rem",
        card: "1.25rem"
      }
    }
  },
  plugins: [],
};

export default config;
