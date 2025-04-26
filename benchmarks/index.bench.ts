import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { Repository as SimpleGitRepository } from '@napi-rs/simple-git';
import { Repository as NodeGitRepository, Revparse as NodeGitRevparse } from 'nodegit';
import { bench, describe } from 'vitest';
import { openRepository } from '../index';
import { createBenchmark, exec } from './util';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const gitDir = path.resolve(dirname, '..');

const benchmark = createBenchmark(gitDir);

describe('open', () => {
  bench('es-git', async () => {
    await openRepository(gitDir);
  });

  bench('nodegit', async () => {
    await NodeGitRepository.open(gitDir);
  });

  bench('@napi-rs/simple-git', () => {
    new SimpleGitRepository(gitDir);
  });
});

describe('rev-parse', () => {
  benchmark.esGit('es-git', repo => repo.revparse('HEAD'));

  benchmark.nodegit('nodegit', repo => NodeGitRevparse.single(repo, 'HEAD'));

  benchmark.simpleGit('@napi-rs/simple-git', repo => repo.head().resolve());

  benchmark.childProcess('child_process', () => exec('git rev-parse HEAD', gitDir));
});

describe('revwalk', () => {
  benchmark.esGit('es-git', repo => {
    const revwalk = repo.revwalk().pushRange('b597cf0b..d47af3b0');
    const oids = [...revwalk];
    console.assert(oids.length === 103);
  });

  benchmark.nodegit('nodegit', async repo => {
    const revwalk = repo.createRevWalk();
    revwalk.pushRange('b597cf0b..d47af3b0');
    const oids = await revwalk.fastWalk(200);
    console.assert(oids.length === 103);
  });

  benchmark.simpleGit('@napi-rs/simple-git', repo => {
    const revwalk = repo.revWalk();
    const oids = [...revwalk.pushRange('b597cf0b..d47af3b0')];
    console.assert(oids.length === 103);
  });

  benchmark.childProcess('child_process', async () => {
    await exec('git log b597cf0b..d47af3b0', gitDir);
  });
});

describe('get commit', () => {
  benchmark.esGit('es-git', repo => repo.getCommit('d47af3b02b36834dcde1b60afb64547460f5abc0'));

  benchmark.nodegit('nodegit', repo => repo.getCommit('d47af3b02b36834dcde1b60afb64547460f5abc0'));

  benchmark.simpleGit('@napi-rs/simple-git', repo => repo.findCommit('d47af3b02b36834dcde1b60afb64547460f5abc0'));

  benchmark.childProcess('child_process', () => exec('git log d47af3b02b36834dcde1b60afb64547460f5abc0', gitDir));
});
