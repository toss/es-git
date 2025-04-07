#!/usr/bin/env -S node --no-warnings=ExperimentalWarning --experimental-strip-types
import fs from 'node:fs/promises';
import { EOL } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const rootdir = path.join(dirname, '..');

const [, , build, commit] = process.argv;
if (build == null || commit == null) {
  throw new Error('Invalid build/commit info');
}

const pkgFilepath = path.join(rootdir, 'package.json');
const pkg = JSON.parse(await fs.readFile(pkgFilepath, 'utf8'));
pkg.version = `${pkg.version}-next.${build}+${commit.slice(0, 7)}`;

await fs.writeFile(pkgFilepath, `${JSON.stringify(pkg, null, 2)}${EOL}`, 'utf8');
