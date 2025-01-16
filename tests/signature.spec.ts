import { describe, expect, it } from 'vitest';
import { createSignature } from '../index';

describe('signature', () => {
  it('create signature', () => {
    const sig = createSignature('Seokju Na', 'seokju.me@toss.im');
    expect(sig.name).toEqual('Seokju Na');
    expect(sig.email).toEqual('seokju.me@toss.im');
  });

  it('create signature with timestamp', () => {
    const sig = createSignature('Seokju Na', 'seokju.me@toss.im', {
      timestamp: 1734280068,
    });
    expect(sig.timestamp).toEqual(1734280068);
  });
});
