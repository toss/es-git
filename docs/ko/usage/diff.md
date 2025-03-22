# 비교

es-git은 변경 사항 비교하는 다양한 메소드를 지원해요. 각 메소드 문서에서 자세한 사용법을 확인할 수 있어요.

- [`diffIndexToIndex()`](../reference/Repository/Methods/diffIndexToIndex.md) : 두 개의 인덱스 간의 차이를 비교해요.
- [`diffIndexToWorkdir()`](../reference/Repository/Methods/diffIndexToWorkdir.md) : 인덱스와 작업 디렉터리 간의 차이를 비교해요. (`git diff` 명령어와 동일한 동작)
- [`diffTreeToTree()`](../reference/Repository/Methods/diffTreeToTree.md) : 두 트리간의 차이를 비교해요. (`git diff <old-tree> <new-tree>` 명령어와 동일한 동작)
- [`diffTreeToWorkdir()`](../reference/Repository/Methods/diffTreeToWorkdir.md) : 트리와 작업 디렉터리 간의 차이를 비교해요.
- [`diffTreeToWorkdirWithIndex()`](../reference/Repository/Methods/diffTreeToWorkdirWithIndex.md) : 트리와 작업 디렉터리 간의 차이를 비교하면서 인덱스 데이터를 반영해요.

---

아래 예제는 [`diffTreeToWorkdirWithIndex()`](../reference/Repository/Methods/diffTreeToWorkdirWithIndex.md) 메소드를 사용해 `HEAD` 트리와 인덱스 간의 diff와 인덱스와 작업 디렉터리 간의 diff를 합쳐서 비교해요.

```ts
import { openRepository } from 'es-git';
import fs from 'node:fs/promises';

const repo = await openRepository('.');
await fs.writeFile('newFile', 'data', 'utf8');

const index = repo.index();
index.addPath('newFile');
index.write();

const headTree = repo.head().peelToTree();
const diff = repo.diffTreeToWorkdirWithIndex(headTree);
const deltas = [...diff.deltas()];

console.assert(deltas[0].status()); // "Added"
console.assert(deltas[0].newFile().path()); // "newfile"
```

## 리모트 브랜치와 비교하기

리모트 브랜치를 [`fetch()`](../reference/Remote/Methods/fetch.md) 메소드로 불러와 리모트 브랜치와 로컬 브랜치 간의 트리를 비교할 수도 있어요.

이는 Pull Request를 병합할 때 변경된 부분만 추적해 CI를 수행해 최적화하는데 도움이 될 수 있어요. 예를들어, GitHub Actions의 경우, Pull Request 이벤트로 트리거된 워크플로에서 [
`github.base_ref`](https://docs.github.com/ko/actions/writing-workflows/choosing-what-your-workflow-does/accessing-contextual-information-about-workflow-runs#github-context) 값을 참조해서 비교할 리모트 브랜치의 레페런스를
가져올 수 있어요.

아래 예제는 리모트의 `main` 브랜치와 로컬의 `HEAD` 레퍼런스 간의 트리를 비교해 변경 사항을 가져와요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const remote = repo.getRemote('origin');
await remote.fetch(['main'], {
  fetch: {
    depth: 1,
  },
});

const baseTree = repo.getReference('refs/remotes/origin/main').peelToTree();
const headTree = repo.head().peelToTree();

// origin 리모트의 "main" 브랜치와 로컬 "HEAD"가 가리키는 트리 간의 차이를 비교해요.
const diff = repo.diffTreeToTree(baseTree, headTree);

for (const delta of diff.deltas()) {
  // ...
}
```

## 포맷된 텍스트 출력하기

`git diff` 명령어로 텍스트로 출력된 결과물을 볼 수 있듯이, [`print()`](../reference/Diff/Methods/print.md) 메소드를 사용해 변경 사항을 텍스트 결과물로 출력할 수 있어요.

Delta를 일일이 순회하지 않고 변경 사항을 한 눈에 파악해 디버깅하는데 도움될 수 있어요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const diff = repo.diffIndexToWorkdir();

console.log(diff.print());
/**
 * diff --git a/first b/first
 * index 9c59e24..46d3a3e 100644
 * --- a/first
 * +++ b/first
 * @@ -1 +1,2 @@
 * first
 * first modified
 *
 * diff --git a/second b/second
 * deleted file mode 100644
 * index e019be0..0000000
 * --- a/second
 * +++ /dev/null
 * @@ -1 +0,0 @@
 * second
 */
```

## 이름 변경 추적하기

git은 명시적으로 파일 이름의 변경이나 파일의 이동을 명시적으로 관리하지 않기 때문에 이름이 변경된 파일을 추적하기 위해서 `renames` 옵션 설정이 필요해요.

[`findSimilar()`](../reference/Diff/Methods/findSimilar.md) 메소드를 사용하면 변경 사항을 추적할 때 여러가지 옵션을 설정할 수 있어요.

아래 예제에서 `renames` 옵션을 사용하면, 변경 사항을 비교할 때 이름이 변경된 파일이 `"Renamed"` 상태가 되는 것을 확인할 수 있어요.  

```ts
import { openRepository } from 'es-git';
import fs from 'node:fs/promises';

const repo = await openRepository('.');
const headTree = repo.head().peelToTree();

await fs.rename('myfile', 'myfile-renamed');
const index = repo.index();
index.addPath('myfile-renamed');
index.write();

const diff = repo.diffTreeToWorkdirWithIndex(headTree);
diff.findSimilar({ renames: true });

for (const delta of diff.deltas()) {
  if (detla.status() === 'Renamed') {
    const status = delta.status().toUpperCase();
    const oldPath = detla.oldFile().path();
    const newPath = detla.newFile().path();
    console.log(`[${status}] ${oldPath} -> ${newPath}`);
  }
}

/**
 * 위 예제 코드는 아래와 같이 출력해요.
 * [RENAMED] myfile -> myfile-renamed
 */
```
