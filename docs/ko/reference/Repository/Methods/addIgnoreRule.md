# addIgnoreRule

리포지토리에 무시 규칙을 추가해요.

이것은 리포지토리에 무시 규칙을 추가해요. 이 규칙들은
기존의 무시 규칙들(.gitignore 파일 등)과 함께 사용돼요.

## 시그니처

```ts
class Repository {
  addIgnoreRule(rules: string): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">rules</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">추가할 규칙들, 줄바꿈으로 구분</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
repo.addIgnoreRule("node_modules/");
```