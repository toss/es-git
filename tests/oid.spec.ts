import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { ObjectType, hashFileOid, hashObjectOid, isValidOid, isZeroOid, zeroOid } from '../index';
import { useFixture } from './fixtures';

describe('oid', () => {
  it('check is valid oid', () => {
    expect(isValidOid('')).toBe(false);
    expect(isValidOid('가나다')).toBe(false);
    expect(isValidOid('123')).toBe(true);
    expect(isValidOid('abc')).toBe(true);
    expect(isValidOid('ABC')).toBe(true);
    expect(isValidOid('37124c3d9881d6eaa0d423729f634c51262d3da5')).toBe(true);
    expect(isValidOid('37124c3d9881d6eaa0d423729f634c51262d3da537124c3d9881d6eaa0d423729f634c51262d3da5')).toBe(false);
  });

  it('check is zero oid', () => {
    expect(isZeroOid('가나다')).toBe(false);
    expect(isZeroOid('37124c3')).toBe(false);
    expect(isZeroOid('0')).toBe(true);
    expect(isZeroOid('0000000')).toBe(true);
    expect(isZeroOid('0000000000000000000000000000000000000000')).toBe(true);
  });

  it('get zero oid', () => {
    expect(zeroOid()).toEqual('0000000000000000000000000000000000000000');
  });

  it('get hash object oid', () => {
    expect(hashObjectOid(ObjectType.Blob, Buffer.from('hello world'))).toHaveLength(40);
  });

  it('get hash file oid', async () => {
    const p = await useFixture('commits');
    expect(hashFileOid(ObjectType.Blob, path.join(p, 'first'))).toHaveLength(40);
  });
});
