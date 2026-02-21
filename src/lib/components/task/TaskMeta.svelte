<script lang="ts">
  import { Clock, Tag as TagIcon, Folder } from 'lucide-svelte';
  import type { Tag, Project } from '$lib/types';

  export let dueDate: number | undefined | null;
  export let tags: Tag[];
  export let priority: number;
  export let source: string | undefined | null = null;
  export let project: Project | undefined = undefined;

  function formatDueDate(ts?: number | null) {
    if (!ts) return '';
    const date = new Date(ts * 1000);
    const now = new Date();
    const isOverdue = date < now;
    const formatted = date.toLocaleDateString(undefined, { month: 'short', day: 'numeric' });
    return { formatted, isOverdue };
  }

  function getPriorityLabel(p: number) {
    if (p === 3) return 'High';
    if (p === 2) return 'Med';
    if (p === 1) return 'Low';
    return null;
  }

  function getPriorityColor(p: number) {
    if (p === 3) return 'var(--error-red)';
    if (p === 2) return '#f59e0b';
    if (p === 1) return 'var(--accent-blue)';
    return 'transparent';
  }

  $: dueDateInfo = formatDueDate(dueDate);
  $: priorityLabel = getPriorityLabel(priority);
  $: priorityColor = getPriorityColor(priority);
</script>

<div class="meta">
  {#if dueDateInfo && typeof dueDateInfo === 'object'}
    <span class="due-date" class:overdue={dueDateInfo.isOverdue}>
      <Clock size={11} />
      {dueDateInfo.formatted}
    </span>
  {/if}
  {#if project}
    <span class="project-badge" style="--proj-color: {project.color ?? '#64748b'}">
      <Folder size={11} />
      {project.name}
    </span>
  {/if}
  {#each tags as tag}
    <span class="tag-chip" style="--tag-color: {tag.color ?? '#64748b'}">
      <TagIcon size={11} />
      {tag.name}
    </span>
  {/each}
  {#if priorityLabel}
    <span class="priority" style="color: {priorityColor}; border-color: {priorityColor}">
      {priorityLabel}
    </span>
  {/if}
  {#if source}
    <span class="source-badge">{source}</span>
  {/if}
</div>

<style>
  .meta {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-top: 0.25rem;
    align-items: center;
  }

  .due-date, .tag-chip, .priority, .source-badge, .project-badge {
    font-size: 0.6875rem;
    display: flex;
    align-items: center;
    gap: 0.2rem;
    color: var(--text-muted);
    line-height: 1;
  }

  .due-date.overdue {
    color: var(--error-red);
  }

  .project-badge {
    color: var(--proj-color);
  }

  .tag-chip {
    padding: 0.15rem 0.4rem;
    border-radius: 4px;
    background: color-mix(in srgb, var(--tag-color) 15%, transparent);
    color: var(--tag-color);
    border: 1px solid color-mix(in srgb, var(--tag-color) 30%, transparent);
  }

  .priority {
    padding: 0.1rem 0.375rem;
    border-radius: 4px;
    border: 1px solid;
    font-weight: 600;
  }

  .source-badge {
    padding: 0.1rem 0.375rem;
    border-radius: 4px;
    background: var(--btn-secondary-hover-bg);
    border: 1px solid var(--border);
    font-weight: 500;
    color: var(--text-muted);
  }
</style>
