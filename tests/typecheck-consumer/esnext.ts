import { openRepository } from '../../index.js';

async function main() {
  const repo = await openRepository('.');
  const names: string[] = repo
    .branches()
    .map(branch => branch.name)
    .toArray();
  return names;
}

void main();
