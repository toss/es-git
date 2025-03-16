# cloneRepository

Clone a remote repository.

This will use the options configured so far to clone the specified URL
into the specified local path.

---

## Signature

```ts
function cloneRepository(url: string, path: string, options?: RepositoryCloneOptions | null, signal?: AbortSignal | null): Promise<Repository>;
```

### Parameters

- `url` (`string`):
- `path` (`string`):
- `options` (`RepositoryCloneOptions` | `null`): 
- `signal` (`AbortSignal` | `null`): 

### Returns

(`Repository`) Cloned repository instance from remote.

## Examples

```ts

```