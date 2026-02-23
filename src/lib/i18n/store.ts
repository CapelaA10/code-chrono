import { writable, derived } from 'svelte/store';
import type { StringKey } from './locales/en';
import { en } from './locales/en';
import { pt } from './locales/pt';
import { br } from './locales/br';
import { es } from './locales/es';
import { el } from './locales/el';

// ── Types ──────────────────────────────────────────────────────────────────

export type Locale = 'en' | 'pt' | 'br' | 'es' | 'el';

export const LOCALE_OPTIONS: { value: Locale; label: string }[] = [
    { value: 'en', label: '🇬🇧 English' },
    { value: 'pt', label: '🇵🇹 Português' },
    { value: 'br', label: '🇧🇷 Português (BR)' },
    { value: 'es', label: '🇪🇸 Español' },
    { value: 'el', label: '🇬🇷 Ελληνικά' },
];

// ── Store ──────────────────────────────────────────────────────────────────

const STORAGE_KEY = 'code-chrono-locale';

function getInitialLocale(): Locale {
    if (typeof localStorage === 'undefined') return 'en';
    const stored = localStorage.getItem(STORAGE_KEY);
    const valid: Locale[] = ['en', 'pt', 'br', 'es', 'el'];
    return valid.includes(stored as Locale) ? (stored as Locale) : 'en';
}

export const locale = writable<Locale>(getInitialLocale());

locale.subscribe((v) => {
    if (typeof localStorage !== 'undefined') localStorage.setItem(STORAGE_KEY, v);
});

// ── Dictionary lookup ─────────────────────────────────────────────────────

const LOCALES = { en, pt, br, es, el } as const;

export const strings = derived(locale, ($locale) => LOCALES[$locale]);

/** Returns the translated string for the given key under the active locale. */
export function t(key: StringKey): string {
    return LOCALES[getInitialLocale()][key];
}
