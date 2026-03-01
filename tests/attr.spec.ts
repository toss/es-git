import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('attr', () => {
  it('get attribute value', async () => {
    const p = await useFixture('attr');
    const repo = await openRepository(p);
    expect(repo.getAttr('a.txt', 'text')).toEqual('auto');
    expect(repo.getAttr('a.txt', 'eol')).toEqual('lf');
    expect(repo.getAttr('b', 'binary')).toEqual(true);
    expect(repo.getAttr('c', 'linguist-generated')).toEqual('true');
    expect(repo.getAttr('c', 'not_exists')).toEqual(null);
    expect(repo.getAttr('not_exists', 'text')).toEqual(null);
  });
});
