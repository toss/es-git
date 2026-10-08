# Repository

## Opening a Repository

To open a locally stored repository, use [`openRepository()`](../reference/Repository/openRepository.md).

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
```

## Cloning a Repository

Use [`cloneRepository()`](../reference/Repository/cloneRepository.md) to copy an existing repository. To clone repository from a remote, you can use protocols such as `https://`, `git://`, or SSH (e.g.,
`user@server:path/to/repo.git`).

```ts
import { cloneRepository } from 'es-git';

const repo = await cloneRepository('https://github.com/toss/es-git', '/path/to/clone');
```

### Authentication

When cloning a repository, you can configure the `credential` option to authenticate.

```ts
import { cloneRepository } from 'es-git';

// Authenticate using ssh-agent
const cloneWithSshAgent = await cloneRepository('git@github.com:toss/es-git', '.', {
  fetch: {
    credential: {
      type: 'SSHKeyFromAgent',
    },
  },
});

// Authenticate using a local SSH key file
const cloneWithSshKeyFromPath = await cloneRepository('git@github.com:toss/es-git', '.', {
  fetch: {
    credential: {
      type: 'SSHKeyFromPath',
      privateKeyPath: '/path/to/ssh/private/key',
    },
  },
});

// Authenticate using an inline SSH key
const cloneWithSshKey = await cloneRepository('git@github.com:toss/es-git', '.', {
  fetch: {
    credential: {
      type: 'SSHKey',
      privateKey: 'MY_PRIVATE_SSH_KEY',
    },
  },
});

// Authenticate using a plain password
const cloneWithPlain = await cloneRepository('https://github.com/toss/es-git', '.', {
  fetch: {
    credential: {
      type: 'Plain',
      password: 'MY_PASSWORD',
    },
  },
});
```

If you're using a GitHub [Personal Access Token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens), you can clone private repositories by specifying the `Plain` credential type.

```ts
import { cloneRepository } from 'es-git';

const repo = await cloneRepository('https://github.com/<owner>/<repo>', '.', {
  fetch: {
    credential: {
      type: 'Plain',
      password: '<personal access token>',
    },
  },
});
```

#### Choosing a credential when asked

`credential` also accepts a function. It is called only when the server asks for authentication, with the remote
`url`, the `usernameFromUrl` (for example `git` in `git@github.com:toss/es-git`, or `null`) and the `allowedTypes`
the server accepts. Use it to pick a credential per host, or to read a token lazily. It may return the credential
or a promise for it.

```ts
import { cloneRepository } from 'es-git';

const repo = await cloneRepository('https://github.com/<owner>/<repo>', '.', {
  fetch: {
    credential: async ({ url, usernameFromUrl, allowedTypes }) => {
      if (allowedTypes.includes('SSHKeyFromAgent')) {
        return { type: 'SSHKeyFromAgent', username: usernameFromUrl ?? 'git' };
      }
      const token = await readTokenFor(new URL(url).host);
      // Returning `null` gives up and fails the clone.
      return token != null ? { type: 'Plain', password: token } : null;
    },
  },
});
```

The function is called again whenever the server rejects the credential it returned. Return `null` or `undefined`,
or throw, to give up; the operation then fails with that reason. After 10 calls the operation fails on its own.
The operation waits for the function to answer, so make sure a returned promise settles.
