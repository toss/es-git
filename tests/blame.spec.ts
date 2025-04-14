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

  it('should collect hunks by line scanning with getHunksByLine', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const blame = repo.blameFile('blame');

    const lineHunks = blame.getHunksByLine();

    expect(lineHunks.length).toBeGreaterThan(0);

    const firstLineHunk = blame.getHunkByLine(1);
    expect(lineHunks[0]?.commitId).toBe(firstLineHunk.commitId);
    expect(lineHunks[0]?.finalStartLineNumber).toBe(firstLineHunk.finalStartLineNumber);

    for (let i = 0; i < lineHunks.length; i++) {
      const hunk = lineHunks[i]!;
      const startLine = hunk.finalStartLineNumber;
      const endLine = startLine + hunk.linesInHunk - 1;

      if (i > 0) {
        const prevHunk = lineHunks[i - 1]!;
        const prevEndLine = prevHunk.finalStartLineNumber + prevHunk.linesInHunk - 1;
        expect(startLine).toBeGreaterThan(prevEndLine);
      }

      for (let line = startLine; line <= endLine; line++) {
        if (line <= 10) {
          const lineHunk = blame.getHunkByLine(line);
          expect(lineHunk.commitId).toBe(hunk.commitId);
        }
      }
    }
  });

  it('should handle different file types with getHunksByLine', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const textBlame = repo.blameFile('blame');
    const textLineHunks = textBlame.getHunksByLine();
    expect(textLineHunks.length).toBeGreaterThan(0);

    const emptyBlame = repo.blameFile('empty_file');
    const emptyLineHunks = emptyBlame.getHunksByLine();
    expect(emptyLineHunks.length).toBe(0);

    const binaryBlame = repo.blameFile('binary_file');
    const binaryLineHunks = binaryBlame.getHunksByLine();
    expect(binaryLineHunks.length).toBeGreaterThan(0);

    const textBuffer = Buffer.from('Line 1\nLine 2\nLine 3\n');
    const textBufferBlame = textBlame.buffer(textBuffer, textBuffer.length);
    const textBufferHunks = textBufferBlame.getHunksByLine();
    expect(textBufferHunks.length).toBeGreaterThan(0);
  });

  it('should blame a specific line using the line option', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const lineBlame = repo.blameFile('blame', { line: 2 });

    expect(lineBlame.getHunkCount()).toBe(1);

    const hunk = lineBlame.getHunkByIndex(0);
    expect(hunk.commitId).toBeTruthy();
    expect(hunk.finalStartLineNumber).toBe(2);
    expect(hunk.signature?.name).toBe('Seokju Me');
    expect(hunk.signature?.email).toBe('seokju.na@toss.im');

    expect(() => lineBlame.getHunkByLine(1)).toThrow();
    expect(() => lineBlame.getHunkByLine(3)).toThrow();
  });

  it('should blame a range of lines using the range option', async () => {
    const p = await useFixture('blame');
    const repo = await openRepository(p);

    const rangeBlame = repo.blameFile('blame', { range: [1, 3] });

    const hunks = rangeBlame.getHunks();
    expect(hunks.length).toBeGreaterThan(0);
    expect(hunks.length).toBeLessThanOrEqual(3);

    const firstLineHunk = rangeBlame.getHunkByLine(1);
    expect(firstLineHunk.commitId).toBeTruthy();
    expect(firstLineHunk.finalStartLineNumber).toBe(1);
    expect(firstLineHunk.signature?.name).toBe('Seokju Na');
    expect(firstLineHunk.signature?.email).toBe('seokju.na@gmail.com');

    const secondLineHunk = rangeBlame.getHunkByLine(2);
    expect(secondLineHunk.commitId).toBeTruthy();
    expect(secondLineHunk.finalStartLineNumber).toBe(2);
    expect(secondLineHunk.signature?.name).toBe('Seokju Me');
    expect(secondLineHunk.signature?.email).toBe('seokju.na@toss.im');

    const thirdLineHunk = rangeBlame.getHunkByLine(3);
    expect(thirdLineHunk.commitId).toBeTruthy();
    expect(thirdLineHunk.finalStartLineNumber).toBeLessThanOrEqual(3);

    expect(() => rangeBlame.getHunkByLine(0)).toThrow();
    expect(() => rangeBlame.getHunkByLine(4)).toThrow();
  });
});
