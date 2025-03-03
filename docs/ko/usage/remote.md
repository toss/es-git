# 리모트

## 리모트 조회하기

현재 리포지토리에 등록된 리모트 이름을 조회하기 위해 [`remoteNames()`](../api/classes/Repository.md#remotenames)를 사용해요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
console.log(repo.remoteNames()); // ["origin"]
```

리모트에 접근하기 위해서 [`getRemote()`](../api/classes/Repository.md#getremote)를 사용해요. 만약 입력한 이름에 해당하는 리모트가 없으면
오류를 발생시켜요. 안전하게 리모트를 가져오고 싶다면 [`findRemote()`](../api/classes/Repository.md#findremote)를 대신 사용할 수 있어요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const remote = repo.getRemote('origin');
console.log(remote.name()); // "origin"
console.log(remote.url()); // "https://github.com/toss/es-git"

// 이 리모트에 설정된 Refspec 조회하기
const refspecs = remote.refspecs();
console.log(refspecs[0]);
// "+refs/heads/*:refs/remotes/origin/*" 스펙에 대한 설정값
// {
//   "direction": "Fetch",
//   "src": "refs/heads/*",
//   "dst": "refs/remotes/origin/*",
//   "force": true
// }
```

## 리모트를 Fetch 하기

리모트 리포지토리에 있는 데이터를 가져오기 위해서 [`fetch()`](../api/classes/Remote.md#fetch)를 사용해요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const remote = repo.getRemote('origin');

// "main" 브랜치의 데이터를 가져오기
await remote.fetch(['main']);

// 빈 배열을 입력하면 리모트에 설정된 기본 Refspec 으로 데이터를 가져와요
await remote.fetch([]);

// 리모트 리포지토리의 기본 브래치를 불러올 때 fetch 동작도 같이 수행돼요
const branch = await remote.defaultBranch();
console.log(branch); // "refs/heads/main"
```

## 리모트에 Push 하기

로컬 변경사항을 리모트 리포지토리로 올리기 위해서 [`push()`](../api/classes/Remote.md#push)를 사용해요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const remote = repo.getRemote('origin');

// 로컬 "main" 브랜치를 리모트 "other" 브랜치로 Push 하기
await remote.push(['refs/heads/main:refs/heads/other']);
```

만약, 리모트 리포지토리에 쓰기 권한이 필요한 경우 인증 옵션을 추가할 수 있어요. 아래 예시는 GitHub 저장소로 Push 하기
위해 [개인용 액세스 토큰](https://docs.github.com/ko/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
을 비밀번호로 지정해요.

```ts
await remote.push(['refs/heads/main:refs/heads/other'], {
  credential: {
    type: 'Plain',
    password: '<personal access token>',
  },
});
```
