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

#### 인증을 요청받을 때 credential 고르기

`credential`에는 함수도 지정할 수 있어요. 이 함수는 서버가 인증을 요청할 때만 호출되고, 리모트 `url`, URL에 들어 있는
`usernameFromUrl`(예: `git@github.com:toss/es-git`의 `git`, 없으면 `null`), 서버가 허용하는 `allowedTypes`를 받아요.
호스트마다 다른 credential을 쓰거나, 토큰을 필요할 때만 읽어오고 싶을 때 사용해요. credential을 바로 반환해도 되고, Promise로 반환해도 돼요.

```ts
import { cloneRepository } from 'es-git';

const repo = await cloneRepository('https://github.com/<owner>/<repo>', '.', {
  fetch: {
    credential: async ({ url, usernameFromUrl, allowedTypes }) => {
      if (allowedTypes.includes('SSHKeyFromAgent')) {
        return { type: 'SSHKeyFromAgent', username: usernameFromUrl ?? 'git' };
      }
      const token = await readTokenFor(new URL(url).host);
      // `null`을 반환하면 인증을 포기하고 클론이 실패해요.
      return token != null ? { type: 'Plain', password: token } : null;
    },
  },
});
```

서버가 반환한 credential을 거부하면 함수가 다시 호출돼요. `null`이나 `undefined`를 반환하거나 에러를 던지면 인증을 포기하고,
작업은 그 이유와 함께 실패해요. 10번 호출된 뒤에는 작업이 자동으로 실패해요. 작업은 함수의 응답을 기다리기 때문에, 반환한 Promise는
반드시 완료되어야 해요.
