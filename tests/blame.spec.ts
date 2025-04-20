import { describe, expect, it } from 'vitest';
import { type BlameHunk, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('blame', () => {
  it('should retrieve basic blame information', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const blame = repo.blameFile('blame');
    const hunks = [...blame.iter()];
    expect(hunks.length).toBeGreaterThan(0);
  });

  it('should retrieve blame hunk by line number', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    const line1Hunk = blame.getHunkByLine(1);
    expect(line1Hunk.finalCommitId).toBeTruthy();
    expect(line1Hunk.finalStartLineNumber).toBe(1);
    expect(line1Hunk.finalSignature?.name).toBe('Seokju Na');
    expect(line1Hunk.finalSignature?.email).toBe('seokju.na@gmail.com');

    const line2Hunk = blame.getHunkByLine(2);
    expect(line2Hunk.finalSignature?.name).toBe('Seokju Me');
    expect(line2Hunk.finalSignature?.email).toBe('seokju.na@toss.im');
  });

  it('should retrieve blame hunk by index', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    const hunkByIndex = blame.getHunkByIndex(0);
    expect(hunkByIndex.finalCommitId).toBeTruthy();
  });

  it('should create blame from buffer', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    const buffer = Buffer.from('Line 1\nLine 2\nLine 3\n');
    const bufferBlame = blame.buffer(buffer);
    expect(bufferBlame.getHunkCount()).toBeGreaterThan(0);
  });

  it('should handle empty files', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const emptyBlame = repo.blameFile('empty_file');
    expect(emptyBlame.getHunkCount()).toBe(1);
    expect(() => emptyBlame.getHunkByLine(1)).toThrow();
  });

  it('should handle binary files', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const binaryBlame = repo.blameFile('binary_file');
    expect(binaryBlame.getHunkCount()).toBeGreaterThan(0);
  });

  it('should throw appropriate errors for invalid inputs', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    expect(() => blame.getHunkByLine(0)).toThrow();
    expect(() => blame.getHunkByLine(9999)).toThrow();
    expect(() => blame.getHunkByIndex(9999)).toThrow();
    expect(() => repo.blameFile('non_existent_file')).toThrow();
  });

  it('should handle special buffer content', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    const specialBuffer = Buffer.from('Line 1\nblah blah blah\nLine 3\nLine 4\n');
    const specialBlame = blame.buffer(specialBuffer);

    const zeroCommitHunk = specialBlame.getHunkByIndex(1);
    expect(zeroCommitHunk.finalCommitId).toBe('0000000000000000000000000000000000000000');
    expect(zeroCommitHunk.finalSignature).toBeUndefined();
    expect(zeroCommitHunk.origSignature).toBeUndefined();
  });

  it('should support line-specific blame options', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const lineBlame = repo.blameFile('blame', { minLine: 2, maxLine: 2 });
    expect(lineBlame.getHunkCount()).toBe(1);
    const hunk = lineBlame.getHunkByIndex(0);
    expect(hunk.finalStartLineNumber).toBe(2);
    expect(hunk.finalSignature?.name).toBe('Seokju Me');
  });

  it('should support range-specific blame options', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const rangeBlame = repo.blameFile('blame', { minLine: 1, maxLine: 3 });
    const rangeHunks = [...rangeBlame.iter()];
    expect(rangeHunks.length).toBeGreaterThan(0);
    expect(rangeHunks.length).toBeLessThanOrEqual(3);
    expect(() => rangeBlame.getHunkByLine(4)).toThrow();
  });

  it('should support advanced blame options', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

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

  it('should check if blame is empty', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    expect(blame.isEmpty()).toBe(false);
  });

  it('should create blame from empty buffer', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    const buffer = Buffer.from(' ');
    const emptyBlame = blame.buffer(buffer);
    expect(emptyBlame.getHunkCount()).toBeGreaterThan(0);
  });

  it('should iterate blame hunks by line', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    const lineHunks = [...blame.iterByLine()];
    expect(lineHunks.length).toBeGreaterThan(0);
  });

  it('should iterate all blame hunks', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    const collectedHunks: BlameHunk[] = [...blame.iter()];
    expect(collectedHunks.length).toBe(blame.getHunkCount());
  });
});
