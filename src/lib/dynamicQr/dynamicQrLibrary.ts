import type { DynamicQrRecord } from './dynamicQrService';

const STORAGE_KEY = 'qr-studio-ultra.dynamic-qr-library';
const STALE_DAYS = 90;
const NEW_RECORD_GRACE_DAYS = 7;

export type StatsStatus = 'not_loaded' | 'loading' | 'loaded' | 'error';

export interface DynamicQrLibraryEntry {
  record: DynamicQrRecord;
  savedAt: string;
  statsStatus: StatsStatus;
  cachedTotalScans: number | null;
  cachedLastScanAt: string | null;
  cachedFetchedAt: string | null;
}

function migrateEntry(raw: any): DynamicQrLibraryEntry {
  // Derive statsStatus for entries saved before this field existed
  const statsStatus: StatsStatus = raw.statsStatus ?? (raw.cachedTotalScans !== null ? 'loaded' : 'not_loaded');
  return {
    record: raw.record,
    savedAt: raw.savedAt ?? new Date().toISOString(),
    statsStatus,
    cachedTotalScans: raw.cachedTotalScans ?? null,
    cachedLastScanAt: raw.cachedLastScanAt ?? null,
    cachedFetchedAt: raw.cachedFetchedAt ?? null,
  };
}

export function loadLibrary(): DynamicQrLibraryEntry[] {
  if (typeof window === 'undefined') return [];
  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    const parsed: any[] = raw ? JSON.parse(raw) : [];
    return parsed.map(migrateEntry);
  } catch {
    return [];
  }
}

function persist(entries: DynamicQrLibraryEntry[]): void {
  if (typeof window === 'undefined') return;
  window.localStorage.setItem(STORAGE_KEY, JSON.stringify(entries));
}

export function addToLibrary(record: DynamicQrRecord): DynamicQrLibraryEntry[] {
  const existing = loadLibrary();
  const deduped = existing.filter(e => e.record.id !== record.id);
  const entry: DynamicQrLibraryEntry = {
    record,
    savedAt: new Date().toISOString(),
    statsStatus: 'not_loaded',
    cachedTotalScans: null,
    cachedLastScanAt: null,
    cachedFetchedAt: null,
  };
  const updated = [entry, ...deduped];
  persist(updated);
  return updated;
}

export function removeFromLibrary(recordId: string): DynamicQrLibraryEntry[] {
  const updated = loadLibrary().filter(e => e.record.id !== recordId);
  persist(updated);
  return updated;
}

export function updateLibraryStats(
  recordId: string,
  totalScans: number,
  lastScanAt: string | null
): DynamicQrLibraryEntry[] {
  const updated = loadLibrary().map(e =>
    e.record.id === recordId
      ? {
          ...e,
          statsStatus: 'loaded' as StatsStatus,
          cachedTotalScans: totalScans,
          cachedLastScanAt: lastScanAt,
          cachedFetchedAt: new Date().toISOString(),
        }
      : e
  );
  persist(updated);
  return updated;
}

export function updateLibraryStatsStatus(
  recordId: string,
  status: StatsStatus
): DynamicQrLibraryEntry[] {
  const updated = loadLibrary().map(e =>
    e.record.id === recordId ? { ...e, statsStatus: status } : e
  );
  persist(updated);
  return updated;
}

export function isStale(entry: DynamicQrLibraryEntry): boolean {
  // Only evaluate staleness once stats have been successfully loaded
  if (entry.statsStatus !== 'loaded') return false;

  // Never stale if the record was created within the grace period
  try {
    const created = new Date(entry.record.createdAt.replace(' ', 'T'));
    const graceCutoff = new Date();
    graceCutoff.setDate(graceCutoff.getDate() - NEW_RECORD_GRACE_DAYS);
    if (created > graceCutoff) return false;
  } catch { /* ignore unparseable date */ }

  if (entry.cachedTotalScans === 0) return true;
  if (!entry.cachedLastScanAt) return true;
  try {
    const last = new Date(entry.cachedLastScanAt.replace(' ', 'T'));
    const cutoff = new Date();
    cutoff.setDate(cutoff.getDate() - STALE_DAYS);
    return last < cutoff;
  } catch {
    return true;
  }
}

export function formatSavedDate(iso: string): string {
  try {
    return new Date(iso).toLocaleDateString(undefined, {
      day: 'numeric', month: 'short', year: 'numeric'
    });
  } catch {
    return iso;
  }
}
