import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('blame', () => {
  it('should provide blame information with signature', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const blame = repo.blameFile('blame');
    const hunks = blame.getHunks();
    expect(hunks.length).toBeGreaterThan(0);

    const hunkByLine = blame.getHunkByLine(2);
    expect(hunkByLine.commitId).toBeTruthy();
    expect(hunkByLine.finalStartLineNumber).toBe(2);
    expect(hunkByLine.signature?.name).toBeTruthy();
    expect(hunkByLine.signature?.email).toBeTruthy();

    const hunkByIndex = blame.getHunkByIndex(0);
    expect(hunkByIndex.commitId).toBeTruthy();

    const buffer = Buffer.from('Line 1\nblah blah blah\nLine 3\nLine 4\n');
    const bufferBlame = blame.buffer(buffer, buffer.length);
    expect(bufferBlame.getHunkCount()).toBeGreaterThan(0);

    for (let i = 0; i < bufferBlame.getHunkCount(); i++) {
      const bufferHunk = bufferBlame.getHunkByIndex(i);
      expect(bufferHunk.commitId).toBeTruthy();
      if (bufferHunk.commitId !== '0000000000000000000000000000000000000000') {
        expect(bufferHunk.signature).toBeDefined();
      }
    }
  });

  it('should throw for invalid resources', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    expect(() => blame.getHunkByLine(9999)).toThrow();
    expect(() => blame.getHunkByIndex(9999)).toThrow();
  });
});
