import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('apply', () => {
  it('apply at working directory', async () => {
    const p = await useFixture('diff');
    const repo = await openRepository(p);

    const headTree = repo.head().peelToTree();
    await fs.writeFile(path.join(p, 'A'), 'A modified');

    const index = repo.index();
    index.addPath('A');
    index.write();
    const treeId = repo.getTree(index.writeTree());

    const diff = repo.diffTreeToTree(headTree, treeId);
    repo.checkoutHead({ force: true });
    repo.apply(diff, 'WorkDir');

    const content = await fs.readFile(path.join(p, 'A'), 'utf-8');
    expect(content).toEqual('A modified');
  });

  it('will not actually apply if check option is enabled', async () => {
    const p = await useFixture('diff');
    const repo = await openRepository(p);

    const headTree = repo.head().peelToTree();
    await fs.writeFile(path.join(p, 'A'), 'A modified');

    const index = repo.index();
    index.addPath('A');
    index.write();
    const treeId = repo.getTree(index.writeTree());

    const diff = repo.diffTreeToTree(headTree, treeId);
    repo.checkoutHead({ force: true });
    repo.apply(diff, 'WorkDir', { check: true });

    const content = await fs.readFile(path.join(p, 'A'), 'utf-8');
    expect(content).not.toEqual('A modified');
  });
});
