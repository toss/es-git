# submoduleSetUrl

구성에서 하위 모듈의 URL을 설정해요.

이것을 호출한 후에는 `Submodule#sync()`를 호출해서 변경 사항을 체크아웃된 하위 모듈 리포지토리에 기록하고 싶을 수도 있어요.

## 시그니처

```ts
class Repository {
  submoduleSetUrl(name: string, url: string): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">구성할 하위 모듈의 이름</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">url</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">하위 모듈에 사용해야 하는 URL</p>
  </li>
</ul>