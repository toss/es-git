import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import glob from 'fast-glob';
import { makeTmpDir } from './tmp';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export async function useFixture(name: string) {
  const src = path.join(__dirname, 'fixtures', name);
  const dest = await makeTmpDir(name);
  await copy(src, dest);
  return dest;
}

async function copy(src: string, dest: string) {
  const files = await glob('**/*', {
    cwd: src,
    onlyFiles: true,
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
