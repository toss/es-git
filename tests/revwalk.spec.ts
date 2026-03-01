import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('revwalk', () => {
  it('list commits from head', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const revwalk = repo.revwalk();
    revwalk.pushHead();
    expect(revwalk.next()).toEqual('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(revwalk.next()).toEqual('b33e0101b828225f77eeff4dfa31259dcf379002');
    expect(revwalk.next()).toBeNull();
  });

  it('list commits from specific oid', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const revwalk = repo.revwalk();
    revwalk.push('b33e0101b828225f77eeff4dfa31259dcf379002');
    expect(revwalk.next()).toEqual('b33e0101b828225f77eeff4dfa31259dcf379002');
    expect(revwalk.next()).toBeNull();
  });

  it('list commits from range', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const revwalk = repo.revwalk();
    revwalk.pushRange('b33e010..a01e988');
    expect(revwalk.next()).toEqual('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(revwalk.next()).toBeNull();
  });

  it('list commits from reference', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const revwalk = repo.revwalk();
    revwalk.pushRef('refs/heads/main');
    expect(revwalk.next()).toEqual('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(revwalk.next()).toEqual('b33e0101b828225f77eeff4dfa31259dcf379002');
    expect(revwalk.next()).toBeNull();
  });

  it('hide specific oid', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const revwalk = repo.revwalk();
    revwalk.pushHead().hide('b33e0101b828225f77eeff4dfa31259dcf379002');
    expect(revwalk.next()).toEqual('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(revwalk.next()).toBeNull();
  });
});
