import { getRandomValues } from 'node:crypto';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { afterAll } from 'vitest';
import { isTarget } from './env';

const tmpDirs: string[] = [];

afterAll(async () => {
  const removes = tmpDirs.map(dir => fs.rm(dir, { recursive: true, force: true }));
  await Promise.allSettled(removes);
});

export async function makeTmpDir(prefix?: string) {
  const tmpdir = isTarget('linux', 'arm64')
    ? path.join(process.cwd(), 'tmp', 'es-git', prefix ?? '', randomHex(8))
    : path.join(os.tmpdir(), 'es-git', prefix ?? '', randomHex(8));
  await fs.mkdir(tmpdir, { recursive: true });
  tmpDirs.push(tmpdir);
  return tmpdir;
}

function randomHex(size: number) {
  const buf = Buffer.alloc(size);
  getRandomValues(buf);
  return buf.toString('hex');
}
