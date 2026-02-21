import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Task, Project, Tag } from '$lib/types';

export const tasks = writable<Task[]>([]);
export const projects = writable<Project[]>([]);
export const tags = writable<Tag[]>([]);
export const searchQuery = writable<string>('');
export const filterProject = writable<number | null>(null);
export const filterTag = writable<number | null>(null);
export const filterStatus = writable<string | null>(null);

export async function refreshTasks() {
    try {
        // Tauri converts camelCase JS params to snake_case Rust params automatically
        const t = await invoke<Task[]>('get_tasks', {
            filterProject: get(filterProject),
            filterTag: get(filterTag),
            filterStatus: get(filterStatus)
        });
        tasks.set(t);
    } catch (e) {
        console.error("Refresh tasks error:", e);
    }
}

export async function refreshProjects() {
    try {
        const p = await invoke<Project[]>('get_projects');
        projects.set(p);
    } catch (e) {
        console.error("Refresh projects error:", e);
    }
}

export async function refreshTags() {
    try {
        const t = await invoke<Tag[]>('get_tags');
        tags.set(t);
    } catch (e) {
        console.error("Refresh tags error:", e);
    }
}

export async function refreshAll() {
    await Promise.all([refreshTasks(), refreshProjects(), refreshTags()]);
}

export async function searchTasks(query: string) {
    try {
        const t = await invoke<Task[]>('search_tasks', { query });
        tasks.set(t);
    } catch (e) {
        console.error("Search tasks error:", e);
    }
}
