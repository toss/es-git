import { describe, expect, it, vi } from 'vitest';
import { ObjectType, isValidTagName, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('tag', () => {
  // Reference: https://git-scm.com/docs/git-check-ref-format
  it('check tag name is valid', () => {
    expect(isValidTagName('blah_blah')).toBe(true);
    expect(isValidTagName('v1.2.3')).toBe(true);
    expect(isValidTagName('my/tag')).toBe(true);
    expect(isValidTagName('@')).toBe(true);

    expect(isValidTagName('-foo')).toBe(false);
    expect(isValidTagName('foo:bar')).toBe(false);
    expect(isValidTagName('foo^bar')).toBe(false);
    expect(isValidTagName('@{')).toBe(false);
    expect(isValidTagName('as\\cd')).toBe(false);
  });

  it('list tag names', async () => {
    const p = await useFixture('tag');
    const repo = await openRepository(p);
    expect(repo.tagNames()).toEqual(['v0', 'v1', 'v2']);
  });

  it('foreach tags', async () => {
    const p = await useFixture('tag');
    const repo = await openRepository(p);
    const foreach = vi.fn().mockReturnValue(true);
    repo.tagForeach(foreach);
    expect(foreach.mock.calls).toEqual([
      ['aa0040546ed22b8bb33f3bd621e8d10ed849b02c', 'refs/tags/v0'],
      ['674e3327707fcf32a348ecfc0cb6b93e57398b8c', 'refs/tags/v1'],
      ['567aa5c6b219312dc7758ab88ebb7a1e5d36d26b', 'refs/tags/v2'],
    ]);
  });

  it('foreach tags with aborting', async () => {
    const p = await useFixture('tag');
    const repo = await openRepository(p);
    const foreach = vi.fn().mockImplementation((_, name: string) => {
      return name !== 'refs/tags/v1';
    });
    repo.tagForeach(foreach);
    expect(foreach.mock.calls).toEqual([
      ['aa0040546ed22b8bb33f3bd621e8d10ed849b02c', 'refs/tags/v0'],
      ['674e3327707fcf32a348ecfc0cb6b93e57398b8c', 'refs/tags/v1'],
    ]);
  });

  it('get tag', async () => {
    const p = await useFixture('tag');
    const repo = await openRepository(p);
    const tag = repo.getTag('aa00405');
    expect(tag.name()).toEqual('v0');
    expect(tag.message()).toEqual('v0\n');
    expect(tag.tagger()).toEqual(
      expect.objectContaining({
        name: 'Seokju Na',
        email: 'seokju.me@gmail.com',
      })
    );
    const obj = tag.target();
    expect(obj.type()).toEqual(ObjectType.Commit);
    const commit = obj.peelToCommit();
    expect(commit.message()).toEqual('A\n');
  });

  it('delete tag', async () => {
    const p = await useFixture('tag');
    const repo = await openRepository(p);
    repo.deleteTag('v0');
    expect(repo.tagNames()).toEqual(['v1', 'v2']);
  });

  it('create tag', async () => {
    const p = await useFixture('tag');
    const repo = await openRepository(p);
    const commit = repo.getCommit('828954d');
    const tagId = repo.createTag('a0', commit.asObject(), 'a0', {
      tagger: {
        name: 'Seokju Na',
        email: 'seokju.me@toss.im',
      },
    });
    const tag = repo.getTag(tagId);
    expect(tag.name()).toEqual('a0');
    expect(tag.peel().id()).toEqual('828954df9f08dc8e172447cdacf0ddea1adf9e63');
    expect(tag.target().id()).toEqual('828954df9f08dc8e172447cdacf0ddea1adf9e63');
  });

  it('create tag force', async () => {
    const p = await useFixture('tag');
    const repo = await openRepository(p);
    const firstCommit = repo.getCommit('828954d');
    repo.createTag('a0', firstCommit.asObject(), 'a0', {
      tagger: {
        name: 'Seokju Na',
        email: 'seokju.me@toss.im',
      },
    });
    const secondCommit = repo.getCommit('26cf7eb');
    expect(() => repo.createTag('a0', secondCommit.asObject(), 'new a0')).toThrowError(/tag already exists/);
    const tagId = repo.createTag('a0', secondCommit.asObject(), 'new a0', { force: true });
    const tag = repo.getTag(tagId);
    expect(tag.name()).toEqual('a0');
  });
});
