import { describe, expect, it } from 'vitest';
import { getBranch, getSha } from '../index';
import { useFixture } from './fixtures';

describe('getSha', () => {
  it('리포지토리가 없으면 오류가 발생한다.', async () => {
    const dir = await useFixture('notgit');
    expect(() => getSha('main', { dir })).toThrowError(/libgit2 error:/)
  });

  it('sha를 가져올 수 있다.', async () => {
    const dir = await useFixture('empty');
    const sha = getSha('main', { dir });
    expect(typeof sha).toBe('string');
  });
});
