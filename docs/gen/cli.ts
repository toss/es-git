import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { Cli } from 'clipanion';
import dotenv from 'dotenv';
import { ReferenceCommand } from './commands/reference';

const dirname = path.dirname(fileURLToPath(import.meta.url));
dotenv.config({ path: path.join(dirname, '..', '.env') });

const [node, app, ...args] = process.argv;

const cli = new Cli({
  binaryLabel: 'documentation gen',
  binaryName: `${node} ${app}`,
  binaryVersion: '0.0.0',
});

cli.register(ReferenceCommand);
cli.runExit(args);
