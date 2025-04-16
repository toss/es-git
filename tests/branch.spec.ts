import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('branch', () => {
  it('get branch', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const branch = repo.getBranch('main', 'Local');
    expect(branch.name()).toEqual('main');
  });

  it('find branch', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const branch = repo.findBranch('not_exists', 'Local');
    expect(branch).toBe(null);
  });

  it('create branch', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const target = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    const branch = repo.createBranch('other', target);
    expect(branch.name()).toEqual('other');
  });

  it('set upstream', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const branch = repo.getBranch('main', 'Local');
    branch.setUpstream('main');
    expect(branch.getUpstream().name()).toEqual('main');
  });

  it('rename branch', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const branch = repo.getBranch('main', 'Local');
    const newBranch = branch.rename('other');
    expect(newBranch.name()).toEqual('other');
  });

  it('get branch reference target', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const branch = repo.getBranch('main', 'Local');
    expect(branch.referenceTarget()).toEqual('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
  });

  it('throws error if renaming branch already exists', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const target = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(() => repo.createBranch('main', target)).toThrowError(
      /libgit2 error: failed to write reference 'refs\/heads\/main': a reference with that name already exists/
    );
  });

  it('rename branch with force option', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const target = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    const branch = repo.createBranch('other', target);
    const newBranch = branch.rename('main', { force: true });
    expect(newBranch.name()).toEqual('main');
  });

  it('iterates branches', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const target = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    repo.createBranch('other1', target);
    repo.createBranch('other2', target);
    expect([...repo.branches()]).toEqual(
      expect.arrayContaining([
        { type: 'Local', name: 'main' },
        { type: 'Local', name: 'other1' },
        { type: 'Local', name: 'other2' },
      ])
    );
  });
});
