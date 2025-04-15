import { describe, expect, it } from 'vitest';
import { type BlameHunk, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('blame', () => {
  it('should provide blame information with signature', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const blame = repo.blameFile('blame');
    const hunks = blame.getHunks();
    expect(hunks.length).toBeGreaterThan(0);

    const line1Hunk = blame.getHunkByLine(1);
    expect(line1Hunk.commitId).toBeTruthy();
    expect(line1Hunk.finalStartLineNumber).toBe(1);
    expect(line1Hunk.signature?.name).toBe('Seokju Na');
    expect(line1Hunk.signature?.email).toBe('seokju.na@gmail.com');

    const line2Hunk = blame.getHunkByLine(2);
    expect(line2Hunk.signature?.name).toBe('Seokju Me');
    expect(line2Hunk.signature?.email).toBe('seokju.na@toss.im');

    const hunkByIndex = blame.getHunkByIndex(0);
    expect(hunkByIndex.commitId).toBeTruthy();

    const buffer = Buffer.from('Line 1\nblah blah blah\nLine 3\nLine 4\n');
    const bufferBlame = blame.buffer(buffer, buffer.length);
    expect(bufferBlame.getHunkCount()).toBeGreaterThan(0);
  });

  it('should handle special files and error cases', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const emptyBlame = repo.blameFile('empty_file');
    expect(emptyBlame.getHunkCount()).toBe(1);
    expect(() => emptyBlame.getHunkByLine(1)).toThrow();

    const binaryBlame = repo.blameFile('binary_file');
    expect(binaryBlame.getHunkCount()).toBeGreaterThan(0);

    const blame = repo.blameFile('blame');
    expect(() => blame.getHunkByLine(0)).toThrow();
    expect(() => blame.getHunkByLine(9999)).toThrow();
    expect(() => blame.getHunkByIndex(9999)).toThrow();
    expect(() => repo.blameFile('non_existent_file')).toThrow();
  });

  it('should support blame options', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const lineBlame = repo.blameFile('blame', { minLine: 2, maxLine: 2 });
    expect(lineBlame.getHunkCount()).toBe(1);
    const hunk = lineBlame.getHunkByIndex(0);
    expect(hunk.finalStartLineNumber).toBe(2);
    expect(hunk.signature?.name).toBe('Seokju Me');

    const rangeBlame = repo.blameFile('blame', { minLine: 1, maxLine: 3 });
    expect(rangeBlame.getHunks().length).toBeGreaterThan(0);
    expect(rangeBlame.getHunks().length).toBeLessThanOrEqual(3);
    expect(() => rangeBlame.getHunkByLine(4)).toThrow();

    const advancedBlame = repo.blameFile('blame', {
      minLine: 1,
      maxLine: 5,
      firstParent: true,
      ignoreWhitespace: true,
      trackCopiesAnyCommitCopies: true,
      useMailmap: true,
    });
    expect(advancedBlame.getHunkCount()).toBeGreaterThan(0);
  });

  it('should support additional methods for working with hunks', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    expect(blame.isEmpty()).toBe(false);

    const buffer = Buffer.from(' ');
    const emptyBlame = blame.buffer(buffer, buffer.length);
    expect(emptyBlame.getHunkCount()).toBeGreaterThan(0);

    const lineHunks = blame.getHunksByLine();
    expect(lineHunks.length).toBeGreaterThan(0);

    const collectedHunks: BlameHunk[] = [];
    blame.forEachHunk((hunk, index) => {
      expect(index).toBe(collectedHunks.length);
      collectedHunks.push(hunk);
      return true;
    });
    expect(collectedHunks.length).toBe(blame.getHunkCount());

    const limitedHunks: BlameHunk[] = [];
    blame.forEachHunk((hunk, index) => {
      limitedHunks.push(hunk);
      return index < 2;
    });
    expect(limitedHunks.length).toBe(3);
  });
});
