[es-git](../globals.md) / RepositoryOpenOptions

# Interface: RepositoryOpenOptions

Options which can be used to configure how a repository is initialized.

## Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="flags"></a> `flags` | `number` | If flags contains `RepositoryOpenFlags.NoSearch`, the path must point directly to a repository; otherwise, this may point to a subdirectory of a repository, and `open` will search up through parent directories. If flags contains `RepositoryOpenFlags.CrossFS`, the search through parent directories will not cross a filesystem boundary (detected when the stat st_dev field changes). If flags contains `RepositoryOpenFlags.Bare`, force opening the repository as bare even if it isn't, ignoring any working directory, and defer loading the repository configuration for performance. If flags contains `RepositoryOpenFlags.NoDotgit`, don't try appending `/.git` to `path`. If flags contains `RepositoryOpenFlags.FromEnv`, `open` will ignore other flags and `ceilingDirs`, and respect the same environment variables git does. Note, however, that `path` overrides `$GIT_DIR`. |
| <a id="ceilingdirs"></a> `ceilingDirs?` | `string`[] | ceiling_dirs specifies a list of paths that the search through parent directories will stop before entering. |
