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
    background-color: #0F0F12;
    background-attachment: fixed;
  }

  .settings-page {
    padding: 20px;
    width: 100%;
    min-height: 100vh;
    box-sizing: border-box;
    color: #F7F8FF;
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
    border: 1px solid rgba(255, 255, 255, 0.14);
    color: #fff;
    padding: 10px 18px;
    border-radius: 14px;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 700;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: all 0.2s;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .back-btn:active {
    transform: translateX(-4px);
  }

  .settings-header h2 {
    font-size: 24px;
    font-weight: 900;
    margin: 0;
    background: linear-gradient(135deg, #21d4fd 0%, #b721ff 100%);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .glass-panel {
    background: #18181F;
    border: 1px solid #2A2A33;
    border-radius: 24px;
    padding: 30px;
    box-shadow: 0 20px 60px rgba(0,0,0,0.42);
    position: relative;
    overflow: hidden;
  }

  .section-header {
    display: flex;
    align-items: center;
    gap: 15px;
    margin-bottom: 15px;
  }

  .section-icon {
    font-size: 24px;
    filter: drop-shadow(0 0 8px #21d4fd);
  }

  .settings-section h3 {
    margin: 0;
    font-size: 1.2rem;
    font-weight: 800;
    color: #fff;
  }

  .eyebrow {
    margin: 4px 0 0;
    color: #21d4fd;
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.15em;
    font-weight: 900;
    text-shadow: 0 0 10px rgba(33, 212, 253, 0.3);
  }

  .description {
    color: rgba(247, 248, 255, 0.68);
    font-size: 0.9rem;
    line-height: 1.6;
    margin-bottom: 25px;
  }

  .provider-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 12px;
    margin-bottom: 22px;
  }

  .provider-card {
    min-width: 0;
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-radius: 14px;
    padding: 15px;
    background: rgba(255, 255, 255, 0.055);
    color: #fff;
    text-align: left;
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .provider-card:hover {
    background: rgba(255, 255, 255, 0.095);
    border-color: rgba(33, 212, 253, 0.38);
  }

  .provider-card.active {
    border-color: #21d4fd;
    background: rgba(33, 212, 253, 0.1);
    box-shadow: 0 0 20px rgba(33, 212, 253, 0.15);
  }

  .provider-card span {
    font-weight: 800;
    font-size: 1rem;
    display: block;
  }

  .provider-card small {
    display: block;
    margin-top: 5px;
    color: rgba(247, 248, 255, 0.48);
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 22px;
  }

  .input-group label {
    font-size: 0.8rem;
    font-weight: 800;
    color: rgba(247, 248, 255, 0.68);
    margin-left: 4px;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .password-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .text-input {
    width: 100%;
    box-sizing: border-box;
    background: rgba(255, 255, 255, 0.055);
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-radius: 14px;
    padding: 14px 16px;
    padding-right: 80px;
    color: #fff;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.95rem;
    transition: all 0.25s ease;
    outline: none;
  }

  .text-input:focus {
    border-color: #21d4fd;
    background: rgba(255, 255, 255, 0.095);
    box-shadow: 0 0 15px rgba(33, 212, 253, 0.15);
  }

  .toggle-visibility {
    position: absolute;
    right: 12px;
    background: rgba(255, 255, 255, 0.095);
    border: 1px solid rgba(255, 255, 255, 0.14);
    color: rgba(247, 248, 255, 0.68);
    padding: 6px 10px;
    border-radius: 8px;
    font-size: 10px;
    font-weight: 900;
    cursor: pointer;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .link-btn {
    background: none;
    border: none;
    color: #21d4fd;
    font-size: 0.75rem;
    text-align: left;
    cursor: pointer;
    padding: 0;
    margin-left: 4px;
    text-decoration: underline;
    font-weight: 700;
  }

  .provider-status-row {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    margin-bottom: 20px;
  }

  .provider-pill {
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-radius: 999px;
    padding: 6px 14px;
    color: rgba(247, 248, 255, 0.48);
    background: rgba(255, 255, 255, 0.055);
    font-size: 0.7rem;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .provider-pill.ready {
    color: #00FF88;
    border-color: rgba(0, 255, 136, 0.3);
    background: rgba(0, 255, 136, 0.1);
    text-shadow: 0 0 8px rgba(0, 255, 136, 0.4);
  }

  .settings-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 30px;
    gap: 15px;
    flex-wrap: wrap;
  }

  .main-actions {
    display: flex;
    gap: 12px;
  }

  .action-btn {
    padding: 12px 24px;
    border-radius: 14px;
    font-size: 0.9rem;
    font-weight: 800;
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    border: none;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .test-btn {
    background: rgba(255, 255, 255, 0.095);
    border: 1px solid rgba(255, 255, 255, 0.14);
    color: #fff;
  }

  .clear-btn {
    background: rgba(255, 26, 146, 0.1);
    border: 1px solid rgba(255, 26, 146, 0.3);
    color: #ff1a92;
  }

  .save-btn {
    color: white;
    background: linear-gradient(135deg, #21d4fd 0%, #b721ff 100%);
    box-shadow: 0 4px 15px rgba(183, 33, 255, 0.3);
  }
  .save-btn:hover {
    box-shadow: 0 6px 20px rgba(183, 33, 255, 0.4);
  }

  .status-msg {
    margin-top: 20px;
    padding: 14px 18px;
    border-radius: 14px;
    font-size: 0.9rem;
    display: flex;
    align-items: center;
    gap: 12px;
    font-weight: 600;
  }

  .status-msg.success {
    background: rgba(33, 212, 253, 0.1);
    border: 1px solid rgba(33, 212, 253, 0.3);
    color: #21d4fd;
  }

  .status-msg.error {
    background: rgba(255, 26, 146, 0.1);
    border: 1px solid rgba(255, 26, 146, 0.3);
    color: #ff1a92;
  }

  .status-icon {
    font-weight: 900;
    font-size: 1.1rem;
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
