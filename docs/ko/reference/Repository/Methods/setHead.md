# setHead

리포지토리 `HEAD`가 지정된 참조를 가리키도록 만들어요.

제공된 참조가 트리나 blob를 가리키는 경우, `HEAD`는 변경되지 않고 오류가 반환돼요.

제공된 참조가 브랜치를 가리키는 경우, `HEAD`는 해당 브랜치를 가리키며 연결된 상태를 유지하거나, 아직 연결되지 않았다면 연결돼요. 브랜치가 아직 존재하지 않더라도 오류는 반환되지 않아요. 그러면 `HEAD`는 아직 생성되지 않은 브랜치에 연결돼요.

그렇지 않으면, `HEAD`는 분리되어 커밋을 직접 가리켜요.

## 시그니처

```ts
class Repository {
  setHead(refname: string): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">refname</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description"><code>HEAD</code>가 가리킬 지정된 참조</p>
  </li>
</ul>