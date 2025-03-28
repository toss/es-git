# 시작하기

es-git은 Node.js용 현대적인 Git 라이브러리예요. 간편한 인터페이스로 복잡한 Git 작업도 쉽게 통합할 수 있으며, TypeScript 타입을 지원해서 빠르고 안정적인 개발이 가능해요.

## es-git을 사용해야하는 이유

Node.js에서 Git을 다루는데 있어 좋은 선택지가 없었어요. 보통, Git을 다룰 때 2가지 옵션이 있어요.

#### `child_process`를 사용해서 git 커맨드 실행하기

자식 프로세스를 spawn하기 때문에 느리고 stdout을 파싱해서 결과를 가져오는 과정은 꽤 고통스러워요.

#### `nodegit` 사용하기

Nodegit도 es-git과 마찬가지로 libgit2를 사용하지만, `node-gyp`를 이용해 빌드하기 때문에, 만약 현재 사용중인 Node.js 버전에 해당하는 사전 빌드된 네이티브 모듈이 없으면 로컬에서 소스 코드를 직접 빌드해야해요.

빌드 과정에서 사용중인 아키텍처 혹은 설치된 의존성 유무에 따라 빌드가 실패하기 때문에 nodegit은 설치부터 힘든 경우가 많아요.

---

`es-git`은 Node.js에서 Git을 사용하는 데 있어 좋은 선택지가 될 수 있어요.

#### 🚀 쉬운 사용

복잡한 Git 기능을 쉬운 API를 이용해 사용할 수 있어요. 또한, TypeScript 타입을 지원해 빠르고 안정적인 개발이 가능해요.

#### ⚡ 고성능 Git

es-git은 `libgit2`를 기반으로 네이티브 모듈을 제공하기 때문에 높은 성능을 가지고 있어요. `child_process`로 git 커맨드 실행하는 것 대비 최대 100배 정도 빨라요.

#### 🔧 고통없는 설치

es-git은 `node-gyp`를 사용하지 않고 [napi.rs](https://napi.rs/)를 사용해 사전 빌드된 네이티브 모듈을 설치해요. 사용자 환경에서 빌드하지 않기 때문에 빌드 실패로 설치조차 안되는 경험을 안해도 돼요.

#### 💻 크로스 플랫폼

Windows/macOS/Linux 운영체제 모두 호환해요.

## 설치

`es-git`은 Node.js 10.20 또는 이후 버전을 지원해요. `es-git`을 설치하려면 다음 명령어를 실행하세요.

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

## 링크

더 많은 정보는 아래 링크에서 확인하세요.

- [GitHub](https://github.com/toss/es-git)
