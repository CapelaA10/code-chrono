import { writable } from "svelte/store";

export type Theme = "light" | "dark";

const STORAGE_KEY = "code-chrono-theme";

function getInitialTheme(): Theme {
  if (typeof document === "undefined") return "light";
  const stored = localStorage.getItem(STORAGE_KEY) as Theme | null;
  if (stored === "light" || stored === "dark") return stored;
  return "light";
}

function applyTheme(theme: Theme) {
  if (typeof document === "undefined") return;
  document.documentElement.setAttribute("data-theme", theme);
  localStorage.setItem(STORAGE_KEY, theme);
}

export function createThemeStore() {
  const initial = getInitialTheme();
  applyTheme(initial);
  const { subscribe, set, update } = writable<Theme>(initial);

  return {
    subscribe,
    set: (theme: Theme) => {
      applyTheme(theme);
      set(theme);
    },
    toggle: () => {
      update((t) => {
        const next = t === "light" ? "dark" : "light";
        applyTheme(next);
        return next;
      });
    },
    init: () => applyTheme(getInitialTheme()),
  };
}

export const theme = createThemeStore();
