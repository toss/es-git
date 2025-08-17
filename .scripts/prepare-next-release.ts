#!/usr/bin/env -S node --no-warnings=ExperimentalWarning --experimental-strip-types
import fs from 'node:fs/promises';
import { EOL } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const rootdir = path.join(dirname, '..');

const { GITHUB_SHA: sha, GITHUB_RUN_NUMBER: runNumber, GITHUB_RUN_ATTEMPT: runAttempt } = process.env;

if (sha == null || runNumber == null || runAttempt == null) {
  throw new Error('Script should be run in GitHub Actions');
}

const build = `${runNumber}.${runAttempt}`;
const commit = sha.slice(0, 7);

const pkgFilepath = path.join(rootdir, 'package.json');
const pkg = JSON.parse(await fs.readFile(pkgFilepath, 'utf8'));
const nextVersion = `${pkg.version}-next.${build}+${commit}`;
console.log('next version:', nextVersion);
pkg.version = nextVersion;

await fs.writeFile(pkgFilepath, `${JSON.stringify(pkg, null, 2)}${EOL}`, 'utf8');
