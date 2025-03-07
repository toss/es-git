[es-git](../globals.md) / RepositoryOpenFlags

# Enumeration: RepositoryOpenFlags

## Enumeration Members

| Enumeration Member | Value | Description |
| ------ | ------ | ------ |
| <a id="nosearch"></a> `NoSearch` | `1` | Only open the specified path; don't walk upward searching. (1 << 0) |
| <a id="crossfs"></a> `CrossFS` | `2` | Search across filesystem boundaries. (1 << 1) |
| <a id="bare"></a> `Bare` | `4` | Force opening as a bare repository, and defer loading its config. (1 << 2) |
| <a id="nodotgit"></a> `NoDotGit` | `8` | Don't try appending `/.git` to the specified repository path. (1 << 3) |
| <a id="fromenv"></a> `FromEnv` | `16` | Respect environment variables like `$GIT_DIR`. (1 << 4) |
