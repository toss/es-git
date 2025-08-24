import { cloneRepository } from 'es-git';
import { describe, inject, it } from 'vitest';
import { prepareTmpDir } from './util.js';

describe(`${inject('es-git.version')} clone`, () => {
  it('should clone public repository', async () => {
    const dir = await prepareTmpDir('dummy-repo');
    await cloneRepository('https://github.com/seokju-na/dummy-repo', dir);
  });

  it('should clone private repository with personal access token', async () => {
    const dir = await prepareTmpDir('dummy-repo-private');
    await cloneRepository('https://github.com/seokju-na/dummy-repo-private', dir, {
      fetch: {
        credential: {
          type: 'Plain',
          password: process.env.TEST_GITHUB_TOKEN!,
        },
      },
    });
  });
});
