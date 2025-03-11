# 커밋하기

변경사항을 커밋하는 간단한 예제를 소개합니다. 아래 예시 코드는 현재 작업중인 브랜치에 새로운 커밋을 생성해요.

```ts
import { openRepository } from 'es-git';
import fs from 'node:fs/promises';

const repo = await openRepository('.');
 
await fs.writeFile('README.md', 'Hello World!', 'utf8');

const index = repo.index();
index.addPath('README.md');
index.write();

const tree = repo.head().peelToTree();
const signature = { name: 'Seokju Na', email: 'seokju.me@toss.im' };
const sha = repo.commit(tree, 'added new file', {
  author: signature,
  committer: signature,
});

const commit = repo.getCommit(sha);
console.log(commit.summary()); // "added new file"
```

`git add *` 명령어처럼 Staging Area에 전체 파일을 Stage하고 싶다면, [`addAll()`](../reference/classes/Index.md#addall)를 사용할 수 있어요.

```ts
const index = repo.index();
index.addAll(['*']);
index.write();
```
