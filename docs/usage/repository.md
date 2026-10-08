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

### Shallow Clone

To clone only the latest part of the history, set `depth` in the `fetch` option. A depth of `1` fetches only the latest
commit of each branch. Use [`isShallow()`](../reference/Repository/Methods/isShallow.md) to check whether a repository is
shallow.

```ts
import { cloneRepository } from 'es-git';

const repo = await cloneRepository('https://github.com/toss/es-git', '/path/to/clone', {
  fetch: { depth: 1 },
});
console.log(repo.isShallow()); // true
```

::: warning
Shallow clones need a network protocol such as `https://`, `git://` or SSH. Cloning from a local path or a `file://` URL
with `depth` fails, because libgit2's local transport does not support shallow fetches.
:::

To fetch the rest of the history later, fetch from the remote with `unshallow: true`. This works like
`git fetch --unshallow`.

```ts
const remote = repo.getRemote('origin');
await remote.fetch([], { fetch: { unshallow: true } });
console.log(repo.isShallow()); // false
```

Fetching with a larger `depth` deepens the history instead. Fetching with `depth: 0` or without `depth` does not unshallow
a shallow repository.
