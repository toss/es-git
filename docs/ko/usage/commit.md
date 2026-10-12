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

서명된 커밋을 만들고 현재 브랜치를 갱신하려면, 먼저 `commitCreateBuffer()`로 서명할 내용을 준비하고 GPG로 서명한 뒤 `commit()`에 서명과 `updateRef: 'HEAD'`를 전달해요. 아래 예제를 실행하려면 GPG와 서명 키가 설정되어 있어야 해요.

```ts
import { openRepository } from 'es-git';
import { execFileSync } from 'node:child_process';

const repo = await openRepository('.');
const index = repo.index();
const treeOid = index.writeTree();
const tree = repo.getTree(treeOid);

const timeOptions = { timestamp: Math.floor(Date.now() / 1000), offset: 0 };
const author = { name: 'Seokju Na', email: 'seokju.me@toss.im', timeOptions };
const committer = { name: 'Seokju Na', email: 'seokju.me@toss.im', timeOptions };
const parents = [repo.head().target()!];
const message = 'signed commit';

const content = repo.commitCreateBuffer(tree, message, { author, committer, parents });
const signature = execFileSync('gpg', ['--detach-sign', '--armor'], {
  input: Buffer.from(content, 'utf8'),
}).toString('utf8');

const oid = repo.commit(tree, message, {
  author,
  committer,
  parents,
  signature,
  updateRef: 'HEAD',
});
```

`commit()`은 서명을 첨부하기 전에 서명되지 않은 커밋 내용을 다시 만들어요. 두 메서드에 같은 트리, 메시지, 작성자, 커미터, 부모 커밋을 전달해야 해요. 위 예제처럼 작성자와 커미터의 `timeOptions`를 모두 고정하세요. 타임스탬프를 지정하지 않으면 호출할 때마다 현재 시간을 사용하므로 서로 다른 바이트가 만들어질 수 있어요. `commitCreateBuffer()`가 반환한 UTF-8 바이트를 그대로 서명하고, 공백이나 줄바꿈을 변경하지 마세요.

서명은 Git의 기본 `gpgsig` 필드에 저장돼요. 두 서명 커밋 메서드는 전달된 서명이 LF(`\n`)로 끝나면 마지막 LF 하나를 제거해요. GPG 출력의 마지막 줄바꿈 때문에 저장된 헤더에 빈 줄이 추가되는 것을 막기 위해서예요. 커밋 내용은 변경하지 않아요. libgit2는 서명을 암호학적으로 검증하지 않으므로, 서명을 저장하거나 추출하는 데 성공해도 유효한 서명이라는 뜻은 아니에요.

### 커밋 객체만 저장하기

서명한 내용을 직접 저장하려면 위 예제의 `commit()` 호출 대신 `commitSigned()`를 사용하세요.

```ts
const oid = repo.commitSigned(content, signature);

const signatureInfo = repo.extractSignature(oid);
console.log(signatureInfo?.signedData === content); // true
```

`commitSigned()`는 커밋을 객체 데이터베이스에 저장하며 `HEAD`, 다른 레퍼런스, reflog를 갱신하지 않아요. 기본 서명 필드는 `gpgsig`예요. 세 번째 인자로 다른 서명 필드를 지정할 수 있어요. 필드 이름은 비어 있으면 안 되고, 공백 문자나 NUL 문자를 포함할 수 없어요.
