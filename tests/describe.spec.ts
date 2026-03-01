import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('describe', () => {
  it('get describe from "HEAD"', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const describe = repo.describe({ describeAll: true });
    expect(describe.format()).toEqual('heads/main');
  });

  it('get describe from commit', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const obj = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82').asObject();
    const describe = obj.describe({ describeAll: true });
    expect(describe.format()).toEqual('heads/main');
  });
});
