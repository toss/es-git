import { describe, expect, it } from 'vitest';
import { RevparseMode, openRepository, revparseModeContains } from '../index';
import { useFixture } from './fixtures';

describe('revparse', () => {
  it('revparse with single spec', async () => {
    const p = await useFixture('revparse');
    const repo = await openRepository(p);
    const spec = repo.revparse('HEAD');
    expect(spec.from).toEqual('ecec200b27c181b64545bd03f99e995768289d4f');
    expect(spec.to).toBeUndefined();
    expect(revparseModeContains(spec.mode, RevparseMode.Single)).toBe(true);
    expect(revparseModeContains(spec.mode, RevparseMode.Range)).toBe(false);
    expect(revparseModeContains(spec.mode, RevparseMode.Single | RevparseMode.Range)).toBe(false);
  });

  it('revparse with range spec', async () => {
    const p = await useFixture('revparse');
    const repo = await openRepository(p);
    const spec = repo.revparse('main..other');
    expect(spec.from).toEqual('ecec200b27c181b64545bd03f99e995768289d4f');
    expect(spec.to).toEqual('b580e5f5030f508a3658a4155f44cdd9754950c5');
    expect(revparseModeContains(spec.mode, RevparseMode.Single)).toBe(false);
    expect(revparseModeContains(spec.mode, RevparseMode.Range)).toBe(true);
  });

  it('revparse single', async () => {
    const p = await useFixture('revparse');
    const repo = await openRepository(p);
    const oid = repo.revparseSingle('HEAD');
    expect(oid).toEqual('ecec200b27c181b64545bd03f99e995768289d4f');
    expect(() => repo.revparseSingle('main..other')).toThrowError(/libgit2 error: failed to parse revision specifier/);
  });
});
