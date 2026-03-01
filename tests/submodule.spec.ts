import fs from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { describe, expect, it } from 'vitest';
import { cloneRepository, openRepository } from '../index';
import { useFixture } from './fixtures';
import { makeTmpDir } from './tmp';

describe('submodule', () => {
  it('smoke', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    const s1 = repo.submodule('/path/to/nowhere', 'foo', true);
    await s1.init(false);
    await s1.sync();

    const s2 = repo.submodule('/path/to/nowhere', 'bar', true);
    await s2.init(false);

    const submodules = repo.submodules();
    expect(submodules).toHaveLength(2);

    const submodule1 = submodules[0]!;
    expect(submodule1.name()).toEqual('bar');
    expect(submodule1.url()).toEqual('/path/to/nowhere');
    expect(submodule1.branch()).toEqual(null);
    expect(submodule1.headId()).toEqual(null);
    expect(submodule1.indexId()).toEqual(null);
    expect(submodule1.workdirId()).toEqual(null);

    expect(() => repo.getSubmodule('bar')).not.toThrowError();
    expect(() => repo.getSubmodule('not_exists')).toThrowError(/libgit2 error: no submodule named 'not_exists'/);

    await expect(submodule1.open()).resolves.toBeTruthy();
    expect(submodule1.path()).toEqual('bar');

    await submodule1.reload(true);
  });

  it('add a submodule', async () => {
    const p1 = await useFixture('commits');
    const p2 = await useFixture('commits');

    const repo1 = await openRepository(p1);
    const repo2 = await openRepository(p2);

    const url = pathToFileURL(repo1.workdir()!);
    const submodule = repo2.submodule(url.toString(), 'bar', true);
    await fs.rm(path.join(p2, 'bar'), { recursive: true });
    await cloneRepository(url.toString(), path.join(p2, 'bar'));
    submodule.addToIndex(false);
    submodule.addFinalize();
  });

  it('update submodule', async () => {
    const p1 = await useFixture('commits');
    const p2 = await useFixture('commits');

    const repo1 = await openRepository(p1);
    const repo2 = await openRepository(p2);

    const url = pathToFileURL(repo1.workdir()!);
    const submodule = repo2.submodule(url.toString(), 'bar', true);
    await fs.rm(path.join(p2, 'bar'), { recursive: true });
    await cloneRepository(url.toString(), path.join(p2, 'bar'));
    submodule.addToIndex(false);
    submodule.addFinalize();

    const submodules = repo1.submodules();
    for (const s of submodules) {
      await s.update(true);
    }
  });

  it('clone submodule', async () => {
    const p1 = await useFixture('commits');
    const p2 = await useFixture('commits');
    const parentPath = await useFixture('commits');

    const repo1 = await openRepository(p1);
    const repo2 = await openRepository(p2);
    const parentRepo = await openRepository(parentPath);

    const url1 = pathToFileURL(repo1.workdir()!);
    const url2 = pathToFileURL(repo2.workdir()!);

    const submodule1 = parentRepo.submodule(url1.toString(), 'bar', true);
    const submodule2 = parentRepo.submodule(url2.toString(), 'bar2', true);

    await expect(submodule1.clone()).resolves.toBeTruthy();
    await expect(submodule2.clone()).resolves.toBeTruthy();
  });

  it('repo init submodule', async () => {
    const childPath = await useFixture('commits');
    const parentPath = await useFixture('commits');

    const childRepo = await openRepository(childPath);
    const parentRepo = await openRepository(parentPath);

    const childUrl = pathToFileURL(childRepo.workdir()!);
    const parentUrl = pathToFileURL(parentRepo.workdir()!);

    const submodule = parentRepo.submodule(childUrl.toString(), 'bar', true);

    await submodule.clone();
    submodule.addToIndex(true);
    submodule.addFinalize();

    const signature = { name: 'Seokju Na', email: 'seokju.me@toss.im' };
    await fs.writeFile(path.join(parentPath, 'foo'), 'foo');
    const index = parentRepo.index();
    index.addPath('foo');
    index.write();
    const treeId = index.writeTree();
    const tree = parentRepo.getTree(treeId);
    parentRepo.commit(tree, 'foo', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [parentRepo.head().target()!],
    });

    const tmpDir = await makeTmpDir('submodule');
    const newParentRepo = await cloneRepository(parentUrl.toString(), tmpDir);

    const submodules = newParentRepo.submodules();
    expect(submodules).toHaveLength(1);
    const childSubmodule = submodules[0]!;

    await childSubmodule.init(false);
    expect(childSubmodule.url()).toEqual(childUrl.toString());

    // open() is not possible before initializing the repo
    await expect(childSubmodule.open()).rejects.toThrowError();
    await childSubmodule.repoInit(true);
    await expect(childSubmodule.open()).resolves.toBeTruthy();
  });
});
