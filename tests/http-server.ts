import { spawn, spawnSync } from 'node:child_process';
import http from 'node:http';
import type { AddressInfo } from 'node:net';
import { afterAll } from 'vitest';

const servers: http.Server[] = [];

afterAll(async () => {
  const closes = servers.map(server => new Promise(resolve => server.close(resolve)));
  await Promise.allSettled(closes);
});

export interface AuthServer {
  url: string;
  /** `Authorization` headers received, in order. `null` for requests without one. */
  authorizations: Array<string | null>;
}

interface BasicAuth {
  username: string;
  password: string;
}

export function basicAuthorization({ username, password }: BasicAuth) {
  return `Basic ${Buffer.from(`${username}:${password}`).toString('base64')}`;
}

async function listen(handler: http.RequestListener): Promise<string> {
  const server = http.createServer(handler);
  servers.push(server);
  await new Promise<void>(resolve => server.listen(0, '127.0.0.1', resolve));
  const { port } = server.address() as AddressInfo;
  return `http://127.0.0.1:${port}`;
}

function rejectUnauthorized(res: http.ServerResponse) {
  res.writeHead(401, { 'WWW-Authenticate': 'Basic realm="es-git"' });
  res.end();
}

/**
 * HTTP server that asks for authentication on every request and never accepts any.
 *
 * Needs nothing but Node.js, so it runs on every test target.
 */
export async function createRejectingServer(): Promise<AuthServer> {
  const authorizations: Array<string | null> = [];
  const url = await listen((req, res) => {
    authorizations.push(req.headers.authorization ?? null);
    req.resume();
    rejectUnauthorized(res);
  });
  return { url, authorizations };
}

export const hasGitHttpBackend =
  spawnSync('git', ['http-backend'], { env: { ...process.env, REQUEST_METHOD: 'GET', PATH_INFO: '/' } }).status !==
  null;

/**
 * Smart HTTP git server for the bare repositories under `root`, behind Basic authentication.
 *
 * Serves through `git http-backend`, so it needs the `git` command line installed.
 */
export async function createGitHttpServer(root: string, auth: BasicAuth): Promise<AuthServer> {
  const authorizations: Array<string | null> = [];
  const expected = basicAuthorization(auth);
  const url = await listen((req, res) => {
    authorizations.push(req.headers.authorization ?? null);
    if (req.headers.authorization !== expected) {
      req.resume();
      rejectUnauthorized(res);
      return;
    }
    const { pathname, search } = new URL(req.url ?? '/', 'http://localhost');
    const backend = spawn('git', ['http-backend'], {
      env: {
        ...process.env,
        GIT_PROJECT_ROOT: root,
        GIT_HTTP_EXPORT_ALL: '1',
        PATH_INFO: pathname,
        QUERY_STRING: search.slice(1),
        REQUEST_METHOD: req.method ?? 'GET',
        CONTENT_TYPE: req.headers['content-type'] ?? '',
        // Setting a user enables `git push` (receive-pack).
        REMOTE_USER: auth.username,
        REMOTE_ADDR: '127.0.0.1',
      },
    });
    req.pipe(backend.stdin);
    const chunks: Buffer[] = [];
    backend.stdout.on('data', chunk => chunks.push(chunk));
    backend.on('close', () => {
      const output = Buffer.concat(chunks);
      const separator = output.indexOf('\r\n\r\n');
      const headerLines = output.subarray(0, separator).toString().split('\r\n');
      let status = 200;
      const headers: Record<string, string> = {};
      for (const line of headerLines) {
        const index = line.indexOf(':');
        const name = line.slice(0, index).trim();
        const value = line.slice(index + 1).trim();
        if (name.toLowerCase() === 'status') {
          status = Number.parseInt(value, 10);
        } else {
          headers[name] = value;
        }
      }
      res.writeHead(status, headers);
      res.end(output.subarray(separator + 4));
    });
  });
  return { url, authorizations };
}
