import { writable } from "svelte/store";

const STORAGE_KEY = "code-chrono-idle-minutes";

function getInitial(): number {
  if (typeof document === "undefined") return 0;
  const v = localStorage.getItem(STORAGE_KEY);
  const n = parseInt(v ?? "0", 10);
  return Number.isFinite(n) && n >= 0 ? n : 0;
}

export const idleMinutes = writable<number>(getInitial());

idleMinutes.subscribe((v) => {
  if (typeof localStorage !== "undefined") localStorage.setItem(STORAGE_KEY, String(v));
});
