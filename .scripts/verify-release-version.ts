#!/usr/bin/env -S node --no-warnings=ExperimentalWarning --experimental-strip-types
import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const SEMVER_PATTERN = /^(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z-.]+))?(?:\+[0-9A-Za-z-.]+)?$/;

interface Semver {
  major: number;
  minor: number;
  patch: number;
  prerelease: string[];
}

export function parseSemver(version: string): Semver {
  const matched = SEMVER_PATTERN.exec(version);
  if (matched == null) {
    throw new Error(`Not a valid semver version: ${version}`);
  }
  const [, major, minor, patch, prerelease] = matched;
  return {
    major: Number(major),
    minor: Number(minor),
    patch: Number(patch),
    prerelease: prerelease == null || prerelease === '' ? [] : prerelease.split('.'),
  };
}

/**
 * Compare two versions by semver precedence. Build metadata is ignored.
 * Returns a negative number when `a` precedes `b`, positive when it follows, 0 when equal.
 */
export function compareSemver(a: string, b: string): number {
  const left = parseSemver(a);
  const right = parseSemver(b);
  if (left.major !== right.major) {
    return left.major - right.major;
  }
  if (left.minor !== right.minor) {
    return left.minor - right.minor;
  }
  if (left.patch !== right.patch) {
    return left.patch - right.patch;
  }
  return comparePrerelease(left.prerelease, right.prerelease);
}

/**
 * A version with a prerelease precedes the same version without one, and identifiers are compared
 * field by field: numeric ones numerically, and a numeric field always precedes an alphanumeric one.
 */
function comparePrerelease(a: string[], b: string[]): number {
  if (a.length === 0 || b.length === 0) {
    return b.length - a.length;
  }
  for (let index = 0; index < Math.max(a.length, b.length); index++) {
    const left = a[index];
    const right = b[index];
    if (left == null || right == null) {
      // Whichever ran out of identifiers first has lower precedence.
      return left == null ? -1 : 1;
    }
    const leftIsNumeric = /^\d+$/.test(left);
    const rightIsNumeric = /^\d+$/.test(right);
    if (leftIsNumeric !== rightIsNumeric) {
      return leftIsNumeric ? -1 : 1;
    }
    if (leftIsNumeric && rightIsNumeric) {
      if (Number(left) !== Number(right)) {
        return Number(left) - Number(right);
      }
      continue;
    }
    if (left !== right) {
      return left < right ? -1 : 1;
    }
  }
  return 0;
}

/**
 * Look up the version the `latest` dist-tag currently points at, or `null` when nothing is published.
 */
export async function fetchPublishedVersion(name: string): Promise<string | null> {
  const response = await fetch(`https://registry.npmjs.org/${encodeURIComponent(name)}/latest`);
  if (response.status === 404) {
    return null;
  }
  if (!response.ok) {
    throw new Error(`Cannot read the published version of ${name}: ${response.status} ${response.statusText}`);
  }
  const { version } = (await response.json()) as { version: string };
  return version;
}

/**
 * Guard the `latest` publish. npm assigns the `latest` dist-tag to whatever is published last with no
 * regard for semver order, so publishing an older version silently moves `latest` backwards.
 */
async function main() {
  const rootdir = path.join(path.dirname(fileURLToPath(import.meta.url)), '..');
  const { name, version } = JSON.parse(await fs.readFile(path.join(rootdir, 'package.json'), 'utf8'));

  const tag = process.env.GITHUB_REF_NAME;
  if (tag !== `v${version}`) {
    throw new Error(`Tag ${tag} does not match the package version ${version}. Bump package.json or retag.`);
  }

  const published = await fetchPublishedVersion(name);
  if (published == null) {
    console.info(`${name} has no published version yet. Releasing ${version}.`);
    return;
  }
  if (compareSemver(version, published) <= 0) {
    throw new Error(
      `Refusing to release ${version}: it is not newer than the published ${published}. ` +
        'Publishing it would move the `latest` dist-tag backwards.'
    );
  }

  console.info(`Releasing ${version} over the published ${published}.`);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await main();
}
