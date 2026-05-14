export const WATERMARK_REGEX = /^[A-Za-z]{2}[0-9]{4}$/;

export interface WatermarkedPayloadV1 {
  qruPayloadVersion: 1;
  payload: string;
  watermark: string;
}

export function isValidWatermark(value: string): boolean {
  return WATERMARK_REGEX.test(value);
}

export function normalizeWatermarkInput(value: string): string {
  return value.trim();
}

export function generateWatermark(): string {
  const letters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';
  const digits = '0123456789';
  
  const array = new Uint8Array(6);
  crypto.getRandomValues(array);
  
  let result = '';
  // Two letters
  result += letters[array[0] % letters.length];
  result += letters[array[1] % letters.length];
  // Four digits
  result += digits[array[2] % digits.length];
  result += digits[array[3] % digits.length];
  result += digits[array[4] % digits.length];
  result += digits[array[5] % digits.length];
  
  return result;
}

export function wrapPayloadWithWatermark(payload: string, watermark?: string): string {
  if (!watermark || !isValidWatermark(watermark)) {
    return payload;
  }
  
  const wrapped: WatermarkedPayloadV1 = {
    qruPayloadVersion: 1,
    payload,
    watermark
  };
  
  return JSON.stringify(wrapped);
}

export function unwrapWatermarkedPayload(decrypted: string): { payload: string; watermark: string | null; isWatermarked: boolean } {
  try {
    const parsed = JSON.parse(decrypted);
    if (
      parsed &&
      parsed.qruPayloadVersion === 1 &&
      typeof parsed.payload === 'string' &&
      typeof parsed.watermark === 'string'
    ) {
      return {
        payload: parsed.payload,
        watermark: parsed.watermark,
        isWatermarked: true
      };
    }
  } catch (e) {
    // Not JSON or doesn't match the schema, treat as legacy
  }
  
  return {
    payload: decrypted,
    watermark: null,
    isWatermarked: false
  };
}
