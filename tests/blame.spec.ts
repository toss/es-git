import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('blame', () => {
  const signature1 = { name: 'Seokju Na', email: 'seokju.me@gmail.com' };
  const signature2 = { name: 'Seokju Me', email: 'seokju.me@toss.im' };

  it('should get blame information with correct authors', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    expect(blame.getHunkByLine(1).signature?.name).toBe(signature1.name);
    expect(blame.getHunkByLine(2).signature?.name).toBe(signature2.name);
    expect(blame.getHunkByLine(3).signature?.name).toBe(signature1.name);
    expect(blame.getHunkByLine(4).signature?.name).toBe(signature2.name);

    const hunks = blame.getHunks();
    expect(hunks.length).toBe(blame.getHunkCount());
  });

  it('should support line option', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const lineBlame = repo.blameFile('blame', { line: 2 });
    expect(lineBlame.getHunkByLine(2).signature?.name).toBe(signature2.name);
    expect(() => lineBlame.getHunkByLine(1)).toThrow();
  });

  it('should support range option', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const rangeBlame = repo.blameFile('blame', { range: [2, 4] });
    expect(rangeBlame.getHunkByLine(2).signature?.name).toBe(signature2.name);
    expect(rangeBlame.getHunkByLine(3).signature?.name).toBe(signature1.name);
    expect(rangeBlame.getHunkByLine(4).signature?.name).toBe(signature2.name);
    expect(() => rangeBlame.getHunkByLine(1)).toThrow();
  });

  it('should support oldest/newest commit options', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const secondCommitSha = 'a6c10f4d68f91e51c9a1e664e4b1efa0d2265edd';
    const oldestBlame = repo.blameFile('blame', { oldestCommit: secondCommitSha });
    expect(oldestBlame.getHunkByLine(2).signature?.name).toBe(signature2.name);
    expect(oldestBlame.getHunkByLine(4).signature?.name).toBe(signature2.name);

    const firstCommitSha = '2021365d09de3644ffe28c2f332a43ba129ead75';
    const newestBlame = repo.blameFile('blame', { newestCommit: firstCommitSha });
    expect(newestBlame.getHunkByLine(2).signature?.name).toBe(signature1.name);

    const standardBlame = repo.blameFile('blame');
    expect(newestBlame.getHunkCount()).not.toEqual(standardBlame.getHunkCount());

    const combinedBlame = repo.blameFile('blame', {
      oldestCommit: secondCommitSha,
      newestCommit: firstCommitSha,
    });
    expect(combinedBlame.getHunkCount()).toBeLessThanOrEqual(standardBlame.getHunkCount());
  });

  it('should throw for non-existent file', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    expect(() => repo.blameFile('non-existent-file')).toThrow();
  });
});
