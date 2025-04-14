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

  it('should verify author information for specific lines', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const blame = repo.blameFile('blame');

    const line1Hunk = blame.getHunkByLine(1);
    expect(line1Hunk.signature?.name).toBe('Seokju Na');
    expect(line1Hunk.signature?.email).toBe('seokju.na@gmail.com');

    const line2Hunk = blame.getHunkByLine(2);
    expect(line2Hunk.signature?.name).toBe('Seokju Me');
    expect(line2Hunk.signature?.email).toBe('seokju.na@toss.im');

    const line7Hunk = blame.getHunkByLine(7);
    expect(line7Hunk.signature?.name).toBe('Seokju Me');
    expect(line7Hunk.signature?.email).toBe('seokju.na@toss.im');

    const line8Hunk = blame.getHunkByLine(8);
    expect(line8Hunk.signature?.name).toBe('Seokju Na');
    expect(line8Hunk.signature?.email).toBe('seokju.na@gmail.com');
  });

  it('should handle all lines in the file', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const blame = repo.blameFile('blame');
    const hunks = blame.getHunks();

    for (let i = 1; i <= 10; i++) {
      const hunk = blame.getHunkByLine(i);
      expect(hunk.commitId).toBeTruthy();
      expect(hunk.finalStartLineNumber).toBeLessThanOrEqual(i);

      const endLineNumber = hunk.finalStartLineNumber + hunk.linesInHunk - 1;
      expect(endLineNumber).toBeGreaterThanOrEqual(i);

      expect(hunk.signature).toBeDefined();
    }
  });

  it('should throw for invalid resources', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);
    const blame = repo.blameFile('blame');

    expect(() => blame.getHunkByLine(0)).toThrow();
    expect(() => blame.getHunkByLine(9999)).toThrow();
    expect(() => blame.getHunkByIndex(9999)).toThrow();
  });

  it('should handle empty files', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const emptyBlame = repo.blameFile('empty_file');

    expect(emptyBlame.getHunkCount()).toBe(1);
    const hunk = emptyBlame.getHunkByIndex(0);
    expect(hunk.commitId).toBeTruthy();

    expect(() => emptyBlame.getHunkByLine(1)).toThrow();
  });

  it('should handle binary files', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const binaryBlame = repo.blameFile('binary_file');
    expect(binaryBlame.getHunkCount()).toBeGreaterThan(0);
    expect(binaryBlame.getHunks().length).toBeGreaterThan(0);
    expect(binaryBlame.getHunkCount()).toEqual(binaryBlame.getHunks().length);

    const hunk = binaryBlame.getHunkByIndex(0);
    expect(hunk.commitId).toBeTruthy();
    expect(hunk.signature).toBeDefined();

    const hunks = binaryBlame.getHunks();
    for (let i = 0; i < hunks.length; i++) {
      expect(hunks[i]?.commitId).toBeTruthy();
      expect(hunks[i]?.linesInHunk).toBeGreaterThan(0);
    }

    const buffer = Buffer.from([0x01, 0x02, 0x03, 0x04]);
    const bufferBlame = binaryBlame.buffer(buffer, buffer.length);
    expect(bufferBlame.getHunkCount()).toBeGreaterThan(0);

    expect(() => binaryBlame.getHunkByLine(9999)).toThrow();
  });

  it('should throw for non-existent files', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    expect(() => repo.blameFile('non_existent_file')).toThrow();
  });
});
