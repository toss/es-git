import { describe, expect, it } from 'vitest';
import { getSha } from '../index';
import { useFixture } from './fixtures';

describe('getSha', () => {
  it('sha를 가져올 수 있다.', async () => {
    const dir = await useFixture('empty');
    const sha = getSha('main', { dir });
    expect(typeof sha).toBe('string');
  });
});
