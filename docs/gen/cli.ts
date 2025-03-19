import { Cli } from 'clipanion';
import { ReferenceCommand } from './commands/reference';

const [node, app, ...args] = process.argv;

const cli = new Cli({
  binaryLabel: 'documentation gen',
  binaryName: `${node} ${app}`,
  binaryVersion: '0.0.0',
});

cli.register(ReferenceCommand);
cli.runExit(args);
