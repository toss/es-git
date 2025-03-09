import { describe, expect, it } from 'vitest';
import { ReferenceFormat, isValidReferenceName, normalizeReferenceName, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('reference', () => {
  it('check is valid reference name', () => {
    expect(isValidReferenceName('HEAD')).toBe(true);
    expect(isValidReferenceName('refs/heads/main')).toBe(true);
    expect(isValidReferenceName('main')).toBe(false);
    expect(isValidReferenceName('refs/heads/*')).toBe(false);
    expect(isValidReferenceName('foo//bar')).toBe(false);
  });

  it('normalize reference name', () => {
    expect(normalizeReferenceName('foo//bar')).toEqual('foo/bar');
    expect(normalizeReferenceName('HEAD', ReferenceFormat.AllowOnelevel)).toEqual('HEAD');
    expect(normalizeReferenceName('refs/heads/*', ReferenceFormat.RefspecPattern)).toEqual('refs/heads/*');
    expect(normalizeReferenceName('main', ReferenceFormat.AllowOnelevel | ReferenceFormat.RefspecShorthand)).toEqual(
      'main'
    );
  });

  it('get symbolic reference', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const ref = repo.getReference('HEAD');
    expect(ref.type()).toEqual('Symbolic');
    expect(ref.target()).toBeNull();
    expect(ref.isBranch()).toBe(false);
    expect(ref.symbolicTarget()).toEqual('refs/heads/main');
  });

  it('resolve symbolic reference', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const ref = repo.getReference('HEAD').resolve();
    expect(ref.name()).toEqual('refs/heads/main');
    expect(ref.shorthand()).toEqual('main');
    expect(ref.type()).toEqual('Direct');
    expect(ref.target()).toEqual('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(ref.isBranch()).toBe(true);
    expect(ref.symbolicTarget()).toBeNull();
  });

  it('rename reference', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const ref = repo.getReference('HEAD').resolve();
    ref.rename('refs/heads/main2');
    const newRef = repo.getReference('refs/heads/main2');
    expect(newRef.target()).toEqual('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
  });

  it('returns null if reference does not exists', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    expect(repo.findReference('UNKNOWN')).toBe(null);
    expect(repo.findReference('refs/heads/*')).toBe(null);
    expect(repo.findReference('foo//bar')).toBe(null);
    expect(repo.findReference('HEAD1')).toBe(null);
  });
});
