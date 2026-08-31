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

## 서명된 커밋 생성하기

외부 서명은 두 단계로 진행해요. 먼저 서명되지 않은 커밋 내용을 만들고, 반환된 문자열의 UTF-8 바이트를 그대로 서명 백엔드에 전달해요. 그런 다음 반환된 서명으로 커밋을 저장해요.

`commitCreateBuffer()`가 반환한 문자열의 공백이나 줄바꿈을 변경하면 서명이 유효하지 않게 돼요. `commitSigned()`는 커밋을 객체 데이터베이스에 저장하지만 `HEAD`나 다른 레퍼런스를 갱신하지 않아요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const index = repo.index();
const treeOid = index.writeTree();
const tree = repo.getTree(treeOid);

const signature = { name: 'Seokju Na', email: 'seokju.me@toss.im' };
const commitContent = repo.commitCreateBuffer(tree, 'signed commit', {
  author: signature,
  committer: signature,
  parents: [repo.head().target()!],
});

// 사용하는 GPG 또는 다른 Git 호환 서명 백엔드로 이 함수를 구현하세요.
const externalSignature = await signCommitContent(Buffer.from(commitContent, 'utf8'));
const oid = repo.commitSigned(commitContent, externalSignature);

const signatureInfo = repo.extractSignature(oid);
console.log(signatureInfo?.signature === externalSignature); // true
console.log(signatureInfo?.signedData === commitContent); // true
```

`commit()`이 생성할 커밋 내용과 정확히 일치하는 서명이 이미 있다면 `CommitOptions.signature`로 계속 전달할 수 있어요.
