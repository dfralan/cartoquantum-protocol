// src/lib/cartoquantum.ts
import pako from 'pako';

export interface Envelope {
  envelope: {
    protocol: string;
    version: number;
    encoding: string;
    issued_at: number;
    sig?: string;
  };
  object: {
    kind: string;
    coords: Record<string, any>;
    ops: string[];
    expires?: number;
    payload: any;
  };
}

export function encodeEnvelope(obj: Envelope['object']): string {
  const envelope: Envelope = {
    envelope: {
      protocol: 'cartoquantum',
      version: 1,
      encoding: 'deflate+base64url',
      issued_at: Math.floor(Date.now() / 1000),
    },
    object: obj,
  };
  const json = JSON.stringify(envelope);
  const compressed = pako.deflate(json);
  return Buffer.from(compressed).toString('base64url');
}

export function decodeEnvelope(data: string): Envelope {
  const compressed = Buffer.from(data, 'base64url');
  const json = pako.inflate(compressed, { to: 'string' });
  return JSON.parse(json);
}
