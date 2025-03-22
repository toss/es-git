import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { openRepository } from 'es-git';
import { Repository, Revparse } from 'nodegit';
import { bench, describe } from 'vitest';
import { exec } from './util';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const gitDir = path.resolve(dirname, '..');

describe('open', () => {
  bench('es-git', async () => {
    await openRepository(gitDir);
  });

  bench('nodegit', async () => {
    await Repository.open(gitDir);
  });
});

describe('rev-parse', () => {
  bench('es-git', async () => {
    const repo = await openRepository(gitDir);
    repo.revparse('HEAD');
  });

  bench('nodegit', async () => {
    const repo = await Repository.open(gitDir);
    await Revparse.single(repo, 'HEAD');
  });

  bench('child_process', async () => {
    await exec('git rev-parse HEAD', gitDir);
  });
});

describe('revwalk', () => {
  bench('es-git', async () => {
    const repo = await openRepository(gitDir);
    const revwalk = repo.revwalk().pushRange('b597cf0b..d47af3b0');
    console.assert([...revwalk].length === 103);
  });

  bench('nodegit', async () => {
    const repo = await Repository.open(gitDir);
    const revwalk = repo.createRevWalk();
    revwalk.pushRange('b597cf0b..d47af3b0');
    const oids = await revwalk.fastWalk(200);
    console.assert(oids.length === 103);
  });

  bench('child_process', async () => {
    await exec('git log b597cf0b..d47af3b0', gitDir);
  });
});

describe('get commit', () => {
  bench('es-git', async () => {
    const repo = await openRepository(gitDir);
    repo.getCommit('d47af3b02b36834dcde1b60afb64547460f5abc0');
  });

  bench('nodegit', async () => {
    const repo = await Repository.open(gitDir);
    await repo.getCommit('d47af3b02b36834dcde1b60afb64547460f5abc0');
  });

  bench('child_process', async () => {
    await exec('git log d47af3b02b36834dcde1b60afb64547460f5abc0', gitDir);
  });
});
