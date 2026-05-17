import { invoke } from '@tauri-apps/api/core';

export interface DynamicQrServerConfig {
  baseUrl: string;
  apiKey?: string;
}

export interface DynamicQrRecord {
  id: string;
  shortCode: string;
  redirectUrl: string;
  targetUrl: string;
  title: string;
  statsEnabled: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface DynamicQrCreateInput {
  title: string;
  targetUrl: string;
  statsEnabled: boolean;
}

export interface StatsByDay    { date: string; scans: number; }
export interface StatsByHour   { hour: number; scans: number; }
export interface StatsByCountry { countryCode: string; countryName: string; scans: number; }
export interface StatsByRegion  { region: string; regionCode: string; countryCode: string; countryName: string; scans: number; }
export interface StatsByTimezone { timezone: string; scans: number; }
export interface StatsByCity   { city: string; countryCode: string; countryName: string; region: string; regionCode: string; timezone: string; scans: number; }

export interface DynamicQrStats {
  privacyMode: string;
  totalScans: number;
  uniqueScans: number | null;
  lastScanAt: string | null;
  scansByDay: StatsByDay[];
  scansByHour: StatsByHour[];
  scansByCountry: StatsByCountry[];
  scansByRegion: StatsByRegion[];
  scansByTimezone: StatsByTimezone[];
  scansByCity: StatsByCity[];
}

export interface DynamicQrStatsResponse {
  recordId: string;
  shortCode: string;
  window: string;
  stats: DynamicQrStats;
}

export function normalizeDynamicQrBaseUrl(baseUrl: string): string {
  let url = baseUrl.trim();
  if (!url) return '';
  if (url.endsWith('/')) url = url.slice(0, -1);
  return url;
}

export function isValidHttpUrl(value: string): boolean {
  try {
    const url = new URL(value);
    return url.protocol === 'http:' || url.protocol === 'https:';
  } catch {
    return false;
  }
}

export async function checkDynamicQrHealth(config: DynamicQrServerConfig): Promise<boolean> {
  const baseUrl = normalizeDynamicQrBaseUrl(config.baseUrl);
  if (!baseUrl) throw new Error('Server URL is required');
  return invoke<boolean>('check_dynamic_qr_health', { baseUrl, apiKey: config.apiKey ?? '' });
}

export async function createDynamicQr(config: DynamicQrServerConfig, input: DynamicQrCreateInput): Promise<DynamicQrRecord> {
  const baseUrl = normalizeDynamicQrBaseUrl(config.baseUrl);
  if (!baseUrl) throw new Error('Server URL is required');
  if (!config.apiKey) throw new Error('API Key is required');
  if (!input.title.trim()) throw new Error('Title is required');
  if (input.title.length > 120) throw new Error('Title must be 120 characters or less');
  if (!isValidHttpUrl(input.targetUrl)) throw new Error('A valid destination URL is required (starting with http:// or https://)');
  return invoke<DynamicQrRecord>('create_dynamic_qr', {
    baseUrl,
    apiKey: config.apiKey,
    title: input.title,
    targetUrl: input.targetUrl,
    statsEnabled: input.statsEnabled,
  });
}

export async function deleteDynamicQr(config: DynamicQrServerConfig, recordId: string): Promise<boolean> {
  const baseUrl = normalizeDynamicQrBaseUrl(config.baseUrl);
  if (!baseUrl) throw new Error('Server URL is required');
  if (!config.apiKey) throw new Error('API Key is required');
  if (!recordId) throw new Error('Record ID is required');
  return invoke<boolean>('delete_dynamic_qr', { baseUrl, apiKey: config.apiKey, recordId });
}

export async function getDynamicQrStats(
  config: DynamicQrServerConfig,
  recordId: string,
  window = '30d'
): Promise<DynamicQrStatsResponse> {
  const baseUrl = normalizeDynamicQrBaseUrl(config.baseUrl);
  if (!baseUrl) throw new Error('Server URL is required');
  if (!config.apiKey) throw new Error('API Key is required');
  if (!recordId) throw new Error('Record ID is required');
  return invoke<DynamicQrStatsResponse>('get_dynamic_qr_stats', {
    baseUrl,
    apiKey: config.apiKey,
    recordId,
    window,
  });
}
