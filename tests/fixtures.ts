import { getRandomValues } from 'node:crypto';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import glob from 'tiny-glob';
import { afterAll } from 'vitest';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const dirs: string[] = [];

afterAll(async () => {
  await Promise.all(dirs.map(dir => fs.rm(dir, { recursive: true, force: true })));
});

export async function useFixture(name: string) {
  const tmpdir = os.tmpdir();
  const src = path.join(__dirname, 'fixtures', name);
  const dest = path.join(tmpdir, 'esgit', randomHex(8));
  await fs.mkdir(dest, { recursive: true });
  dirs.push(dest);
  await copy(src, dest);
  return dest;
}

async function copy(src: string, dest: string) {
  const files = await glob('**/*', {
    cwd: src,
    filesOnly: true,
    dot: true,
  });
  await Promise.all(
    files.map(async filepath => {
      const source = path.join(src, filepath);
      const destination = path.join(dest, filepath.replace('_git', '.git'));
      await fs.mkdir(path.dirname(destination), { recursive: true });
      return fs.copyFile(source, destination);
    })
  );
}

function randomHex(size: number) {
  const buf = Buffer.alloc(size);
  getRandomValues(buf);
  return buf.toString('hex');
}
