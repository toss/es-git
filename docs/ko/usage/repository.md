# 리포지토리

## 리포지토리 열기

로컬에 저장된 리포지토리를 열기 위해 [`openRepository()`](../reference/Repository/openRepository.md)를 사용해요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
```

## 리포지토리 클론하기

기존 리포지토리를 클론하기 위해 [`cloneRepository()`](../reference/Repository/cloneRepository.md)를 사용할 수 있어요. 리모트에서 클론하기 위해 `https://`, `git://` 혹은 `user@server:path/to/repo.git` 처럼
SSH 프로토콜을 사용할 수 있어요.

```ts
import { cloneRepository } from 'es-git';

const repo = await cloneRepository('https://github.com/toss/es-git', '/path/to/clone');
```

### 인증하기

리포지토리를 클론할 때 `credential` 옵션을 설정해 인증이 가능해요.

```ts
import { cloneRepository } from 'es-git';

// ssh-agent를 통해 인증
const cloneWithSshAgent = await cloneRepository('git@github.com:toss/es-git', '.', {
  fetch: {
    credential: {
      type: 'SSHKeyFromAgent',
    },
  },
});

// 로컬에 저장된 ssh키 파일을 통해 인증
const cloneWithSshKeyFromPath = await cloneRepository('git@github.com:toss/es-git', '.', {
  fetch: {
    credential: {
      type: 'SSHKeyFromPath',
      privateKeyPath: '/path/to/ssh/private/key',
    },
  },
});

// ssh키를 입력해 인증
const cloneWithSshKey = await cloneRepository('git@github.com:toss/es-git', '.', {
  fetch: {
    credential: {
      type: 'SSHKey',
      privateKey: 'MY_PRIVATE_SSH_KEY',
    },
  },
});

// plain 비밀번호를 통해 인증
const cloneWithPlain = await cloneRepository('https://github.com/toss/es-git', '.', {
  fetch: {
    credential: {
      type: 'Plain',
      password: 'MY_PASSWORD',
    },
  },
});
```

GitHub [개인용 액세스 토큰](https://docs.github.com/ko/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
을 사용중이라면, "Plain" 유형의 `credential` 옵션을 지정해 비공개 리포지토리를 클론받을 수 있어요.

```ts
import { cloneRepository } from 'es-git';

const repo = await cloneRepository('https://github.com/<owner>/<repo>', '.', {
  fetch: {
    credential: {
      type: 'Plain',
      password: '<personal access token>',
    },
  },
});
```

### 얕은 클론하기

히스토리의 최근 일부만 클론하려면 `fetch` 옵션에 `depth`를 설정하세요. `depth`가 `1`이면 각 브랜치의 최신 커밋만
가져와요. 리포지토리가 얕은 리포지토리인지는 [`isShallow()`](../reference/Repository/Methods/isShallow.md)로 확인할 수 있어요.

```ts
import { cloneRepository } from 'es-git';

const repo = await cloneRepository('https://github.com/toss/es-git', '/path/to/clone', {
  fetch: { depth: 1 },
});
console.log(repo.isShallow()); // true
```

::: warning
얕은 클론은 `https://`, `git://`, SSH 같은 네트워크 프로토콜에서만 동작해요. libgit2의 로컬 전송은 얕은 fetch를
지원하지 않기 때문에, 로컬 경로나 `file://` URL에서 `depth`를 설정해 클론하면 실패해요.
:::

나머지 히스토리를 나중에 가져오려면 `unshallow: true`로 리모트에서 fetch하세요. `git fetch --unshallow`와 같아요.

```ts
const remote = repo.getRemote('origin');
await remote.fetch([], { fetch: { unshallow: true } });
console.log(repo.isShallow()); // false
```

더 큰 `depth`로 fetch하면 히스토리를 그만큼 더 가져와요. `depth: 0`으로 fetch하거나 `depth`를 지정하지 않으면 얕은
리포지토리가 전체 리포지토리로 바뀌지 않아요.
