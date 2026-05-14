<script lang="ts">
  import { 
    isValidWatermark, 
    generateWatermark, 
    normalizeWatermarkInput 
  } from '$lib/security/watermark';

  interface Props {
    enabled: boolean;
    value: string;
    onToggle: (enabled: boolean) => void;
    onValueChange: (value: string) => void;
  }

  let { enabled, value, onToggle, onValueChange }: Props = $props();

  let localError = $derived(enabled && value && !isValidWatermark(value) 
    ? "Invalid format: 2 letters + 4 digits" 
    : "");

  function handleGenerate() {
    const newVal = generateWatermark();
    onValueChange(newVal);
  }

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    const normalized = normalizeWatermarkInput(target.value);
    onValueChange(normalized);
  }
</script>

<div class="watermark-control">
  <div class="field-head">
    <div class="toggle-row">
      <input 
        type="checkbox" 
        id="wm-toggle" 
        checked={enabled} 
        onchange={(e) => onToggle((e.target as HTMLInputElement).checked)}
      />
      <label for="wm-toggle">Attach steganographic watermark</label>
    </div>
    <button class="help-dot" type="button" title="Adds a compact traceability marker authenticated inside the encrypted payload.">?</button>
  </div>

  {#if enabled}
    <div class="wm-input-group">
      <div class="input-row">
        <input
          type="text"
          value={value}
          oninput={handleInput}
          placeholder="aZ4821"
          maxlength="6"
          class="wm-input"
          class:error={!!localError}
        />
        <button class="mini-gen-btn" type="button" onclick={handleGenerate}>
          Generate
        </button>
      </div>
      
      {#if localError}
        <p class="wm-error">{localError}</p>
      {/if}
      
      <p class="helper-text">
        Two case-sensitive letters + four digits. Authenticated inside the encrypted QR.
      </p>
    </div>
  {/if}
</div>

<style>
  .watermark-control {
    margin-top: 15px;
    padding: 12px;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 10px;
  }

  .field-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .toggle-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.85rem;
    color: #ccc;
    cursor: pointer;
  }

  .toggle-row input {
    cursor: pointer;
    accent-color: #4dffb3;
  }

  .toggle-row label {
    cursor: pointer;
    user-select: none;
  }

  .wm-input-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 10px;
  }

  .input-row {
    display: flex;
    gap: 8px;
  }

  .wm-input {
    flex: 1;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #fff;
    padding: 8px 12px;
    border-radius: 6px;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.9rem;
    outline: none;
    transition: all 0.2s;
  }

  .wm-input:focus {
    border-color: rgba(77, 255, 179, 0.4);
    box-shadow: 0 0 8px rgba(77, 255, 179, 0.1);
  }

  .wm-input.error {
    border-color: #ff4d4d;
  }

  .mini-gen-btn {
    background: rgba(77, 255, 179, 0.1);
    border: 1px solid rgba(77, 255, 179, 0.3);
    color: #4dffb3;
    padding: 0 12px;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .mini-gen-btn:hover {
    background: rgba(77, 255, 179, 0.2);
    border-color: #4dffb3;
  }

  .wm-error {
    color: #ff4d4d;
    font-size: 0.75rem;
    margin: 0;
  }

  .helper-text {
    font-size: 0.75rem;
    color: #888;
    margin: 0;
    line-height: 1.3;
  }

  .help-dot {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.1);
    border: none;
    color: #888;
    font-size: 0.7rem;
    cursor: help;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }

  .help-dot:hover {
    background: rgba(255, 255, 255, 0.2);
    color: #fff;
  }
</style>
