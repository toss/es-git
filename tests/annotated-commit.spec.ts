import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('annotated commit', () => {
  it('create from commit', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const commit = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    const annotatedCommit = repo.getAnnotatedCommit(commit);
    expect(annotatedCommit.id()).toEqual('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(annotatedCommit.refname()).toBeNull();
  });

  it('create from reference', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const reference = repo.getReference('refs/heads/main');
    const annotatedCommit = repo.getAnnotatedCommitFromReference(reference);
    expect(annotatedCommit.id()).toEqual('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(annotatedCommit.refname()).toEqual('refs/heads/main');
  });
});
