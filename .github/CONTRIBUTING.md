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
# Build project
just build

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

### Generated Files

`index.js` and `index.d.ts` are written by the build, but they are committed to the repository:
`index.js` is the package entry point, and the [reference documentation](https://es-git.slash.page)
is generated from `index.d.ts`.

If you change the public API on the Rust side, run `just build` and commit the result together with
your change. CI builds the project and fails when what you committed differs from a fresh build.

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

## Releasing

Maintainers release from `main`.

1. Run the [Prepare release](https://github.com/toss/es-git/actions/workflows/prepare-release.yml)
   workflow and choose whether to raise the major, minor or patch version. It raises the version,
   writes the changelog section from the commits since the last release, thanks the outside
   contributors among them, rebuilds the binding and pushes a `release/vX.Y.Z` branch.
2. Open a pull request from that branch and merge it. Open it yourself rather than leaving it to a
   bot, because a pull request a bot opens gets no CI run.

Merging is what releases. A commit landing on `main` whose version has no tag yet is tagged, gets a
GitHub release built from its changelog section, and is published to npm. Every other push to `main`
publishes a prerelease under the `next` dist-tag instead, so there is nothing to do between releases.

A release is refused if the version is not newer than the one npm currently serves as `latest`,
which is what stops a release from moving the tag backwards. The binding is rebuilt during the
release and the freshly built one is what gets published, so what ships always matches the commit it
was built from.

## Documentation

This project aims to maintain high documentation quality.

Please refer to [docs/README.md](../docs/README.md) for instructions on writing documentation.

