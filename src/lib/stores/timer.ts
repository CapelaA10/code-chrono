import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { TimerState } from '$lib/types';

export const activeTimer = writable<TimerState | null>(null);

let initialized = false;

export async function initTimerStore() {
    if (typeof window === 'undefined' || initialized) return;
    initialized = true;

    try {
        const initialState = await invoke<TimerState>('get_timer');
        activeTimer.set(initialState);

        await listen<TimerState>('timer-tick', (e) => {
            activeTimer.set(e.payload);
        });
    } catch (e) {
        console.error("Failed to init timer store", e);
    }
}
