import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('ignore', () => {
  it('should handle basic ignore rules', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'server.log'), 'server.log');
    await fs.writeFile(path.join(p, 'index.js'), 'index.js');

    repo.addIgnoreRule('*.log');
    expect(repo.isPathIgnored('server.log')).toBe(true);
    expect(repo.isPathIgnored('index.js')).toBe(false);

    repo.clearIgnoreRules();
    expect(repo.isPathIgnored('server.log')).toBe(false);
    expect(repo.isPathIgnored('.git/config')).toBe(true);
  });

  it('should handle multiple ignore patterns', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'server.log'), 'server.log');
    await fs.writeFile(path.join(p, 'index.js'), 'index.js');
    await fs.mkdir(path.join(p, 'node_modules'), { recursive: true });
    await fs.mkdir(path.join(p, 'dist'), { recursive: true });
    await fs.writeFile(path.join(p, 'node_modules/package.json'), '{}');
    await fs.writeFile(path.join(p, 'dist/bundle.js'), 'dist/bundle.js');

    repo.addIgnoreRule('*.log\nnode_modules/\ndist/');

    expect(repo.isPathIgnored('server.log')).toBe(true);
    expect(repo.isPathIgnored('node_modules/package.json')).toBe(true);
    expect(repo.isPathIgnored('dist/bundle.js')).toBe(true);
    expect(repo.isPathIgnored('index.js')).toBe(false);
  });

  it('should respect .gitignore files', async () => {
    const p = await useFixture('empty');

    await fs.mkdir(path.join(p, 'dist'), { recursive: true });
    await fs.mkdir(path.join(p, 'src'), { recursive: true });
    await fs.writeFile(path.join(p, 'dist/bundle.js'), 'dist/bundle.js');
    await fs.writeFile(path.join(p, 'src/main.js'), 'index.js');
    await fs.writeFile(path.join(p, '.gitignore'), 'dist/\n*.js\n!src/main.js');

    const repo = await openRepository(p);

    expect(repo.isPathIgnored('dist/bundle.js')).toBe(true);
    expect(repo.isPathIgnored('src/main.js')).toBe(false);
  });

  it('should combine added rules with .gitignore rules', async () => {
    const p = await useFixture('empty');

    await fs.mkdir(path.join(p, 'dist'), { recursive: true });
    await fs.mkdir(path.join(p, 'logs'), { recursive: true });
    await fs.writeFile(path.join(p, 'dist/bundle.js'), 'dist/bundle.js');
    await fs.writeFile(path.join(p, 'logs/server.log'), 'logs/server.log');
    await fs.writeFile(path.join(p, '.gitignore'), 'dist/');

    const repo = await openRepository(p);

    repo.addIgnoreRule('logs/');

    expect(repo.isPathIgnored('dist/bundle.js')).toBe(true);
    expect(repo.isPathIgnored('logs/server.log')).toBe(true);
  });

  it('should handle invalid inputs gracefully', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'test.js'), 'test');

    repo.addIgnoreRule('');
    expect(repo.isPathIgnored('test.js')).toBe(false);

    expect(() => repo.isPathIgnored('')).not.toThrow();
    expect(() => repo.isPathIgnored('../outside')).not.toThrow();
    expect(() => repo.isPathIgnored('/absolute/path')).not.toThrow();

    // @ts-expect-error
    expect(() => repo.isPathIgnored(null)).toThrow();
    // @ts-expect-error
    expect(() => repo.isPathIgnored(undefined)).toThrow();
  });

  it('should handle special patterns correctly', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.mkdir(path.join(p, 'logs'), { recursive: true });
    await fs.writeFile(path.join(p, 'logs/error.log'), 'error');
    await fs.writeFile(path.join(p, 'logs/important.log'), 'important');
    await fs.mkdir(path.join(p, 'a/b/c'), { recursive: true });
    await fs.writeFile(path.join(p, 'a/b/c/file.txt'), 'file');
    await fs.writeFile(path.join(p, 'test[1].js'), 'test');

    repo.addIgnoreRule('*.log\n!logs/important.log\na/**/c/*.txt\ntest[[]*.js');

    expect(repo.isPathIgnored('logs/error.log')).toBe(true);
    expect(repo.isPathIgnored('logs/important.log')).toBe(false);
    expect(repo.isPathIgnored('a/b/c/file.txt')).toBe(true);
    expect(repo.isPathIgnored('test[1].js')).toBe(true);
  });

  it('should correctly handle path prefixes and comments', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.mkdir(path.join(p, 'src/build'), { recursive: true });
    await fs.mkdir(path.join(p, 'build'), { recursive: true });
    await fs.mkdir(path.join(p, 'logs-2025-04-23'), { recursive: true });
    await fs.writeFile(path.join(p, 'src/build/output.js'), 'output');
    await fs.writeFile(path.join(p, 'build/output.js'), 'output');
    await fs.writeFile(path.join(p, 'logs-2025-04-23/app.log'), 'log');
    await fs.writeFile(path.join(p, 'temp.txt'), 'temp');

    repo.addIgnoreRule('/build/\nlogs-*/\n# This is a comment\ntemp.txt');

    expect(repo.isPathIgnored('build/output.js')).toBe(true);
    expect(repo.isPathIgnored('src/build/output.js')).toBe(false);
    expect(repo.isPathIgnored('logs-2025-04-23/app.log')).toBe(true);
    expect(repo.isPathIgnored('temp.txt')).toBe(true);
  });
});
