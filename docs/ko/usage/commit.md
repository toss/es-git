# 커밋

변경사항을 커밋하는 간단한 예제를 소개합니다. 아래 예시 코드는 현재 작업중인 브랜치에 새로운 커밋을 생성해요.

```ts
import { openRepository } from 'es-git';
import fs from 'node:fs/promises';

const repo = await openRepository('.');
 
await fs.writeFile('README.md', 'Hello World!', 'utf8');

const treeOid = index.writeTree();
const tree = repo.getTree(treeOid);

const signature = { name: 'Seokju Na', email: 'seokju.me@toss.im' };
const oid = repo.commit(tree, 'added new file', {
  updateRef: 'HEAD',
  author: signature,
  committer: signature,
  parents: [repo.head().target()!],
});

const commit = repo.getCommit(oid);
console.log(commit.summary()); // "added new file"
```

`git add *` 명령어처럼 Staging Area에 전체 파일을 Stage하고 싶다면, [`addAll()`](../reference/Index/Methods/addAll.md)를 사용할 수 있어요.

```ts
const index = repo.index();
index.addAll(['*']);
index.write();
```
