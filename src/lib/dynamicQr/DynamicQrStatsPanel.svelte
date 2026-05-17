<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import {
    getDynamicQrStats,
    type DynamicQrRecord,
    type DynamicQrStatsResponse,
  } from '$lib/dynamicQr/dynamicQrService';

  let {
    record,
    baseUrl,
    apiKey,
    onBack,
    onStatsLoaded = undefined,
    onStatsError = undefined,
  } = $props<{
    record: DynamicQrRecord;
    baseUrl: string;
    apiKey: string;
    onBack: () => void;
    onStatsLoaded?: (totalScans: number, lastScanAt: string | null) => void;
    onStatsError?: (error: string) => void;
  }>();

  const WINDOWS = [
    { label: '24h', value: '24h' },
    { label: '7d',  value: '7d'  },
    { label: '30d', value: '30d' },
    { label: '90d', value: '90d' },
    { label: 'All', value: 'all' },
  ];

  let statsWindow = $state('30d');
  let loading     = $state(false);
  let result      = $state<DynamicQrStatsResponse | null>(null);
  let statsError  = $state('');

  async function fetchStats(w: string) {
    if (!record?.id) { statsError = 'No record ID available.'; return; }
    if (!baseUrl)    { statsError = 'Server URL not configured. Go to Settings.'; return; }
    if (!apiKey)     { statsError = 'API key not configured. Go to Settings.'; return; }
    loading = true;
    statsError = '';
    try {
      result = await getDynamicQrStats({ baseUrl, apiKey }, record.id, w);
      onStatsLoaded?.(result.stats.totalScans, result.stats.lastScanAt ?? null);
    } catch (e: any) {
      statsError = e.message || 'Failed to fetch stats.';
      result = null;
      onStatsError?.(statsError);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    const w = statsWindow;
    fetchStats(w);
  });

  function formatHour(h: number): string {
    const suffix = h < 12 ? 'am' : 'pm';
    const display = h % 12 === 0 ? 12 : h % 12;
    return `${display}${suffix}`;
  }
</script>

<div class="stats-page" in:fly={{ y: 20, duration: 400 }} out:fade>
  <!-- Header -->
  <div class="stats-header">
    <button class="back-btn" type="button" onclick={onBack}>
      <span>←</span> Back to Studio
    </button>
    <h2>Dynamic QR Stats</h2>
  </div>

  <div class="stats-body">
    <!-- Record info card -->
    <div class="info-card">
      <div class="info-row">
        <span class="info-label">Title</span>
        <span class="info-value">{record.title}</span>
      </div>
      <div class="info-row">
        <span class="info-label">Destination</span>
        <span class="info-value break">{record.targetUrl}</span>
      </div>
      <div class="info-row">
        <span class="info-label">Redirect URL</span>
        <span class="info-value break">{record.redirectUrl}</span>
      </div>
      <div class="info-row">
        <span class="info-label">Record ID</span>
        <span class="info-value mono">{record.id}</span>
      </div>
    </div>

    <!-- Window selector + Refresh -->
    <div class="window-row">
      <div class="window-tabs">
        {#each WINDOWS as w}
          <button
            type="button"
            class="window-tab"
            class:active={statsWindow === w.value}
            onclick={() => { statsWindow = w.value; }}
          >{w.label}</button>
        {/each}
      </div>
      <button
        type="button"
        class="refresh-btn"
        disabled={loading}
        onclick={() => fetchStats(statsWindow)}
      >{loading ? '…' : '↻'}</button>
    </div>

    <!-- States -->
    {#if loading}
      <div class="state-box" transition:fade>
        <div class="spinner"></div>
        <span>Loading stats…</span>
      </div>
    {:else if statsError}
      <div class="error-box" transition:fade>
        <span class="err-icon">⚠</span>
        <span>{statsError}</span>
      </div>
    {:else if result}
      {@const s = result.stats}

      <!-- Summary strip -->
      <div class="summary-strip">
        <div class="summary-cell">
          <span class="cell-num">{s.totalScans}</span>
          <span class="cell-lbl">Total Scans</span>
        </div>
        <div class="summary-cell">
          <span class="cell-num small">{s.lastScanAt ?? '—'}</span>
          <span class="cell-lbl">Last Scan</span>
        </div>
        <div class="summary-cell">
          <span class="cell-num small">{s.privacyMode.replace(/_/g, ' ')}</span>
          <span class="cell-lbl">Privacy</span>
        </div>
      </div>

      <!-- Data sections -->
      {#if s.scansByDay.length}
        <div class="stat-section">
          <p class="section-title">Scans by Day</p>
          {#each s.scansByDay as row}
            <div class="stat-row">
              <span class="stat-key">{row.date}</span>
              <span class="stat-val">{row.scans}</span>
            </div>
          {/each}
        </div>
      {/if}

      {#if s.scansByHour.length}
        <div class="stat-section">
          <p class="section-title">Scans by Hour</p>
          {#each s.scansByHour as row}
            <div class="stat-row">
              <span class="stat-key">{formatHour(row.hour)}</span>
              <span class="stat-val">{row.scans}</span>
            </div>
          {/each}
        </div>
      {/if}

      {#if s.scansByCountry.length}
        <div class="stat-section">
          <p class="section-title">Scans by Country</p>
          {#each s.scansByCountry as row}
            <div class="stat-row">
              <span class="stat-key">{row.countryName} ({row.countryCode})</span>
              <span class="stat-val">{row.scans}</span>
            </div>
          {/each}
        </div>
      {/if}

      {#if s.scansByRegion.length}
        <div class="stat-section">
          <p class="section-title">Scans by Region</p>
          {#each s.scansByRegion as row}
            <div class="stat-row">
              <span class="stat-key">{row.region}, {row.countryCode}</span>
              <span class="stat-val">{row.scans}</span>
            </div>
          {/each}
        </div>
      {/if}

      {#if s.scansByTimezone.length}
        <div class="stat-section">
          <p class="section-title">Scans by Timezone</p>
          {#each s.scansByTimezone as row}
            <div class="stat-row">
              <span class="stat-key">{row.timezone}</span>
              <span class="stat-val">{row.scans}</span>
            </div>
          {/each}
        </div>
      {/if}

      {#if s.scansByCity.length}
        <div class="stat-section">
          <p class="section-title">Scans by City</p>
          {#each s.scansByCity as row}
            <div class="stat-row">
              <span class="stat-key">{row.city}, {row.countryCode}</span>
              <span class="stat-val">{row.scans}</span>
            </div>
          {/each}
        </div>
      {/if}

      {#if s.totalScans === 0}
        <div class="no-data">No scans recorded in this window yet.</div>
      {/if}

    {:else}
      <div class="state-box">
        <span>No data yet. Try refreshing.</span>
      </div>
    {/if}
  </div>
</div>

<style>
  :global(html), :global(body) {
    min-height: 100%;
    background-color: #0F0F12;
    background-attachment: fixed;
  }

  .stats-page {
    padding: 20px;
    width: 100%;
    min-height: 100vh;
    box-sizing: border-box;
    color: #F7F8FF;
  }

  .stats-header {
    display: flex;
    align-items: center;
    gap: 20px;
    max-width: 680px;
    margin: 0 auto 24px;
  }

  .back-btn {
    background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.14);
    color: #fff;
    padding: 10px 18px;
    border-radius: 14px;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 700;
    display: flex;
    align-items: center;
    gap: 8px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    flex-shrink: 0;
  }
  .back-btn:active { transform: translateX(-4px); }

  .stats-header h2 {
    font-size: 22px;
    font-weight: 900;
    margin: 0;
    background: linear-gradient(135deg, #21d4fd 0%, #b721ff 100%);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .stats-body {
    max-width: 680px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  /* Record info card */
  .info-card {
    background: #18181F;
    border: 1px solid #2A2A33;
    border-radius: 20px;
    padding: 18px 20px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .info-row {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .info-label {
    font-size: 0.65rem;
    font-weight: 900;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: #21d4fd;
  }
  .info-value {
    font-size: 0.88rem;
    color: rgba(247,248,255,0.85);
    font-weight: 600;
  }
  .info-value.break { word-break: break-all; }
  .info-value.mono  { font-family: 'JetBrains Mono', monospace; font-size: 0.78rem; color: rgba(247,248,255,0.55); }

  /* Window selector */
  .window-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .window-tabs {
    display: flex;
    flex: 1;
    background: rgba(255,255,255,0.05);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 12px;
    padding: 3px;
    gap: 3px;
  }
  .window-tab {
    flex: 1;
    padding: 8px 4px;
    border-radius: 9px;
    border: none;
    cursor: pointer;
    font-size: 0.8rem;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    transition: all 0.2s;
    background: transparent;
    color: rgba(255,255,255,0.5);
  }
  .window-tab.active {
    background: linear-gradient(135deg, #21d4fd 0%, #b721ff 100%);
    color: #fff;
    box-shadow: 0 2px 10px rgba(33,212,253,0.25);
  }
  .refresh-btn {
    width: 42px;
    height: 42px;
    border-radius: 12px;
    border: 1px solid rgba(255,255,255,0.14);
    background: rgba(255,255,255,0.07);
    color: #fff;
    font-size: 1.2rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: all 0.2s;
  }
  .refresh-btn:disabled { opacity: 0.4; cursor: default; }
  .refresh-btn:not(:disabled):active { transform: rotate(180deg); }

  /* State boxes */
  .state-box {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 40px 20px;
    color: rgba(247,248,255,0.45);
    font-size: 0.9rem;
  }
  .spinner {
    width: 20px;
    height: 20px;
    border: 2px solid rgba(33,212,253,0.2);
    border-top-color: #21d4fd;
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
    flex-shrink: 0;
  }
  @keyframes spin { to { transform: rotate(360deg); } }

  .error-box {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 14px 18px;
    border-radius: 14px;
    background: rgba(255,26,146,0.08);
    border: 1px solid rgba(255,26,146,0.25);
    color: #ff6ab0;
    font-size: 0.88rem;
    font-weight: 600;
  }
  .err-icon { font-size: 1.1rem; flex-shrink: 0; }

  /* Summary strip */
  .summary-strip {
    display: flex;
    gap: 10px;
  }
  .summary-cell {
    flex: 1;
    background: #18181F;
    border: 1px solid #2A2A33;
    border-radius: 16px;
    padding: 14px 10px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    text-align: center;
  }
  .cell-num {
    font-size: 1.6rem;
    font-weight: 900;
    color: #21d4fd;
    line-height: 1;
  }
  .cell-num.small {
    font-size: 0.78rem;
    font-weight: 700;
    color: rgba(247,248,255,0.75);
    text-transform: capitalize;
    word-break: break-word;
  }
  .cell-lbl {
    font-size: 0.6rem;
    font-weight: 900;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: rgba(247,248,255,0.38);
  }

  /* Stat sections */
  .stat-section {
    background: #18181F;
    border: 1px solid #2A2A33;
    border-radius: 18px;
    overflow: hidden;
  }
  .section-title {
    margin: 0;
    padding: 12px 16px 8px;
    font-size: 0.68rem;
    font-weight: 900;
    text-transform: uppercase;
    letter-spacing: 0.14em;
    color: #21d4fd;
    border-bottom: 1px solid rgba(255,255,255,0.06);
  }
  .stat-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 16px;
    border-bottom: 1px solid rgba(255,255,255,0.04);
    gap: 12px;
  }
  .stat-row:last-child { border-bottom: none; }
  .stat-key {
    font-size: 0.85rem;
    color: rgba(247,248,255,0.75);
    font-weight: 500;
  }
  .stat-val {
    font-size: 0.9rem;
    font-weight: 800;
    color: #fff;
    flex-shrink: 0;
  }

  .no-data {
    text-align: center;
    color: rgba(247,248,255,0.35);
    font-size: 0.88rem;
    padding: 20px;
  }
</style>
