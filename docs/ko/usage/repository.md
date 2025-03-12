# 리포지토리

## 리포지토리 열기

로컬에 저장된 리포지토리를 열기 위해 [`openRepository()`](../reference/functions/openRepository.md)를 사용해요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
```

## 리포지토리 클론하기

기존 리포지토리를 클론하기 위해 [`cloneRepository()`](../reference/functions/cloneRepository.md)를 사용할 수 있어요. 리모트에서 클론하기 위해 `https://`, `git://` 혹은 `user@server:path/to/repo.git` 처럼
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
