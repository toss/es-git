import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { createBranch, deleteBranch, getBranch, getSha } from '../index';

describe('branch', () => {
  const gitContext = { dir: path.resolve(__dirname, '..') };

  it('createBranch 는 브랜치를 만들 수 있다.', () => {
    const mainSha = getSha('main', gitContext);

    const branch = createBranch(
      {
        branchName: 'my-branch',
        targetSha: mainSha,
      },
      gitContext
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
      gitContext
    );
  });

  it('getBranch 는 브랜치를 반환한다.', () => {
    const mainSha = getSha('main', gitContext);

    createBranch(
      {
        branchName: 'my-branch',
        targetSha: mainSha,
      },
      gitContext
    );

    const branch = getBranch(
      {
        branchName: 'my-branch',
      },
      gitContext
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
      gitContext
    );
  });

  it('deleteBranch 는 브랜치를 제거할 수 있다.', () => {
    const mainSha = getSha('main', gitContext);

    createBranch(
      {
        branchName: 'my-branch',
        targetSha: mainSha,
      },
      gitContext
    );

    /* cleanup */
    deleteBranch(
      {
        branchName: 'my-branch',
      },
      gitContext
    );

    expect(() => getBranch({ branchName: 'my-branch' }, gitContext)).toThrow(/cannot locate local branch 'my-branch'/);
  });
});
