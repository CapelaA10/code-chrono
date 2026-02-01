import { writable } from "svelte/store";

export const currentTaskName = writable<string>("");
