import { describe, expect, it } from 'vitest';
import { ObjectType, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('object', () => {
  it('get object', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const obj = repo.getObject('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(obj.type()).toEqual(ObjectType.Commit);
    const commit = obj.asCommit();
    expect(commit).not.toBeNull();
    expect(commit!.author().name).toEqual('Seokju Na');
  });

  it('peel to commit', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const obj = repo.getObject('a01e988');
    const commit = obj.peelToCommit();
    expect(commit.message()).toEqual('second\n');
  });

  it('returns null if oid of object does not exists', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const obj = repo.findObject('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(obj).toBeNull();
  });
});
