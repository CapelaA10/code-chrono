/**
 * Shared formatting utilities.
 * Import from here rather than redefining in each component.
 */

/** Format seconds into "mm:ss" — used by the timer display */
export function formatTime(totalSeconds: number): string {
    const m = Math.floor(totalSeconds / 60);
    const s = totalSeconds % 60;
    return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
}

/** Format a seconds value as "Xh Ym" or "Ym" — used by stats */
export function formatDuration(totalSeconds: number): string {
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    return hours > 0 ? `${hours}h ${minutes}m` : `${minutes}m`;
}

/** Format a Unix timestamp (seconds) as a short date label, e.g. "Feb 21" */
export function formatDate(unixSeconds: number): string {
    return new Date(unixSeconds * 1000).toLocaleDateString(undefined, {
        month: 'short',
        day: 'numeric'
    });
}

/**
 * Format a YYYY-MM-DD date string as a long label,
 * e.g. "Saturday, Feb 21"
 */
export function formatDayLabel(dateStr: string): string {
    // Append T00:00:00 so it's parsed in local time, not UTC
    return new Date(`${dateStr}T00:00:00`).toLocaleDateString(undefined, {
        weekday: 'long',
        month: 'short',
        day: 'numeric'
    });
}

/** Returns true when a Unix-seconds timestamp is in the past */
export function isOverdue(unixSeconds: number): boolean {
    return unixSeconds * 1000 < Date.now();
}
