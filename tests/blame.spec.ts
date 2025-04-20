import { describe, expect, it } from 'vitest';
import { type BlameHunk, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('blame', () => {
  it('should provide blame information with signature', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const blame = repo.blameFile('blame');
    const hunks = [...blame.iter()];
    expect(hunks.length).toBeGreaterThan(0);

    const line1Hunk = blame.getHunkByLine(1);
    expect(line1Hunk.finalCommitId).toBeTruthy();
    expect(line1Hunk.finalStartLineNumber).toBe(1);
    expect(line1Hunk.finalSignature?.name).toBe('Seokju Na');
    expect(line1Hunk.finalSignature?.email).toBe('seokju.na@gmail.com');

    const line2Hunk = blame.getHunkByLine(2);
    expect(line2Hunk.finalSignature?.name).toBe('Seokju Me');
    expect(line2Hunk.finalSignature?.email).toBe('seokju.na@toss.im');

    const hunkByIndex = blame.getHunkByIndex(0);
    expect(hunkByIndex.finalCommitId).toBeTruthy();

    const buffer = Buffer.from('Line 1\nLine 2\nLine 3\n');
    const bufferBlame = blame.buffer(buffer);
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

    const specialBuffer = Buffer.from('Line 1\nblah blah blah\nLine 3\nLine 4\n');
    const specialBlame = blame.buffer(specialBuffer);

    const zeroCommitHunk = specialBlame.getHunkByIndex(1);
    expect(zeroCommitHunk.finalCommitId).toBe('0000000000000000000000000000000000000000');
    expect(zeroCommitHunk.finalSignature).toBeUndefined();
    expect(zeroCommitHunk.origSignature).toBeUndefined();
  });

  it('should support blame options', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const lineBlame = repo.blameFile('blame', { minLine: 2, maxLine: 2 });
    expect(lineBlame.getHunkCount()).toBe(1);
    const hunk = lineBlame.getHunkByIndex(0);
    expect(hunk.finalStartLineNumber).toBe(2);
    expect(hunk.finalSignature?.name).toBe('Seokju Me');

    const rangeBlame = repo.blameFile('blame', { minLine: 1, maxLine: 3 });
    const rangeHunks = [...rangeBlame.iter()];
    expect(rangeHunks.length).toBeGreaterThan(0);
    expect(rangeHunks.length).toBeLessThanOrEqual(3);
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
    const emptyBlame = blame.buffer(buffer);
    expect(emptyBlame.getHunkCount()).toBeGreaterThan(0);

    const lineHunks = [...blame.iterByLine()];
    expect(lineHunks.length).toBeGreaterThan(0);

    const collectedHunks: BlameHunk[] = [...blame.iter()];
    expect(collectedHunks.length).toBe(blame.getHunkCount());
  });
});
