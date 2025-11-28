import { writable } from "svelte/store";
import { browser } from "$app/environment";
import { get_settings, update_settings } from "$lib/services/settings";
import type { Theme } from "$lib/types/workspace";

export const themes = [
  "system",
  "light",
  "dark",
  "latte",
  "frappe",
  "macchiato",
  "mocha",
] as const;

export const themeLabels: Record<Theme, string> = {
  system: "System",
  light: "Light",
  dark: "Dark",
  latte: "Latte (Catppuccin)",
  frappe: "Frappe (Catppuccin)",
  macchiato: "Macchiato (Catppuccin)",
  mocha: "Mocha (Catppuccin)",
  custom: "Custom",
};

const createThemeStore = () => {
  const { subscribe, set } = writable<Theme>("system");

  function getSystemTheme(): "light" | "dark" {
    if (browser) {
      return window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light";
    }
    return "light";
  }

  function applyThemeToDOM(theme: Theme) {
    if (browser) {
      const allThemes = [
        "light",
        "dark",
        "latte",
        "frappe",
        "macchiato",
        "mocha",
      ];
      document.documentElement.classList.remove(...allThemes);

      const effectiveTheme = theme === "system" ? getSystemTheme() : theme;
      if (effectiveTheme !== "custom") {
        document.documentElement.classList.add(effectiveTheme);
      }
    }
  }

  async function applyTheme(theme: Theme) {
    applyThemeToDOM(theme);
    set(theme);
    try {
      await update_settings({ theme });
    } catch (e) {
      console.error("Failed to save theme setting:", e);
    }
  }

  async function initializeTheme() {
    if (browser) {
      try {
        const settings = await get_settings();
        const theme = settings.theme;
        applyThemeToDOM(theme);
        set(theme);
      } catch (e) {
        console.error("Failed to load theme setting:", e);
        applyThemeToDOM("system");
        set("system");
      }

      // Listen for system theme changes
      window
        .matchMedia("(prefers-color-scheme: dark)")
        .addEventListener("change", () => {
          // Re-apply if using system theme
          const currentTheme = get(themeStore);
          if (currentTheme === "system") {
            applyThemeToDOM("system");
          }
        });
    }
  }

  async function resetToSystem() {
    await applyTheme("system");
  }

  const themeStore = {
    subscribe,
    applyTheme,
    initializeTheme,
    resetToSystem,
  };

  return themeStore;
};

function get<T>(store: {
  subscribe: (fn: (value: T) => void) => () => void;
}): T {
  let value: T;
  store.subscribe((v) => (value = v))();
  return value!;
}

export const theme = createThemeStore();
