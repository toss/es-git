import { exec as execChildProcess } from 'node:child_process';
import { Repository as SimpleGitRepository } from '@napi-rs/simple-git';
import { Repository as NodeGitRepository } from 'nodegit';
import { type BenchOptions, bench } from 'vitest';
import { type Repository, openRepository } from '../index';

export function exec(command: string, cwd: string): Promise<string> {
  return new Promise<string>((resolve, reject) => {
    let output = '';
    const cp = execChildProcess(
      command,
      {
        encoding: 'utf8',
        cwd,
      },
      (err, stdout) => {
        if (err != null) {
          reject(err);
          return;
        }
        output += stdout;
      }
    );
    cp.on('close', () => resolve(output));
  });
}

export function createBenchmark(gitDir: string) {
  const benchESGit = (
    name: string,
    operation: (repo: Repository) => unknown | Promise<unknown>,
    options?: BenchOptions
  ) => {
    let esGitRepo: Repository;
    const { setup: userSetup, ...restOptions } = options || {};

    bench(
      name,
      async () => {
        if (!esGitRepo) throw new Error('Setup failed for es-git benchmark');
        await operation(esGitRepo);
      },
      {
        setup: async (task, mode) => {
          esGitRepo = await openRepository(gitDir);
          if (userSetup) {
            await userSetup(task, mode);
          }
        },
        ...restOptions,
      }
    );
  };

  const benchNodeGit = (
    name: string,
    operation: (repo: NodeGitRepository) => unknown | Promise<unknown>,
    options?: BenchOptions
  ) => {
    let nodeGitRepo: NodeGitRepository;
    const { setup: userSetup, ...restOptions } = options || {};

    bench(
      name,
      async () => {
        if (!nodeGitRepo) throw new Error('Setup failed for nodegit benchmark');
        await operation(nodeGitRepo);
      },
      {
        setup: async (task, mode) => {
          nodeGitRepo = await NodeGitRepository.open(gitDir);
          if (userSetup) {
            await userSetup(task, mode);
          }
        },
        ...restOptions,
      }
    );
  };

  const benchSimpleGit = (
    name: string,
    operation: (repo: SimpleGitRepository) => unknown | Promise<unknown>,
    options?: BenchOptions
  ) => {
    let simpleGitRepo: SimpleGitRepository;
    const { setup: userSetup, ...restOptions } = options || {};

    bench(
      name,
      async () => {
        if (!simpleGitRepo) throw new Error('Setup failed for simple-git benchmark');
        await Promise.resolve(operation(simpleGitRepo));
      },
      {
        setup: async (task, mode) => {
          simpleGitRepo = new SimpleGitRepository(gitDir);
          if (userSetup) {
            await userSetup(task, mode);
          }
        },
        ...restOptions,
      }
    );
  };

  const benchChildProcess = (name: string, operation: () => unknown | Promise<unknown>, options?: BenchOptions) => {
    bench(
      name,
      async () => {
        await operation();
      },
      options
    );
  };

  return {
    esGit: benchESGit,
    nodegit: benchNodeGit,
    simpleGit: benchSimpleGit,
    childProcess: benchChildProcess,
  };
}
