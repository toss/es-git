import { exec as execChildProcess } from 'node:child_process';

export function exec(command: string, cwd: string) {
  return new Promise<string>((resolve, reject) => {
    let output = '';
    const cp = execChildProcess(
      command,
      {
        encoding: 'utf8',
        cwd,
      },
      (err, stdout) => {
        if (err != null) {
          reject(err);
          return;
        }
        output += stdout;
      }
    );
    cp.on('close', () => resolve(output));
  });
}
