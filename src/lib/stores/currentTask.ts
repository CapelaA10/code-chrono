import { writable } from "svelte/store";

/** Current task name for the timer (used by global shortcut to toggle pause). */
export const currentTaskName = writable<string>("");
