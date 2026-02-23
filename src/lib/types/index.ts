// ── Domain models (mirror the Rust structs exactly) ────────────────────────

export interface Project {
    id: number;
    name: string;
    color?: string | null;
}

export interface Tag {
    id: number;
    name: string;
    color?: string | null;
}

export type TaskStatus = 'todo' | 'doing' | 'done';

export interface Task {
    id: number;
    title: string;
    description?: string | null;
    due_date?: number | null;
    priority: number;
    status: TaskStatus;
    project_id?: number | null;
    parent_id?: number | null;
    position: number;
    external_id?: string | null;
    source?: string | null;
    created_at: number;
    completed_at?: number | null;
    tags: Tag[];
}

// ── Timer ──────────────────────────────────────────────────────────────────

export interface TimerState {
    remaining: number;
    paused: boolean;
    /** 0 = work session, 1 = short break, 2 = long break */
    phase: number;
    task_active: boolean;
    active_task_name: string | null;
    session_duration: number;
    last_activity: number;
}

// ── Statistics ─────────────────────────────────────────────────────────────

/** Returned by `get_task_stats` */
export interface TaskStat {
    task_name: string;
    sessions: number;
    total_seconds: number;
}

/** Returned by `get_daily_breakdown` */
export interface DailyStat {
    /** ISO date string "YYYY-MM-DD" */
    day: string;
    task_name: string;
    sessions: number;
    total_seconds: number;
}

// ── Integrations ───────────────────────────────────────────────────────────

export interface ExternalTask {
    id: string;
    title: string;
    description: string | null;
    status: string;
    url: string;
    labels: string[];
    project: string | null;
    already_imported: boolean;
}

