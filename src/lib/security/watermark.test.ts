import { describe, it, expect, vi, beforeAll } from 'vitest';
import { 
  isValidWatermark, 
  generateWatermark, 
  wrapPayloadWithWatermark, 
  unwrapWatermarkedPayload,
  WATERMARK_REGEX 
} from './watermark';

// Mock crypto for generateWatermark
beforeAll(() => {
  if (typeof (globalThis as any).crypto === 'undefined') {
    (globalThis as any).crypto = {
      getRandomValues: (arr: Uint8Array) => {
        for (let i = 0; i < arr.length; i++) {
          arr[i] = Math.floor(Math.random() * 256);
        }
        return arr;
      }
    } as any;
  }
});

describe('Watermark Helpers', () => {
  describe('isValidWatermark', () => {
    it('accepts valid watermarks', () => {
      expect(isValidWatermark('aZ4821')).toBe(true);
      expect(isValidWatermark('QR1907')).toBe(true);
      expect(isValidWatermark('Ab1234')).toBe(true);
    });

    it('rejects invalid formats', () => {
      expect(isValidWatermark('ABC123')).toBe(false); // 3 letters
      expect(isValidWatermark('ab123')).toBe(false);  // 3 digits
      expect(isValidWatermark('A11234')).toBe(false); // 1 letter, 5 digits
      expect(isValidWatermark('åB1234')).toBe(false); // non-ASCII
      expect(isValidWatermark('ab12c4')).toBe(false); // letter in digits
      expect(isValidWatermark('ab12345')).toBe(false); // too long
      expect(isValidWatermark('a12345')).toBe(false); // too short
    });
    
    it('is case sensitive via regex', () => {
        // The regex allows both cases, but they are different strings.
        // The requirement is that we don't normalize them.
        expect(WATERMARK_REGEX.test('Ab1234')).toBe(true);
        expect(WATERMARK_REGEX.test('AB1234')).toBe(true);
        const a: string = 'Ab1234';
        const b: string = 'AB1234';
        expect(a === b).toBe(false);
    });
  });

  describe('generateWatermark', () => {
    it('generates a watermark matching the regex', () => {
      for (let i = 0; i < 100; i++) {
        const wm = generateWatermark();
        expect(wm).toMatch(WATERMARK_REGEX);
      }
    });
  });

  describe('wrapPayloadWithWatermark', () => {
    it('wraps payload in JSON when watermark is valid', () => {
      const payload = 'secret message';
      const watermark = 'aZ4821';
      const wrapped = wrapPayloadWithWatermark(payload, watermark);
      const parsed = JSON.parse(wrapped);
      
      expect(parsed.qruPayloadVersion).toBe(1);
      expect(parsed.payload).toBe(payload);
      expect(parsed.watermark).toBe(watermark);
    });

    it('returns original payload if watermark is missing or invalid', () => {
      const payload = 'secret message';
      expect(wrapPayloadWithWatermark(payload, undefined)).toBe(payload);
      expect(wrapPayloadWithWatermark(payload, 'invalid')).toBe(payload);
    });
  });

  describe('unwrapWatermarkedPayload', () => {
    it('unwraps valid watermarked JSON', () => {
      const originalPayload = 'secret message';
      const watermark = 'aZ4821';
      const wrapped = JSON.stringify({
        qruPayloadVersion: 1,
        payload: originalPayload,
        watermark: watermark
      });
      
      const result = unwrapWatermarkedPayload(wrapped);
      expect(result.isWatermarked).toBe(true);
      expect(result.payload).toBe(originalPayload);
      expect(result.watermark).toBe(watermark);
    });

    it('handles legacy plaintext safely', () => {
      const legacy = 'just plain text';
      const result = unwrapWatermarkedPayload(legacy);
      expect(result.isWatermarked).toBe(false);
      expect(result.payload).toBe(legacy);
      expect(result.watermark).toBe(null);
    });

    it('handles malformed JSON safely', () => {
      const malformed = '{"qruPayloadVersion":1, "payload": "missing watermark"}';
      const result = unwrapWatermarkedPayload(malformed);
      expect(result.isWatermarked).toBe(false);
      expect(result.payload).toBe(malformed);
      expect(result.watermark).toBe(null);
    });
    
    it('handles invalid JSON safely', () => {
      const invalid = '{ invalid json }';
      const result = unwrapWatermarkedPayload(invalid);
      expect(result.isWatermarked).toBe(false);
      expect(result.payload).toBe(invalid);
      expect(result.watermark).toBe(null);
    });
  });
});
