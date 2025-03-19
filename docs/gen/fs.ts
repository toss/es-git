import fs from 'node:fs/promises';
import path from 'node:path';

let rootDir: string | null = null;

export async function findRootDir(): Promise<string> {
  if (rootDir != null) {
    return rootDir;
  }
  let current = process.cwd();
  while (true) {
    if (current === '.') {
      throw new Error('Cannot find root directory.');
    }
    const files = await fs.readdir(current);
    if (files.includes('yarn.lock')) {
      break;
    }
    current = path.resolve(current, '../');
  }
  rootDir = current;
  return rootDir;
}
