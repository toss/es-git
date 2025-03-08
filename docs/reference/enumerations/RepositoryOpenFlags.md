[es-git](../globals.md) / RepositoryOpenFlags

# Enumeration: RepositoryOpenFlags

Flags for opening repository

## Enumeration Members

| Enumeration Member | Value | Description |
| ------ | ------ | ------ |
| <a id="nosearch"></a> `NoSearch` | `1` | Only open the specified path; don't walk upward searching. |
| <a id="crossfs"></a> `CrossFS` | `2` | Search across filesystem boundaries. |
| <a id="bare"></a> `Bare` | `4` | Force opening as a bare repository, and defer loading its config. |
| <a id="nodotgit"></a> `NoDotGit` | `8` | Don't try appending `/.git` to the specified repository path. |
| <a id="fromenv"></a> `FromEnv` | `16` | Respect environment variables like `$GIT_DIR`. |
