# setHead

리포지토리의 `HEAD`를 지정된 레퍼런스로 변경해요.

- 지정된 레퍼런스가 트리(tree)나 블롭(blob)을 가리키면 `HEAD`는 변경되지 않고, 에러가 발생해요.
- 지정된 레퍼런스가 브랜치를 가리키면 `HEAD`는 해당 브랜치를 가리키게 돼요.
    - 이미 브랜치에 연결된 상태라면 그대로 유지돼요.
    - 기존에 연결되지 않은 상태였다면 연결돼요.
    - 브랜치가 아직 존재하지 않아도 에러가 발생하지 않으며, `HEAD`는 생성되지 않은 브랜치를 가리키게 돼요.
- 위 조건에 해당하지 않으면 `HEAD`는 분리 상태가 되어 특정 커밋을 직접 가리켜요.

## 시그니처

```ts
class Repository {
  setHead(refname: string): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">refname</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description"><code>HEAD</code>가 가리킬 레퍼런스를 지정해요.</p>
  </li>
</ul>