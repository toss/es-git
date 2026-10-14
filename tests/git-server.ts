import { type ChildProcess, spawn, spawnSync } from 'node:child_process';
import http from 'node:http';
import type { AddressInfo } from 'node:net';

export const hasGit = spawnSync('git', ['--version']).status === 0;

/**
 * Serve git repositories under `root` over the smart HTTP protocol, using `git http-backend`.
 *
 * libgit2's local transport (local paths and `file://` URLs) does not support shallow fetches,
 * so tests for shallow clones need a real protocol.
 */
export async function serveGitRepositories(root: string) {
  const children = new Set<ChildProcess>();
  const server = http.createServer((req, res) => {
    const url = new URL(req.url ?? '/', 'http://localhost');
    const child = spawn('git', ['http-backend'], {
      env: {
        ...process.env,
        GIT_PROJECT_ROOT: root,
        GIT_HTTP_EXPORT_ALL: '1',
        PATH_INFO: url.pathname,
        QUERY_STRING: url.search.slice(1),
        REQUEST_METHOD: req.method,
        CONTENT_TYPE: req.headers['content-type'] ?? '',
      },
    });
    children.add(child);
    child.on('close', () => children.delete(child));
    child.on('error', error => {
      res.destroy(error);
    });
    // Ignore EPIPE when the backend exits before reading the whole request; 'error' above handles failures.
    child.stdin.on('error', () => {});
    req.pipe(child.stdin);

    // The CGI response is headers, a blank line, then the (binary) body.
    let head: Buffer | null = Buffer.alloc(0);
    child.stdout.on('data', (chunk: Buffer) => {
      if (head == null) {
        res.write(chunk);
        return;
      }
      head = Buffer.concat([head, chunk]);
      const end = head.indexOf('\r\n\r\n');
      if (end === -1) {
        return;
      }
      let status = 200;
      for (const line of head.subarray(0, end).toString('utf8').split('\r\n')) {
        const separator = line.indexOf(':');
        const name = line.slice(0, separator).trim();
        const value = line.slice(separator + 1).trim();
        if (name.toLowerCase() === 'status') {
          status = Number.parseInt(value, 10);
        } else {
          res.setHeader(name, value);
        }
      }
      res.writeHead(status);
      res.write(head.subarray(end + 4));
      head = null;
    });
    child.stdout.on('end', () => {
      if (head != null) {
        res.destroy(new Error('git http-backend exited without a response'));
        return;
      }
      res.end();
    });
  });

  await new Promise<void>(resolve => server.listen(0, '127.0.0.1', resolve));
  const { port } = server.address() as AddressInfo;

  return {
    url: `http://127.0.0.1:${port}`,
    close: async () => {
      for (const child of children) {
        child.kill();
      }
      server.closeAllConnections();
      await new Promise(resolve => server.close(resolve));
    },
  };
}
