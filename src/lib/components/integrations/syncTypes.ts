/**
 * Shared TypeScript types for the SyncPreviewModal component family.
 * Imported by all sub-components to guarantee consistency.
 */
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
