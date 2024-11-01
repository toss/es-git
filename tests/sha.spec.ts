import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { getSha } from '../index';

describe('getSha', () => {
  const gitContext = { dir: path.resolve(__dirname, '..') };

  it('sha를 가져올 수 있다.', () => {
    const sha = getSha('main', gitContext);
    expect(typeof sha).toBe('string');
  });
});
