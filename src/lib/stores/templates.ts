import { writable } from 'svelte/store';
import type { StringKey } from '$lib/i18n/locales/en';

// ── Types ──────────────────────────────────────────────────────────────────

export interface TaskTemplate {
    id: string;
    name: string;
    title: string;
    priority: number;
    project_id: number | null;
    description: string;
    tagIds: number[];
}

// ── Store ──────────────────────────────────────────────────────────────────

const STORAGE_KEY = 'code-chrono-templates';

function load(): TaskTemplate[] {
    if (typeof localStorage === 'undefined') return [];
    try {
        return JSON.parse(localStorage.getItem(STORAGE_KEY) ?? '[]');
    } catch {
        return [];
    }
}

export const templates = writable<TaskTemplate[]>(load());

templates.subscribe((v) => {
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(v));
    }
});

// ── Helpers ────────────────────────────────────────────────────────────────

/** Persist a new template (or overwrite if same id). */
export function saveTemplate(t: TaskTemplate) {
    templates.update(list => {
        const idx = list.findIndex(x => x.id === t.id);
        return idx >= 0 ? list.with(idx, t) : [...list, t];
    });
}

/** Remove a template by id. */
export function deleteTemplate(id: string) {
    templates.update(list => list.filter(t => t.id !== id));
}
