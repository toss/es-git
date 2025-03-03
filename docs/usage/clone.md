# Cloning a Repository

Use [`cloneRepository`](../api/functions/cloneRepository.md) to copy an existing repository. To clone repository from a remote, you can use protocols such as `https://`, `git://`, or SSH (e.g.,
`user@server:path/to/repo.git`).

```ts
import { cloneRepository } from 'es-git';

const repo = await cloneRepository('https://github.com/toss/es-git', '/path/to/clone');
```

## Authentication

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
