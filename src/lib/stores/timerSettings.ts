import { writable } from "svelte/store";

const STORAGE_KEY = "code-chrono-timer-duration";

function getInitial(): number {
    if (typeof document === "undefined") return 25;
    const v = localStorage.getItem(STORAGE_KEY);
    const n = parseInt(v ?? "25", 10);
    return Number.isFinite(n) && n > 0 ? n : 25;
}

export const timerDuration = writable<number>(getInitial());

timerDuration.subscribe((v) => {
    if (typeof localStorage !== "undefined") localStorage.setItem(STORAGE_KEY, String(v));
});