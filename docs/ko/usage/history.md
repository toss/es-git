# 커밋 히스토리

커밋 히스토리를 조회하기 위해 [`revwalk`](../api/classes/Repository.md#revwalk)로 [`Revwalk`](../api/classes/Revwalk.md)를 생성할 수 있어요.

[`Revwalk`](../api/classes/Revwalk.md)는 [순회 프로토콜](https://developer.mozilla.org/ko/docs/Web/JavaScript/Reference/Iteration_protocols)을 구현하고 있기 때문에 [
`for...of`](https://developer.mozilla.org/ko/docs/Web/JavaScript/Reference/Statements/for...of) 명령문을 사용해 쉽게 커밋 히스토리를 순회할 수 있어요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');

/**
 * 예시로 이 리포지토리의 커밋 히스토리는 다음과 같다고 해볼게요.
 * ❯ git log --pretty=format:'%s %H %an <%ae>'
 * E 2b2278987ac96270325a2281499cced99a2a41c0 Seokju Na <seokju.me@toss.im>
 * D 2091c70eaf9d0b36c8a063d4eaa75e522b31fc6b Seokju Na <seokju.me@toss.im>
 * C 0ed417bb37612320099ac013cdf2b0c2614a7afc Seokju Na <seokju.me@toss.im>
 * B 8761d9d89b1198d913bb31fede8c45405fa16ef3 Seokju Na <seokju.me@toss.im>
 * A 38b79dc02462eedc463fa3028ed39ac4d0339608 Seokju Na <seokju.me@toss.im>
 */

// HEAD가 가리키는 커밋부터 순서대로 읽어요.
const revwalk = repo.revwalk().pushHead();

// Revwalk를 순회하며 커밋 해시를 읽어요.
// 아래 코드는 위에서 사용한 `git log` 명령과 동일한 결과를 출력해요.
for (const sha of revwalk) {
  const commit = repo.getCommit(sha);
  const summary = commit.summary();
  const id = commit.id();
  const author = commit.author();
  console.log(`${summary} ${id} ${author.name} <${author.email}>`);
}
```

조회할 커밋의 범위를 설정하고 싶다면, [`pushRange`](../api/classes/Revwalk.md#pushrange) 메소드를 사용할 수 있어요.

`<commit>..<commit>` 포맷으로 범위를 설정할 수 있어요. 범위값에서 오른쪽 커밋부터 순회가 진행되고 왼쪽 커밋과 그 이후의 커밋들은 순회 범위에 포함되지 않아요.

```ts
const revwalk = repo.revwalk().pushRange('8761d9d..2b22789');

for (const sha of revwalk) {
  const commit = repo.getCommit(sha);
  const summary = commit.summary();
  const id = commit.id();
  const author = commit.author();
  console.log(`${summary} ${id} ${author.name} <${author.email}>`);
}

/**
 * 위 동작은 아래와 같이 출력해요.
 * E 2b2278987ac96270325a2281499cced99a2a41c0 Seokju Na <seokju.me@toss.im>
 * D 2091c70eaf9d0b36c8a063d4eaa75e522b31fc6b Seokju Na <seokju.me@toss.im>
 * C 0ed417bb37612320099ac013cdf2b0c2614a7afc Seokju Na <seokju.me@toss.im>
 */
```
