# isPathIgnored

무시 규칙이 주어진 경로에 적용되는지 테스트해요.

이 함수는 무시 규칙을 확인하여 주어진 파일에 적용되는지 확인해요.
이는 파일이 이미 인덱스에 있거나 리포지토리에 커밋되었는지 여부와 관계없이 파일이 무시될지를 나타내요.

## 시그니처

```ts
class Repository {
  isPathIgnored(path: string): boolean;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">확인할 경로</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">- 경로가 무시되면 true, 그렇지 않으면 false</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const isIgnored = repo.isPathIgnored("node_modules/some-package");
console.log(`Path is ${isIgnored ? "ignored" : "not ignored"}`);
```