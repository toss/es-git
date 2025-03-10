[es-git](../globals.md) / RepositoryInitOptions

# Interface: RepositoryInitOptions

## Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="bare"></a> `bare?` | `boolean` | Create a bare repository with no working directory. Defaults to `false`. |
| <a id="noreinit"></a> `noReinit?` | `boolean` | Return an error if the repository path appears to already be a git repository. Defaults to `false`. |
| <a id="nodotgitdir"></a> `noDotgitDir?` | `boolean` | Normally a '/.git/' will be appended to the repo path for non-bare repos (if it is not already there), but passing this flag prevents that behavior. Defaults to `false`. |
| <a id="mkdir"></a> `mkdir?` | `boolean` | Make the repo path (and workdir path) as needed. The ".git" directory will always be created regardless of this flag. Defaults to `true`. |
| <a id="mkpath"></a> `mkpath?` | `boolean` | Make the repo path (and workdir path) as needed. The ".git" directory will always be created regardless of this flag. Defaults to `true`. |
| <a id="mode"></a> `mode?` | `number` | Set to one of the `RepositoryInit` constants, or a custom value. |
| <a id="externaltemplate"></a> `externalTemplate?` | `boolean` | Enable or disable using external templates. If enabled, then the `template_path` option will be queried first, then `init.templatedir` from the global config, and finally `/usr/share/git-core-templates` will be used (if it exists). Defaults to `true`. |
| <a id="templatepath"></a> `templatePath?` | `string` | When the `externalTemplate` option is set, this is the first location to check for the template directory. If this is not configured, then the default locations will be searched instead. |
| <a id="workdirpath"></a> `workdirPath?` | `string` | The path to the working directory. If this is a relative path it will be evaluated relative to the repo path. If this is not the "natural" working directory, a .git gitlink file will be created here linking to the repo path. |
| <a id="description"></a> `description?` | `string` | If set, this will be used to initialize the "description" file in the repository instead of using the template content. |
| <a id="initialhead"></a> `initialHead?` | `string` | The name of the head to point HEAD at. If not configured, this will be taken from your git configuration. If this begins with `refs/` it will be used verbatim; otherwise `refs/heads/` will be prefixed. |
| <a id="originurl"></a> `originUrl?` | `string` | If set, then after the rest of the repository initialization is completed an `origin` remote will be added pointing to this URL. |
