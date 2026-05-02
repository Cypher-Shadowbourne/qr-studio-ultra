import { invoke } from '@tauri-apps/api/core';
import type { AiProvider, AiProviderKeys } from './settingsStore.svelte';

export const aiProviders: { id: AiProvider; label: string; model: string; keyUrl: string; placeholder: string }[] = [
  {
    id: 'groq',
    label: 'Groq',
    model: 'llama-3.3-70b-versatile',
    keyUrl: 'https://console.groq.com/keys',
    placeholder: 'gsk_...'
  },
  {
    id: 'gemini',
    label: 'Google Gemini',
    model: 'gemini-2.5-flash',
    keyUrl: 'https://aistudio.google.com/app/apikey',
    placeholder: 'AIza...'
  },
  {
    id: 'deepseek',
    label: 'DeepSeek',
    model: 'deepseek-chat',
    keyUrl: 'https://platform.deepseek.com/api_keys',
    placeholder: 'sk-...'
  },
  {
    id: 'openrouter',
    label: 'OpenRouter',
    model: 'openrouter/auto',
    keyUrl: 'https://openrouter.ai/settings/keys',
    placeholder: 'sk-or-v1-...'
  }
];

export type AiMagicDesign = {
  gradient_start: string;
  gradient_mid: string;
  gradient_end: string;
  ring_color: string;
  curved_text_top?: string | null;
  curved_text_bottom?: string | null;
  center_image_prompt: string;
  overall_mood: string;
  provider_used?: AiProvider;
  model_used?: string;
};

export function getProviderLabel(provider: AiProvider | string | undefined) {
  return aiProviders.find((item) => item.id === provider)?.label ?? 'AI Magic';
}

export function getFallbackOrder(preferredProvider: AiProvider) {
  const ids = aiProviders.map((provider) => provider.id);
  return [preferredProvider, ...ids.filter((provider) => provider !== preferredProvider)];
}

export async function generateAiMagicDesign(
  prompt: string,
  preferredProvider: AiProvider,
  apiKeys: AiProviderKeys
) {
  return invoke<AiMagicDesign>('ai_magic', {
    prompt,
    preferredProvider,
    apiKeys
  });
}

export async function testAiProvider(provider: AiProvider, apiKeys: AiProviderKeys) {
  return invoke<AiMagicDesign>('ai_magic', {
    prompt: 'test connection with compact valid QR Studio Ultra JSON',
    preferredProvider: provider,
    apiKeys: {
      groq: provider === 'groq' ? apiKeys.groq : '',
      gemini: provider === 'gemini' ? apiKeys.gemini : '',
      deepseek: provider === 'deepseek' ? apiKeys.deepseek : '',
      openrouter: provider === 'openrouter' ? apiKeys.openrouter : ''
    }
  });
}
