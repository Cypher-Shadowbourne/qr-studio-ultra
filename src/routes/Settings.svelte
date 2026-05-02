<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api/core';
  import { aiProviders, getProviderLabel, testAiProvider } from '$lib/aiService';
  import { settingsStore, type AiProvider } from '$lib/settingsStore.svelte';

  let { onBack } = $props<{ onBack: () => void }>();

  let selectedProvider = $state<AiProvider>(settingsStore.preferredAiProvider);
  let apiKeys = $state({ ...settingsStore.aiApiKeys });
  let showKey = $state(false);
  let testing = $state(false);
  let testStatus = $state<{ type: 'success' | 'error' | 'idle'; message: string }>({ type: 'idle', message: '' });

  const selectedProviderMeta = $derived(aiProviders.find((provider) => provider.id === selectedProvider) ?? aiProviders[0]);

  function syncSelectedProvider() {
    settingsStore.setPreferredAiProvider(selectedProvider);
    testStatus = { type: 'success', message: `Preferred provider set to ${getProviderLabel(selectedProvider)}.` };
    setTimeout(() => { testStatus = { type: 'idle', message: '' }; }, 2500);
  }

  function handleSave() {
    settingsStore.setPreferredAiProvider(selectedProvider);
    settingsStore.setProviderApiKey(selectedProvider, apiKeys[selectedProvider]);
    testStatus = { type: 'success', message: `${getProviderLabel(selectedProvider)} key saved locally.` };
    setTimeout(() => { testStatus = { type: 'idle', message: '' }; }, 3000);
  }

  function handleClear() {
    if (confirm(`Clear the ${getProviderLabel(selectedProvider)} API key?`)) {
      apiKeys = { ...apiKeys, [selectedProvider]: '' };
      settingsStore.clearProviderApiKey(selectedProvider);
      testStatus = { type: 'success', message: `${getProviderLabel(selectedProvider)} key cleared.` };
      setTimeout(() => { testStatus = { type: 'idle', message: '' }; }, 3000);
    }
  }

  async function testConnection() {
    const key = apiKeys[selectedProvider].trim();
    if (!key) {
      testStatus = { type: 'error', message: `Enter a ${getProviderLabel(selectedProvider)} key first.` };
      return;
    }

    testing = true;
    testStatus = { type: 'idle', message: `Testing ${getProviderLabel(selectedProvider)}...` };

    try {
      settingsStore.setProviderApiKey(selectedProvider, key);
      const result = await testAiProvider(selectedProvider, { ...settingsStore.aiApiKeys, [selectedProvider]: key });
      testStatus = {
        type: 'success',
        message: `Connected through ${getProviderLabel(result.provider_used ?? selectedProvider)}.`
      };
    } catch (err: any) {
      testStatus = { type: 'error', message: `Connection failed: ${err}` };
    } finally {
      testing = false;
    }
  }

  function openProviderKeyPage() {
    invoke('open_external_link', { url: selectedProviderMeta.keyUrl });
  }
</script>

<div class="settings-page" in:fly={{ y: 20, duration: 400 }} out:fade>
  <div class="settings-header">
    <button class="back-btn" type="button" onclick={onBack}>
      <span class="icon">←</span> Back to Studio
    </button>
    <h2>Settings</h2>
  </div>

  <div class="settings-content glass-panel">
    <section class="settings-section">
      <div class="section-header">
        <span class="section-icon">✨</span>
        <div>
          <h3>AI Magic Configuration</h3>
          <p class="eyebrow">Multi-provider creative engine</p>
        </div>
      </div>

      <p class="description">
        Choose your preferred AI provider. QR Studio Ultra tries that provider first, then falls back through configured providers if one is rate-limited or unavailable.
      </p>

      <div class="provider-grid">
        {#each aiProviders as provider}
          <button
            class:active={selectedProvider === provider.id}
            class="provider-card"
            type="button"
            onclick={() => {
              selectedProvider = provider.id;
              syncSelectedProvider();
            }}
          >
            <span>{provider.label}</span>
            <small>{provider.model}</small>
          </button>
        {/each}
      </div>

      <div class="input-group">
        <label for="provider-select">Preferred AI Provider</label>
        <select id="provider-select" class="text-input provider-select" bind:value={selectedProvider} onchange={syncSelectedProvider}>
          {#each aiProviders as provider}
            <option value={provider.id}>{provider.label}</option>
          {/each}
        </select>
      </div>

      <div class="input-group">
        <label for="provider-key">{selectedProviderMeta.label} API Key</label>
        <div class="password-wrapper">
          <input
            id="provider-key"
            type={showKey ? 'text' : 'password'}
            bind:value={apiKeys[selectedProvider]}
            placeholder={selectedProviderMeta.placeholder}
            class="text-input"
            autocomplete="off"
          />
          <button class="toggle-visibility" type="button" onclick={() => showKey = !showKey}>
            {showKey ? 'Hide' : 'Show'}
          </button>
        </div>
        <button class="link-btn" type="button" onclick={openProviderKeyPage}>
          Get {selectedProviderMeta.label} Key
        </button>
      </div>

      <div class="provider-status-row">
        {#each aiProviders as provider}
          <span class:ready={Boolean(apiKeys[provider.id])} class="provider-pill">
            {provider.label}: {apiKeys[provider.id] ? 'Ready' : 'No key'}
          </span>
        {/each}
      </div>

      {#if testStatus.type !== 'idle'}
        <div class="status-msg {testStatus.type}" transition:fade>
          <span class="status-icon">
            {#if testStatus.type === 'success'}✓{:else}⚠{/if}
          </span>
          {testStatus.message}
        </div>
      {/if}

      <div class="settings-actions">
        <button class="action-btn test-btn" type="button" onclick={testConnection} disabled={testing}>
          {testing ? 'Testing...' : `Test ${selectedProviderMeta.label}`}
        </button>
        <div class="main-actions">
          <button class="action-btn clear-btn" type="button" onclick={handleClear}>Clear</button>
          <button class="action-btn save-btn primary-gradient" type="button" onclick={handleSave}>Save</button>
        </div>
      </div>
    </section>
  </div>
</div>

<style>
  :global(html),
  :global(body) {
    min-height: 100%;
    background: #0F0F12;
  }

  .settings-page {
    padding: 20px;
    width: 100%;
    min-height: 100vh;
    box-sizing: border-box;
    background: #0F0F12;
    color: white;
  }

  .settings-header {
    display: flex;
    align-items: center;
    gap: 20px;
    max-width: 680px;
    margin: 0 auto 30px;
  }

  .settings-content {
    max-width: 680px;
    margin: 0 auto;
  }

  .back-btn {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: white;
    padding: 8px 16px;
    border-radius: 12px;
    cursor: pointer;
    font-size: 14px;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: all 0.2s;
  }

  .back-btn:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .settings-header h2 {
    font-size: 24px;
    font-weight: 800;
    margin: 0;
    background: var(--accent-gradient, linear-gradient(135deg, #a855f7, #3b82f6));
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .glass-panel {
    background: rgba(20, 20, 30, 0.72);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 24px;
    padding: 30px;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4);
  }

  .section-header {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 10px;
  }

  .section-icon {
    font-size: 20px;
  }

  .settings-section h3 {
    margin: 0;
    font-size: 18px;
  }

  .eyebrow {
    margin: 4px 0 0;
    color: #9db0c8;
    font-size: 0.78rem;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    font-weight: 800;
  }

  .description {
    color: rgba(255, 255, 255, 0.64);
    font-size: 14px;
    line-height: 1.6;
    margin-bottom: 25px;
  }

  .provider-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 10px;
    margin-bottom: 22px;
  }

  .provider-card {
    min-width: 0;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 14px;
    padding: 13px;
    background: rgba(255, 255, 255, 0.05);
    color: #f5f7fb;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s;
  }

  .provider-card:hover,
  .provider-card.active {
    border-color: rgba(33, 212, 253, 0.55);
    background: rgba(33, 212, 253, 0.1);
  }

  .provider-card span,
  .provider-card small {
    display: block;
  }

  .provider-card span {
    font-weight: 800;
  }

  .provider-card small {
    margin-top: 5px;
    color: #94a3b8;
    font-size: 0.78rem;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 22px;
  }

  .input-group label {
    font-size: 13px;
    font-weight: 700;
    color: rgba(255, 255, 255, 0.82);
    margin-left: 4px;
  }

  .password-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .text-input {
    width: 100%;
    box-sizing: border-box;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 14px;
    padding: 12px 16px;
    padding-right: 70px;
    color: white;
    font-family: 'JetBrains Mono', monospace;
    font-size: 14px;
    transition: all 0.2s;
  }

  .provider-select {
    padding-right: 16px;
  }

  .text-input:focus {
    outline: none;
    border-color: #a855f7;
    background: rgba(0, 0, 0, 0.5);
    box-shadow: 0 0 0 4px rgba(168, 85, 247, 0.1);
  }

  .toggle-visibility {
    position: absolute;
    right: 12px;
    background: rgba(255, 255, 255, 0.1);
    border: none;
    color: rgba(255, 255, 255, 0.7);
    padding: 4px 8px;
    border-radius: 8px;
    font-size: 11px;
    font-weight: 800;
    cursor: pointer;
    text-transform: uppercase;
  }

  .link-btn {
    background: none;
    border: none;
    color: #60a5fa;
    font-size: 12px;
    text-align: left;
    cursor: pointer;
    padding: 0;
    margin-left: 4px;
    text-decoration: underline;
  }

  .provider-status-row {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 18px;
  }

  .provider-pill {
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 999px;
    padding: 6px 10px;
    color: #94a3b8;
    background: rgba(255, 255, 255, 0.05);
    font-size: 0.76rem;
    font-weight: 800;
  }

  .provider-pill.ready {
    color: #9ce3c2;
    border-color: rgba(34, 197, 94, 0.22);
    background: rgba(34, 197, 94, 0.08);
  }

  .settings-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 26px;
    gap: 15px;
    flex-wrap: wrap;
  }

  .main-actions {
    display: flex;
    gap: 12px;
  }

  .action-btn {
    padding: 10px 20px;
    border-radius: 14px;
    font-size: 14px;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.2s;
    border: 1px solid transparent;
  }

  .test-btn {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(255, 255, 255, 0.1);
    color: white;
  }

  .clear-btn {
    background: rgba(239, 68, 68, 0.1);
    border-color: rgba(239, 68, 68, 0.2);
    color: #ef4444;
  }

  .save-btn {
    color: white;
    border: none;
  }

  .primary-gradient {
    background: linear-gradient(135deg, #a855f7, #3b82f6);
    box-shadow: 0 4px 15px rgba(168, 85, 247, 0.3);
  }

  .status-msg {
    margin-top: 20px;
    padding: 12px 16px;
    border-radius: 12px;
    font-size: 13px;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .status-msg.success {
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.2);
    color: #4ade80;
  }

  .status-msg.error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.2);
    color: #f87171;
  }

  .status-icon {
    font-weight: bold;
  }

  @media (max-width: 560px) {
    .provider-grid {
      grid-template-columns: 1fr;
    }

    .settings-actions,
    .main-actions {
      align-items: stretch;
      flex-direction: column;
      width: 100%;
    }
  }
</style>
