[es-git](../globals.md) / RepositoryInitMode

# Enumeration: RepositoryInitMode

Mode options for `RepositoryInitOptions`.

## Enumeration Members

| Enumeration Member | Value | Description |
| ------ | ------ | ------ |
| <a id="sharedunmask"></a> `SharedUnmask` | `0` | Use permissions configured by umask (default) |
| <a id="sharedgroup"></a> `SharedGroup` | `1533` | Use `--shared=group` behavior, chmod'ing the new repo to be group writable and "g+sx" for sticky group assignment. |
| <a id="sharedall"></a> `SharedAll` | `1535` | Use `--shared=all` behavior, adding world readability. |
