/// <reference types="vitest" />

import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { execa } from 'execa';
import type { TestProject } from 'vitest/node';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const tmpDir = path.join(dirname, 'tmp');

export async function setup(project: TestProject) {
  await fs.mkdir(tmpDir, { recursive: true });
  project.provide('tmpDir', tmpDir);

  const version = await install();
  project.provide('es-git.version', version);
}

export async function teardown() {
  await fs.rm(tmpDir, { recursive: true, force: true });
}

async function install() {
  const version = await getVersion();
  await execa('npm', ['install', `es-git@${version}`, '-E'], {
    cwd: dirname,
    stderr: 'inherit',
  });
  return version;
}

async function getVersion() {
  const { ES_GIT_VERSION } = process.env;
  if (ES_GIT_VERSION != null) {
    return ES_GIT_VERSION;
  }
  const rootPkgPath = path.join(dirname, '..', 'package.json');
  const rootPkgJsonRaw = await fs.readFile(rootPkgPath, 'utf8');
  const rootPkgJson = JSON.parse(rootPkgJsonRaw);
  return rootPkgJson.version;
}

declare module 'vitest' {
  export interface ProvidedContext {
    tmpDir: string;
    'es-git.version': string;
  }
}
