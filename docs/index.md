---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: 'es-git'
  text: 'Modern Git library built for Node.js'
  image:
    loading: eager
    fetchpriority: high
    decoding: async
    src: /hero.webp
    alt:
  actions:
    - theme: brand
      text: Get Started
      link: /getting-started
    - theme: alt
      text: Usage
      link: /usage/repository
    - theme: alt
      text: Reference
      link: /reference/Repository/openRepository

features:
  - title: Easy Git Integration
    details: es-git integrates Git functionality into Node.js environment.
  - title: High-Performance Git Operations
    details: es-git build on `libgit2`, and uses a native module for fast and seamless execution.
  - title: Cross-Platform Compatibility
    details: es-git powered by `napi-rs` to ensure compatibility with Windows, macOS, and Linux.
  - title: Easy Installation & Maintenance
    details: es-git provided prebuilt binaries, making installation straightforward.
  - title: Robust Types
    details: es-git offers simple yet robust types for all functions.
---
