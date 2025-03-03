# 커밋 히스토리 조회하기

커밋 히스토리를 조회하기 위해 [`Repository#revwalk`](../api/classes/Repository.md#revwalk) 메소드로 [
`Revwalk`](../api/classes/Revwalk.md)를 생성할 수 있어요.
[`Revwalk`](../api/classes/Revwalk.md)
는 [순회 프로토콜](https://developer.mozilla.org/ko/docs/Web/JavaScript/Reference/Iteration_protocols)을 구현하고 있기 때문에 [
`for...of`](https://developer.mozilla.org/ko/docs/Web/JavaScript/Reference/Statements/for...of) 명령문을 사용해 쉽게 커밋 히스토리를 순회할
수 있어요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
```
