---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: 'es-git'
  text: 'Node.js를 위한 현대적인 Git 라이브러리'
  image:
    loading: eager
    fetchpriority: high
    decoding: async
    src: /hero.webp
    alt:
  actions:
    - theme: brand
      text: es-git 시작하기
      link: /ko/getting-started
    - theme: alt
      text: 사용법
      link: /ko/usage/repository
    - theme: alt
      text: 레퍼런스
      link: /ko/reference/Repository/openRepository

features:
  - title: 쉬운 Git 통합
    details: es-git은 Git 기능을 Node.js 환경에서 쉽게 사용할 수 있도록 제공해요.
  - title: 고성능 Git 작업
    details: es-git은 libgit2를 기반으로 하며, 네이티브 모듈을 사용해 빠르고 매끄럽게 실행돼요.
  - title: 다양한 운영 체제 지원
    details: es-git은 napi.rs를 사용해 빌드되며, Windows, macOS, Linux에서 모두 호환돼요.
  - title: 간편한 설치 및 유지보수
    details: 사전 빌드된 바이너리를 제공하여 설치가 간단해요.
  - title: 강력한 타입 지원
    details: 모든 기능에 대해 간단하지만 견고한 타입을 제공해요.
---
