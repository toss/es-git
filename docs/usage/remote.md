# Remotes

## Viewing the Remotes

To check the names of remotes configured in the current repository, use [`remoteNames`](../api/classes/Repository.md#remotenames).

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
console.log(repo.remoteNames()); // ["origin"]
```

To get a remote, use [`getRemote`](../api/classes/Repository.md#getremote). If the remote for given name does not exist, it will throw an error. To safely get a remote, you can
use [`findRemote`](../api/classes/Repository.md#findremote) instead.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const remote = repo.getRemote('origin');
console.log(remote.name()); // "origin"
console.log(remote.url()); // "https://github.com/toss/es-git"

// Retrieving the Refspecs configured for this remote
const refspecs = remote.refspecs();
console.log(refspecs[0]);
// For the "+refs/heads/*:refs/remotes/origin/*" Refspec
// {
//   "direction": "Fetch",
//   "src": "refs/heads/*",
//   "dst": "refs/remotes/origin/*",
//   "force": true
// }
```

## Fetching from a Remote

To fetch data from a remote repository, use [`fetch`](../api/classes/Remote.md#fetch).

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const remote = repo.getRemote('origin');

// Fetching data from the "main" branch
await remote.fetch(['main']);

// Providing an empty array fetches data using the default Refspec configured for the remote
await remote.fetch([]);

// Fetching the default branch from the remote repository also performs a fetch operation
const branch = await remote.defaultBranch();
console.log(branch); // "refs/heads/main"
```

## Pushing to a Remote

To push local changes to a remote repository, use [`push`](../api/classes/Remote.md#push).

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const remote = repo.getRemote('origin');

// Push the local "main" branch to the remote "other" branch
await remote.push(['refs/heads/main:refs/heads/other']);
```

If authentication is required to write to the remote repository, you can add authentication options. The following example uses
a GitHub [Personal Access Token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens) as the password to push to a GitHub repository.

```ts
await remote.push(['refs/heads/main:refs/heads/other'], {
  credential: {
    type: 'Plain',
    password: '<personal access token>',
  },
});
```
