[es-git](../globals.md) / RepositoryInitOptions

# 인터페이스: RepositoryInitOptions

Options which can be used to configure how a repository is initialized

## 속성

| 속성 | 유형 | 설명 |
| ------ | ------ | ------ |
| <a id="bare"></a> `bare?` | `boolean` | Create a bare repository with no working directory. Defaults to `false`. |
| <a id="noreinit"></a> `noReinit?` | `boolean` | Return an error if the repository path appears to already be a git repository. Defaults to `false`. |
| <a id="nodotgitdir"></a> `noDotgitDir?` | `boolean` | Normally a '/.git/' will be appended to the repo path for non-bare repos (if it is not already there), but passing this flag prevents that behavior. Defaults to `false`. |
| <a id="mkdir"></a> `mkdir?` | `boolean` | Make the repo path (and workdir path) as needed. The ".git" directory will always be created regardless of this flag. Defaults to `true`. |
| <a id="mkpath"></a> `mkpath?` | `boolean` | Recursively make all components of the repo and workdir path as necessary. Defaults to `true`. |
| <a id="externaltemplate"></a> `externalTemplate?` | `boolean` | Enable or disable using external templates. If enabled, then the `template_path` option will be queried first, then `init.templatedir` from the global config, and finally `/usr/share/git-core-templates` will be used (if it exists). Defaults to `true`. |
| <a id="initialhead"></a> `initialHead?` | `string` | The name of the head to point HEAD at. If not configured, this will be taken from your git configuration. If this begins with `refs/` it will be used verbatim; otherwise `refs/heads/` will be prefixed |
| <a id="originurl"></a> `originUrl?` | `string` | If set, then after the rest of the repository initialization is completed an `origin` remote will be added pointing to this URL. |
