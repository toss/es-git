# Contributing

We welcome contribution from everyone in the community. All communications in this repository will be in English.

> Every contributor to es-git should adhere to our Code of Conduct. Please read the [full text](./CODE_OF_CONDUCT.md) to
> understand what actions will and will not be tolerated.

## Issues

You can contribute to es-git via:

- Improving our [docs](https://es-git.slash.page)
- [Reporting a bug in our issues tab](https://github.com/toss/es-git/issues/new/choose)
- [Requesting a new feature or package](https://github.com/toss/es-git/issues/new/choose)
- [Having a look at our issue list](https://github.com/toss/es-git/issues) to see what's to be fixed

## Development

Building this project requires a stable Rust toolchain, which can be installed
using [rustup](https://www.rust-lang.org/tools/install).

Clone the repository with follow command:

```shell
git clone https://github.com/toss/es-git
cd es-git
```

And then, install [Just](https://github.com/casey/just#packages) to run scripts and tasks.

Once installed, run the following command install the required tools:

```shell
just setup
```

### Development Commands

```shell
# Format all files
just format

# Run tests
just test

# Lint code
just lint

# Typecheck
just typecheck

# Open docs
just docs
```

## Pull Requests

Please open a Pull Request to merge changes.

Since we use squash merge for PRs, the commit message is not important. However, please ensure that the PR title follows
the [conventional commit format](https://www.conventionalcommits.org/en/v1.0.0-beta.2/).

- `feat:` - for any new functionality additions
- `refactor:` - refactor of the code without change in functionality
- `fix:` - for any fixes that don't add new functionality
- `docs:` - if you only change documentation
- `test:` - if you only change tests
- `chore:` - anything else

Below are examples of well-formatted commits:

```
fix: into repo u32 repository open flags
feat(repository): add options for initialize repository
docs: fix link to website page
chore: upgrade vitest to v3
```
