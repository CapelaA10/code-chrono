<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Calendar, ChevronLeft, ChevronRight } from 'lucide-svelte';
  import { strings, locale } from '$lib/i18n/store';

  export let value: string; // YYYY-MM-DD
  export let placeholder: string = "Select Date";

  const dispatch = createEventDispatcher();

  let isOpen = false;
  let currentMonth = new Date();
  
  // Parse value
  $: selectedDate = value ? new Date(value + 'T00:00:00') : null;
  $: if (isOpen && selectedDate) {
    currentMonth = new Date(selectedDate.getFullYear(), selectedDate.getMonth(), 1);
  } else if (isOpen) {
    currentMonth = new Date();
    currentMonth.setDate(1);
  }

  $: monthName = currentMonth.toLocaleString($locale, { month: 'long' });
  $: year = currentMonth.getFullYear();

  function getDaysInMonth(year: number, month: number) {
    return new Date(year, month + 1, 0).getDate();
  }

  function getFirstDayOfMonth(year: number, month: number) {
    let day = new Date(year, month, 1).getDay();
    return day === 0 ? 6 : day - 1; // Mon-Sun
  }

  $: daysInMonth = getDaysInMonth(year, currentMonth.getMonth());
  $: firstDay = getFirstDayOfMonth(year, currentMonth.getMonth());
  
  $: days = Array.from({ length: daysInMonth }, (_, i) => i + 1);
  $: blankDays = Array.from({ length: firstDay }, (_, i) => i);

  function prevMonth() {
    currentMonth = new Date(year, currentMonth.getMonth() - 1, 1);
  }

  function nextMonth() {
    currentMonth = new Date(year, currentMonth.getMonth() + 1, 1);
  }

  function selectDate(day: number) {
    const d = new Date(year, currentMonth.getMonth(), day);
    const yyyy = d.getFullYear();
    const mm = String(d.getMonth() + 1).padStart(2, '0');
    const dd = String(d.getDate()).padStart(2, '0');
    value = `${yyyy}-${mm}-${dd}`;
    isOpen = false;
    dispatch('change', value);
  }
  
  function clearDate() {
    value = '';
    isOpen = false;
    dispatch('change', value);
  }

  $: DAY_NAMES = Array.from({ length: 7 }, (_, i) => {
    const d = new Date(2024, 0, i + 1); // 2024-01-01 is Monday
    const name = d.toLocaleDateString($locale, { weekday: 'short' });
    // Try to grab just 2 letters or shortest representation if it's longer
    return name.slice(0, 2).replace('.', '').trim();
  });

  let popupRef: HTMLElement;

  function closePopup(e: MouseEvent) {
    if (popupRef && !popupRef.contains(e.target as Node)) {
      isOpen = false;
    }
  }

  function toggleOpen(e: MouseEvent) {
    e.stopPropagation();
    isOpen = !isOpen;
  }
</script>

<svelte:window on:click={closePopup} />

<div class="datepicker-wrapper" bind:this={popupRef}>
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="input-trigger" on:click={toggleOpen}>
    <Calendar size={18} class="calendar-icon" />
    <span class="value-text" class:empty={!value}>
      {value ? new Date(value + 'T00:00:00').toLocaleDateString() : placeholder}
    </span>
  </div>

  {#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div class="popup" on:click={(e) => e.stopPropagation()}>
      <div class="header">
        <button class="nav-btn" on:click={prevMonth}><ChevronLeft size={16}/></button>
        <div class="month-year">{monthName} {year}</div>
        <button class="nav-btn" on:click={nextMonth}><ChevronRight size={16}/></button>
      </div>

      <div class="grid days-header">
        {#each DAY_NAMES as d}
          <div class="day-name">{d}</div>
        {/each}
      </div>

      <div class="grid">
        {#each blankDays as _}
          <div class="day blank"></div>
        {/each}
        {#each days as day}
          {@const isSelected = selectedDate?.getDate() === day && selectedDate?.getMonth() === currentMonth.getMonth() && selectedDate?.getFullYear() === year}
          {@const isToday = new Date().getDate() === day && new Date().getMonth() === currentMonth.getMonth() && new Date().getFullYear() === year}
          <button 
            type="button"
            class="day cell" 
            class:selected={isSelected}
            class:today={isToday}
            on:click={() => selectDate(day)}
          >
            {day}
          </button>
        {/each}
      </div>
      
      <div class="footer">
        <button type="button" class="clear-btn" on:click={clearDate}>{$strings.none}</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .datepicker-wrapper {
    position: relative;
    width: 100%;
  }

  .input-trigger {
    background: var(--input-bg);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 0.5rem 0.625rem;
    color: var(--text);
    font-size: 0.875rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    transition: border-color 0.15s;
    user-select: none;
  }

  .input-trigger:hover {
    border-color: var(--accent-blue);
  }

  :global(.calendar-icon) {
    color: var(--text-muted);
  }

  .value-text {
    flex: 1;
    text-align: left;
  }

  .value-text.empty {
    color: var(--text-muted);
    opacity: 0.8;
  }

  .popup {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    z-index: 200;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 1rem;
    box-shadow: var(--shadow-lg), 0 0 0 1px var(--border);
    min-width: 260px;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }

  .month-year {
    font-weight: 600;
    font-size: 0.95rem;
    color: var(--text);
  }

  .nav-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .nav-btn:hover {
    background: var(--bg-page);
    color: var(--text);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 2px;
    text-align: center;
  }

  .days-header {
    margin-bottom: 0.25rem;
  }

  .day-name {
    font-size: 0.7rem;
    font-weight: 600;
    color: var(--text-muted);
    padding: 4px;
    text-transform: uppercase;
  }

  .cell {
    background: transparent;
    border: none;
    cursor: pointer;
    border-radius: 6px;
    padding: 6px 0;
    font-size: 0.85rem;
    color: var(--text);
    transition: all 0.1s;
  }

  .cell:hover:not(.selected) {
    background: var(--bg-page);
  }

  .cell.today {
    font-weight: 700;
    color: var(--accent-blue);
  }

  .cell.selected {
    background: var(--accent-blue);
    color: white;
    font-weight: 600;
  }
  
  .footer {
    display: flex;
    justify-content: center;
    margin-top: 0.5rem;
    padding-top: 0.5rem;
    border-top: 1px solid var(--border);
  }
  
  .clear-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 0.75rem;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
  }
  
  .clear-btn:hover {
    color: var(--text);
    background: var(--bg-page);
  }
</style>
