import { writable } from 'svelte/store';

export const selectedDurationStore = writable<"25" | "45" | "60" | "custom">("25");
export const customDurationStore = writable<string>("");