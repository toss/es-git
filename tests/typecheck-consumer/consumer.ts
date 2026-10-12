// Regression fixture for https://github.com/toss/es-git/issues/215.
//
// Typechecks a small slice of the public API under the exact consumer
// config that surfaced the bug: a plain ES2022 target with skipLibCheck
// disabled. CI runs `tsc -p tests/typecheck-consumer` directly against the
// committed `index.d.ts`, no build step required.
import { openRepository } from '../../index.js';

async function main() {
  const repo = await openRepository('.');

  // `Branches extends Iterator<...>` failed to typecheck once `Iterator`
  // resolved to the ES2015 interface instead of the esnext class.
  for (const branch of repo.branches()) {
    branch.name;
  }

  // The generated signature referenced `GitReference`, a Rust-only type
  // alias with no corresponding exported class.
  const reference = repo.head();
  repo.getAnnotatedCommitFromReference(reference);
}

void main();
