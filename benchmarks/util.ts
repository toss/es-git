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

type BenchmarkOptions = Omit<BenchOptions, "setup">;

export function createBenchmark(gitDir: string) {
  const benchESGit = (
    name: string,
    operation: (repo: Repository) => unknown | Promise<unknown>,
    options?: BenchmarkOptions
  ) => {
    let esGitRepo: Repository;
    bench(
      name,
      async () => {
        if (!esGitRepo) throw new Error('Setup failed for es-git benchmark');
        await operation(esGitRepo);
      },
      {
        setup: async () => {
          esGitRepo = await openRepository(gitDir);
        },
        ...options,
      }
    );
  };

  const benchNodeGit = (
    name: string,
    operation: (repo: NodeGitRepository) => unknown | Promise<unknown>,
    options?: BenchmarkOptions
  ) => {
    let nodeGitRepo: NodeGitRepository;
    bench(
      name,
      async () => {
        if (!nodeGitRepo) throw new Error('Setup failed for nodegit benchmark');
        await operation(nodeGitRepo);
      },
      {
        setup: async () => {
          nodeGitRepo = await NodeGitRepository.open(gitDir);
        },
        ...options,
      }
    );
  };

  const benchSimpleGit = (
    name: string,
    operation: (repo: SimpleGitRepository) => unknown | Promise<unknown>,
    options?: BenchmarkOptions
  ) => {
    let simpleGitRepo: SimpleGitRepository;
    bench(
      name,
      () => {
        if (!simpleGitRepo) throw new Error('Setup failed for simple-git benchmark');
        operation(simpleGitRepo);
      },
      {
        setup: () => {
          simpleGitRepo = new SimpleGitRepository(gitDir);
        },
        ...options,
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
