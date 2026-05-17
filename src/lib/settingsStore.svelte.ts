import { browser } from '$app/environment';

export type AiProvider = 'groq' | 'gemini' | 'deepseek' | 'openrouter';

export type AiProviderKeys = Record<AiProvider, string>;

const providerKeys: Record<AiProvider, string> = {
    groq: 'groq_api_key',
    gemini: 'gemini_api_key',
    deepseek: 'deepseek_api_key',
    openrouter: 'openrouter_api_key'
};

const providerStorageKey = 'qr_studio_ai_provider';
const dynamicQrBaseUrlKey = 'qr_studio_dynamic_qr_base_url';
const dynamicQrApiKeyKey = 'qr_studio_dynamic_qr_api_key';

const defaultProvider: AiProvider = 'groq';
const defaultDynamicQrBaseUrl = 'https://qrstudio.shadowbourne.org/qru-dynamic';
const defaultDynamicQrApiKey = 'QRU_LOCAL_DEV_KEY_CHANGE_ME';

const providers: AiProvider[] = ['groq', 'gemini', 'deepseek', 'openrouter'];

function readStorage(key: string) {
    return browser ? localStorage.getItem(key) || '' : '';
}

function normalizeProvider(value: string | null): AiProvider {
    return providers.includes(value as AiProvider) ? value as AiProvider : defaultProvider;
}

function createSettingsStore() {
    let preferredAiProvider = $state<AiProvider>(normalizeProvider(readStorage(providerStorageKey)));
    let apiKeys = $state<AiProviderKeys>({
        groq: readStorage(providerKeys.groq),
        gemini: readStorage(providerKeys.gemini),
        deepseek: readStorage(providerKeys.deepseek),
        openrouter: readStorage(providerKeys.openrouter)
    });

    let dynamicQrBaseUrl = $state<string>(readStorage(dynamicQrBaseUrlKey) || defaultDynamicQrBaseUrl);
    let dynamicQrApiKey = $state<string>(readStorage(dynamicQrApiKeyKey) || defaultDynamicQrApiKey);

    return {
        get preferredAiProvider() { return preferredAiProvider; },
        get aiApiKeys() { return apiKeys; },
        get groqApiKey() { return apiKeys.groq; },
        get dynamicQrBaseUrl() { return dynamicQrBaseUrl; },
        get dynamicQrApiKey() { return dynamicQrApiKey; },
        getProviderKey(provider: AiProvider) {
            return apiKeys[provider];
        },
        setPreferredAiProvider(provider: AiProvider) {
            preferredAiProvider = provider;
            if (browser) {
                localStorage.setItem(providerStorageKey, provider);
            }
        },
        setDynamicQrConfig(baseUrl: string, apiKey: string) {
            dynamicQrBaseUrl = baseUrl;
            dynamicQrApiKey = apiKey;
            if (browser) {
                localStorage.setItem(dynamicQrBaseUrlKey, baseUrl);
                localStorage.setItem(dynamicQrApiKeyKey, apiKey);
            }
        },
        setProviderApiKey(provider: AiProvider, key: string) {
            apiKeys = { ...apiKeys, [provider]: key };
            if (browser) {
                const storageKey = providerKeys[provider];
                if (key.trim()) {
                    localStorage.setItem(storageKey, key);
                } else {
                    localStorage.removeItem(storageKey);
                }
            }
        },
        clearProviderApiKey(provider: AiProvider) {
            apiKeys = { ...apiKeys, [provider]: '' };
            if (browser) {
                localStorage.removeItem(providerKeys[provider]);
            }
        },
        setGroqApiKey(key: string) {
            this.setProviderApiKey('groq', key);
        },
        clearGroqApiKey() {
            this.clearProviderApiKey('groq');
        }
    };
}

export const settingsStore = createSettingsStore();
