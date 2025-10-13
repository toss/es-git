import { randomBytes } from 'node:crypto';
import fs from 'node:fs/promises';
import path from 'node:path';
import { inject } from 'vitest';

export async function prepareTmpDir(prefix?: string) {
  const tmpDir = inject('tmpDir');
  const dir = path.join(tmpDir, prefix ?? '', randomBytes(16).toString('hex'));
  await fs.mkdir(path.dirname(dir), { recursive: true });
  return dir;
}
