import { getRandomValues } from 'node:crypto';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { afterAll } from 'vitest';

const tmpDirs: string[] = [];

afterAll(async () => {
  const removes = tmpDirs.map(dir => fs.rm(dir, { recursive: true, force: true }));
  await Promise.all(removes);
});

export async function makeTmpDir(prefix?: string) {
  const tmpdir = path.join(os.tmpdir(), 'esgit', prefix ?? '', randomHex(8));
  await fs.mkdir(tmpdir, { recursive: true });
  tmpDirs.push(tmpdir);
  return tmpdir;
}

function randomHex(size: number) {
  const buf = Buffer.alloc(size);
  getRandomValues(buf);
  return buf.toString('hex');
}
