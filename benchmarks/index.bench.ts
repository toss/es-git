import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { Repository as NodeGitRepository, Revparse as NodeGitRevparse } from 'nodegit';
import { Repository as SimpleGitRepository } from 'simple-git';
import { bench, describe } from 'vitest';
import { openRepository } from '../index';
import { exec } from './util';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const gitDir = path.resolve(dirname, '..');

describe('open', () => {
  bench('es-git', async () => {
    await openRepository(gitDir);
  });

  bench('nodegit', async () => {
    await NodeGitRepository.open(gitDir);
  });

  bench('simple-git', () => {
    SimpleGitRepository.init(gitDir);
  });
});

describe('rev-parse', () => {
  bench('es-git', async () => {
    const repo = await openRepository(gitDir);
    repo.revparse('HEAD');
  });

  bench('nodegit', async () => {
    const repo = await NodeGitRepository.open(gitDir);
    await NodeGitRevparse.single(repo, 'HEAD');
  });

  bench('simple-git', () => {
    const repo = SimpleGitRepository.init(gitDir);
    repo.head().resolve();
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
    const repo = await NodeGitRepository.open(gitDir);
    const revwalk = repo.createRevWalk();
    revwalk.pushRange('b597cf0b..d47af3b0');
    const oids = await revwalk.fastWalk(200);
    console.assert(oids.length === 103);
  });

  bench('simple-git', () => {
    const repo = SimpleGitRepository.init(gitDir);
    const revwalk = repo.revWalk();
    const oids = [...revwalk.pushRange('b597cf0b..d47af3b0')];
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
    const repo = await NodeGitRepository.open(gitDir);
    await repo.getCommit('d47af3b02b36834dcde1b60afb64547460f5abc0');
  });

  bench('simple-git', () => {
    const repo = SimpleGitRepository.init(gitDir);
    repo.findCommit('d47af3b02b36834dcde1b60afb64547460f5abc0');
  });

  bench('child_process', async () => {
    await exec('git log d47af3b02b36834dcde1b60afb64547460f5abc0', gitDir);
  });
});
