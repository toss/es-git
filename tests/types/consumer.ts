import { openRepository } from '../../index.js';

async function main() {
  const repo = await openRepository('.');
  const statuses = repo.statuses();

  for (let index = 0; index < Number(statuses.len()); index += 1) {
    const entry = statuses.get(index);
    if (entry != null) {
      entry.path();
      entry.status();
    }
  }

  repo.getAnnotatedCommitFromReference(repo.head());
}

void main();
