import { describe, expect, it } from 'vitest';
import { createBranch, deleteBranch, getBranch, getSha } from '../index';
import { useFixture } from './fixtures';

describe('branch', () => {
  it('createBranch 는 브랜치를 만들 수 있다.', async () => {
    const dir = await useFixture('empty');
    const mainSha = getSha('main', { dir });

    const branch = createBranch(
      {
        branchName: 'my-branch',
        targetSha: mainSha,
      },
      { dir }
    );
    expect(branch).toEqual(
      expect.objectContaining({
        name: 'my-branch',
        oid: expect.any(String),
      })
    );

    /* cleanup */
    deleteBranch(
      {
        branchName: 'my-branch',
      },
      { dir }
    );
  });

  it('getBranch 는 브랜치를 반환한다.', async () => {
    const dir = await useFixture('empty');
    const mainSha = getSha('main', { dir });

    createBranch(
      {
        branchName: 'my-branch',
        targetSha: mainSha,
      },
      { dir }
    );

    const branch = getBranch(
      {
        branchName: 'my-branch',
      },
      { dir }
    );

    expect(branch).toEqual(
      expect.objectContaining({
        name: 'my-branch',
        oid: expect.any(String),
      })
    );

    /* cleanup */
    deleteBranch(
      {
        branchName: 'my-branch',
      },
      { dir }
    );
  });

  it('deleteBranch 는 브랜치를 제거할 수 있다.', async () => {
    const dir = await useFixture('empty');
    const mainSha = getSha('main', { dir });

    createBranch(
      {
        branchName: 'my-branch',
        targetSha: mainSha,
      },
      { dir }
    );

    /* cleanup */
    deleteBranch(
      {
        branchName: 'my-branch',
      },
      { dir }
    );

    expect(() => getBranch({ branchName: 'my-branch' }, { dir })).toThrow(/cannot locate local branch 'my-branch'/);
  });
});
