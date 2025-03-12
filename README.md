# es-git &middot; [![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/toss/es-git/blob/main/LICENSE) [![NPM badge](https://img.shields.io/npm/v/es-git?logo=npm)](https://www.npmjs.com/package/es-git) 

English | [한국어](https://github.com/toss/es-git/blob/main/README-ko_kr.md)

es-git is a modern git library built for Node.js. With its simple and intuitive interface, even complex git operations
can be easily integrated, and the built-in TypeScript types ensure fast and reliable development.

es-git builds high-performance git functionality across different operating systems by using [napi-rs](https://napi.rs/)
to compile a native module from [git2-rs](https://github.com/rust-lang/git2-rs), which binds to libgit2.

Furthermore, by offering pre-built native modules tailored to your OS and CPU architecture without relying
on node-gyp, the module can be installed effortlessly without the need to build it locally.

## Examples

```ts
import { cloneRepository } from 'es-git';

const repo = await cloneRepository('https://github.com/toss/es-git', '/path/to/clone');
const head = repo.head().name();
console.log(head); // "refs/heads/main"
```

## Documentation

- [Usage](https://es-git.slash.page/usage/repository.html)
- [Reference](https://es-git.slash.page/reference/globals.html)

## Contributing

We welcome contribution from everyone in the community. Read below for detailed contribution guide.

[CONTRIBUTING](https://github.com/toss/es-git/blob/main/.github/CONTRIBUTING.md)

## License

MIT © Viva Republica, Inc. See [LICENSE](./LICENSE) for details.

<a title="Toss" href="https://toss.im">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://static.toss.im/logos/png/4x/logo-toss-reverse.png">
    <img alt="Toss" src="https://static.toss.im/logos/png/4x/logo-toss.png" width="100">
  </picture>
</a>
