# es-git 소개

es-git은 Node.js를 위한 현대적인 git 라이브러리예요. 간편하고 직관적인 인터페이스 덕분에 복잡한 git 작업도 쉽게 통합할 수 있으며, TypeScript 타입을 내장해 빠르고 안정적인 개발을
지원해요.

es-git은 libgit2를 바인딩한 [git2-rs](https://github.com/rust-lang/git2-rs)를 [napi-rs](https://napi.rs/)를 사용해 네이티브 모듈로 빌드하여,
운영체제와 상관없이 고성능 git 기능을 제공해요.

또한, node-gyp를 사용하지 않고 운영체제 및 cpu 아키텍처에 맞는 사전 빌드된 네이티브 모듈을 제공하기 때문에 사용자 환경에서 빌드하지 않고 간편하게
모듈을 설치할 수 있어요.

## 링크

이 프로젝트에 대해서 더 많은 정보를 얻기 위해서는 아래 링크를 참고하세요.

- [GitHub](https://github.com/toss/es-git)
