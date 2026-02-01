<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import Icons from "$lib/components/Icons.svelte";

    type TaskStats = {
        task_name: string;
        sessions: number;
        total_seconds: number;
    };

    type DailyStats = {
        day: string;
        task_name: string;
        sessions: number;
        total_seconds: number;
    };

    type FilterPeriod = "today" | "week" | "month" | "custom";

    let stats: TaskStats[] = [];
    let dailyStats: DailyStats[] = [];
    let loading = false;
    let selectedPeriod: FilterPeriod = "today";
    let customStartDate = "";
    let customEndDate = "";

    $: totalSessions = stats.reduce((sum, s) => sum + s.sessions, 0);
    $: totalMinutes = Math.floor(
        stats.reduce((sum, s) => sum + s.total_seconds, 0) / 60,
    );
    $: totalHours = Math.floor(totalMinutes / 60);
    $: remainingMinutes = totalMinutes % 60;

    onMount(() => {
        loadStats();
    });

    function getTimestampRange(): [number, number] {
        const now = new Date();
        const endOfDay = new Date(
            now.getFullYear(),
            now.getMonth(),
            now.getDate(),
            23,
            59,
            59,
        );
        const endTimestamp = Math.floor(endOfDay.getTime() / 1000);

        let startTimestamp: number;

        switch (selectedPeriod) {
            case "today": {
                const startOfDay = new Date(
                    now.getFullYear(),
                    now.getMonth(),
                    now.getDate(),
                    0,
                    0,
                    0,
                );
                startTimestamp = Math.floor(startOfDay.getTime() / 1000);
                break;
            }
            case "week": {
                const startOfWeek = new Date(now);
                startOfWeek.setDate(now.getDate() - now.getDay());
                startOfWeek.setHours(0, 0, 0, 0);
                startTimestamp = Math.floor(startOfWeek.getTime() / 1000);
                break;
            }
            case "month": {
                const startOfMonth = new Date(
                    now.getFullYear(),
                    now.getMonth(),
                    1,
                    0,
                    0,
                    0,
                );
                startTimestamp = Math.floor(startOfMonth.getTime() / 1000);
                break;
            }
            case "custom": {
                if (!customStartDate || !customEndDate) {
                    const startOfDay = new Date(
                        now.getFullYear(),
                        now.getMonth(),
                        now.getDate(),
                        0,
                        0,
                        0,
                    );
                    return [
                        Math.floor(startOfDay.getTime() / 1000),
                        endTimestamp,
                    ];
                }
                const start = new Date(customStartDate);
                start.setHours(0, 0, 0, 0);
                const end = new Date(customEndDate);
                end.setHours(23, 59, 59, 999);
                return [
                    Math.floor(start.getTime() / 1000),
                    Math.floor(end.getTime() / 1000),
                ];
            }
        }

        return [startTimestamp, endTimestamp];
    }

    async function loadStats() {
        loading = true;
        try {
            const [startTimestamp, endTimestamp] = getTimestampRange();

            stats = await invoke<TaskStats[]>("get_task_stats", {
                startTimestamp,
                endTimestamp,
            });

            dailyStats = await invoke<DailyStats[]>("get_daily_breakdown", {
                startTimestamp,
                endTimestamp,
            });
        } catch (error) {
            console.error("Failed to load stats:", error);
            stats = [];
            dailyStats = [];
        } finally {
            loading = false;
        }
    }

    function formatDuration(seconds: number): string {
        const hours = Math.floor(seconds / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);
        if (hours > 0) {
            return `${hours}h ${minutes}m`;
        }
        return `${minutes}m`;
    }

    function getBarWidth(taskSeconds: number): number {
        const maxSeconds = Math.max(...stats.map((s) => s.total_seconds), 1);
        return (taskSeconds / maxSeconds) * 100;
    }

    function setPeriod(period: FilterPeriod) {
        selectedPeriod = period;
        loadStats();
    }

    function applyCustomDates() {
        if (customStartDate && customEndDate) {
            loadStats();
        }
    }

    $: dailyGroups = dailyStats.reduce(
        (acc, stat) => {
            if (!acc[stat.day]) {
                acc[stat.day] = [];
            }
            acc[stat.day].push(stat);
            return acc;
        },
        {} as Record<string, DailyStats[]>,
    );
</script>

<main class="page">
    <div class="stats">
        <a href="/" class="back">
            <Icons name="back" size={18} className="back-icon" />
            Back to timer
        </a>
        <h1 class="title">Statistics</h1>

        <section class="section">
            <div class="filter-buttons">
                <button
                    class="filter-btn"
                    class:active={selectedPeriod === "today"}
                    on:click={() => setPeriod("today")}
                >
                    Today
                </button>
                <button
                    class="filter-btn"
                    class:active={selectedPeriod === "week"}
                    on:click={() => setPeriod("week")}
                >
                    This Week
                </button>
                <button
                    class="filter-btn"
                    class:active={selectedPeriod === "month"}
                    on:click={() => setPeriod("month")}
                >
                    This Month
                </button>
                <button
                    class="filter-btn"
                    class:active={selectedPeriod === "custom"}
                    on:click={() => setPeriod("custom")}
                >
                    Custom
                </button>
            </div>

            {#if selectedPeriod === "custom"}
                <div class="custom-date-range">
                    <input
                        type="date"
                        bind:value={customStartDate}
                        class="date-input"
                    />
                    <span class="date-separator">to</span>
                    <input
                        type="date"
                        bind:value={customEndDate}
                        class="date-input"
                    />
                    <button class="btn-apply" on:click={applyCustomDates}>
                        Apply
                    </button>
                </div>
            {/if}
        </section>

        <section class="section">
            <div class="summary-cards">
                <div class="card">
                    <div class="card-label">Total Sessions</div>
                    <div class="card-value">{totalSessions}</div>
                </div>
                <div class="card">
                    <div class="card-label">Total Time</div>
                    <div class="card-value">
                        {#if totalHours > 0}
                            {totalHours}h {remainingMinutes}m
                        {:else}
                            {remainingMinutes}m
                        {/if}
                    </div>
                </div>
            </div>
        </section>

        <section class="section">
            <h2 class="section-title">Time by Task</h2>
            {#if loading}
                <p class="loading">Loading...</p>
            {:else if stats.length === 0}
                <p class="empty">No data for this period</p>
            {:else}
                <div class="task-list">
                    {#each stats as stat}
                        <div class="task-item">
                            <div class="task-header">
                                <span class="task-name">{stat.task_name}</span>
                                <span class="task-duration"
                                    >{formatDuration(stat.total_seconds)}</span
                                >
                            </div>
                            <div class="task-bar-container">
                                <div
                                    class="task-bar"
                                    style="width: {getBarWidth(
                                        stat.total_seconds,
                                    )}%"
                                ></div>
                            </div>
                            <div class="task-sessions">
                                {stat.sessions} session{stat.sessions !== 1
                                    ? "s"
                                    : ""}
                            </div>
                        </div>
                    {/each}
                </div>
            {/if}
        </section>

        {#if Object.keys(dailyGroups).length > 0}
            <section class="section">
                <h2 class="section-title">Daily Breakdown</h2>
                <div class="daily-list">
                    {#each Object.entries(dailyGroups) as [day, dayStats]}
                        <div class="daily-group">
                            <div class="daily-date">
                                {new Date(day).toLocaleDateString()}
                            </div>
                            <div class="daily-tasks">
                                {#each dayStats as stat}
                                    <div class="daily-task">
                                        <span class="daily-task-name"
                                            >{stat.task_name}</span
                                        >
                                        <span class="daily-task-time"
                                            >{formatDuration(
                                                stat.total_seconds,
                                            )}</span
                                        >
                                    </div>
                                {/each}
                            </div>
                        </div>
                    {/each}
                </div>
            </section>
        {/if}
    </div>
</main>

<style>
    .page {
        width: 100%;
        max-width: 500px;
        padding: 1rem;
    }

    :global(body) {
        margin: 0;
        padding: 0;
        min-height: 100vh;
        background: var(--bg-page);
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
            sans-serif;
        display: flex;
        align-items: flex-start;
        justify-content: center;
        padding-top: 2rem;
        color: var(--text);
    }

    .stats {
        background: var(--bg-card);
        border: 1px solid var(--border);
        border-radius: 12px;
        padding: 1.5rem 1.25rem;
        box-shadow: var(--shadow);
    }

    .back {
        display: inline-flex;
        align-items: center;
        gap: 0.35rem;
        color: var(--text-muted);
        text-decoration: none;
        font-size: 0.8125rem;
        margin-bottom: 1rem;
        transition: color 0.15s;
    }

    .back:hover {
        color: var(--text);
    }

    .title {
        font-size: 1.25rem;
        font-weight: 600;
        color: var(--text);
        margin: 0 0 1.25rem 0;
    }

    .section {
        margin-bottom: 1.5rem;
    }

    .section-title {
        font-size: 0.6875rem;
        font-weight: 600;
        color: var(--text-muted);
        text-transform: uppercase;
        letter-spacing: 0.1em;
        margin: 0 0 0.75rem 0;
    }

    .filter-buttons {
        display: flex;
        gap: 0.5rem;
        flex-wrap: wrap;
    }

    .filter-btn {
        padding: 0.5rem 1rem;
        border: 1px solid var(--border);
        border-radius: 8px;
        font-size: 0.875rem;
        background: var(--bg-card);
        color: var(--text-muted);
        cursor: pointer;
        transition:
            background 0.15s,
            color 0.15s,
            border-color 0.15s;
    }

    .filter-btn:hover {
        color: var(--text);
        background: var(--btn-secondary-hover-bg);
    }

    .filter-btn.active {
        border-color: var(--accent-blue);
        color: var(--accent-blue);
        background: var(--accent-blue-hover);
    }

    .custom-date-range {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin-top: 0.75rem;
        flex-wrap: wrap;
    }

    .date-input {
        padding: 0.4rem 0.5rem;
        border: 1px solid var(--input-border);
        border-radius: 8px;
        font-size: 0.875rem;
        background: var(--input-bg);
        color: var(--text);
    }

    .date-input:focus {
        outline: none;
        border-color: var(--accent-blue);
    }

    .date-separator {
        font-size: 0.875rem;
        color: var(--text-muted);
    }

    .btn-apply {
        padding: 0.4rem 0.75rem;
        border: 1px solid var(--accent-blue-border);
        border-radius: 8px;
        font-size: 0.875rem;
        background: var(--bg-card);
        color: var(--accent-blue);
        cursor: pointer;
        transition: background 0.15s;
    }

    .btn-apply:hover {
        background: var(--accent-blue-hover);
    }

    .summary-cards {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 0.75rem;
    }

    .card {
        padding: 1rem;
        background: var(--bg-page);
        border: 1px solid var(--border);
        border-radius: 8px;
    }

    .card-label {
        font-size: 0.6875rem;
        font-weight: 600;
        color: var(--text-muted);
        text-transform: uppercase;
        letter-spacing: 0.05em;
        margin-bottom: 0.5rem;
    }

    .card-value {
        font-size: 1.5rem;
        font-weight: 600;
        color: var(--text);
        font-variant-numeric: tabular-nums;
    }

    .loading,
    .empty {
        font-size: 0.875rem;
        color: var(--text-muted);
        text-align: center;
        padding: 2rem 0;
    }

    .task-list {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .task-item {
        padding: 0.75rem;
        background: var(--bg-page);
        border: 1px solid var(--border);
        border-radius: 8px;
    }

    .task-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 0.5rem;
    }

    .task-name {
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--text);
    }

    .task-duration {
        font-size: 0.875rem;
        font-weight: 600;
        color: var(--accent-blue);
        font-variant-numeric: tabular-nums;
    }

    .task-bar-container {
        width: 100%;
        height: 6px;
        background: var(--border);
        border-radius: 3px;
        overflow: hidden;
        margin-bottom: 0.5rem;
    }

    .task-bar {
        height: 100%;
        background: var(--accent-blue);
        border-radius: 3px;
        transition: width 0.3s ease;
    }

    .task-sessions {
        font-size: 0.75rem;
        color: var(--text-muted);
    }

    .daily-list {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .daily-group {
        padding: 0.75rem;
        background: var(--bg-page);
        border: 1px solid var(--border);
        border-radius: 8px;
    }

    .daily-date {
        font-size: 0.75rem;
        font-weight: 600;
        color: var(--text-muted);
        text-transform: uppercase;
        letter-spacing: 0.05em;
        margin-bottom: 0.5rem;
    }

    .daily-tasks {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .daily-task {
        display: flex;
        justify-content: space-between;
        align-items: center;
        font-size: 0.875rem;
    }

    .daily-task-name {
        color: var(--text);
    }

    .daily-task-time {
        color: var(--text-muted);
        font-variant-numeric: tabular-nums;
    }
</style>
