# Getting Started

A modern Git library built for Node.js with blazing-fast installation and rock-solid stability, powered by N-API.

With its simple interface, you can easily integrate complex Git operations, and it supports TypeScript types for fast and reliable development.

## Why es-git?

There was no good option for handling Git in a Node.js environment. Usually, when you have to handle Git, there are two options:

#### Using `child_process` to execute git command

Since it spawns a child process, it tends to be slow, and parsing stdout to retrieve the results can be quite painful.

#### Using `nodegit`

Nodegit also uses libgit2 like es-git, but since it builds using `node-gyp`, if there is no prebuilt for the Node.js version you are currently using, you will need to build the source code locally.

Depending on the architecture or the presence of required dependencies during the build process, the build may fail, making Nodegit a challenge to install from the start. 

---

`es-git` can be a good choice for using Git in Node.js.

#### 🚀 Ease of Use 

Access complex Git features through simple APIs. Additionally, it supports TypeScript types for fast and reliable development.

#### ⚡ High-Performance Git

es-git provides native modules built on libgit2, offering high performance. It can be up to 100 times faster than executing Git commands via `child_process`.

#### 🔧 Hassle-Free Installation

Since es-git makes prebuilt native modules with [napi.rs](https://napi.rs/) instead of `node-gyp`, there's no need to build native modules locally when installing the package. This means you won’t face installation failures due to build errors.

#### 💻 Cross-Platform

It is compatible with all Windows/macOS/Linux operating systems.

## Installation

`es-git` supports Node.js 10.20 or later. To install `es-git`, run one of the following commands:

::: code-group

```sh [npm]
npm install es-git
```

```sh [pnpm]
pnpm add es-git
```

```sh [yarn]
yarn add es-git
```

```sh [bun]
bun add es-git
```

:::

## Links

For more information, check the link below:

- [GitHub](https://github.com/toss/es-git)
