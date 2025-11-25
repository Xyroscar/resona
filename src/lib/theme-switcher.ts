import { writable } from "svelte/store";
import { browser } from "$app/environment";

export const themes = [
  "light",
  "dark",
  "latte",
  "frappe",
  "macchiato",
  "mocha",
] as const;
type Theme = (typeof themes)[number];

const createThemeStore = () => {
  const { subscribe, set } = writable<Theme>("light");

  function applyTheme(theme: Theme) {
    if (browser) {
      document.documentElement.classList.remove(...themes);
      document.documentElement.classList.add(theme);
      localStorage.setItem("theme", theme);
      set(theme);
    }
  }

  function initializeTheme() {
    if (browser) {
      const storedTheme = localStorage.getItem("theme") as Theme | null;
      const systemTheme: Theme = window.matchMedia(
        "(prefers-color-scheme: dark)"
      ).matches
        ? "dark"
        : "light";
      const initialTheme = storedTheme ?? systemTheme;
      applyTheme(initialTheme);
    }
  }

  function resetToSystem() {
    if (browser) {
      localStorage.removeItem("theme");
      initializeTheme();
    }
  }

  return {
    subscribe,
    applyTheme,
    initializeTheme,
    resetToSystem,
  };
};

export const theme = createThemeStore();
