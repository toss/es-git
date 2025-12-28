# reflog

이름으로 reflog를 조회해요.

## 시그니처

```ts
class Repository {
  reflog(name: string): Reflog;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">조회할 reflog의 대상이 되는 참조 이름(예: &quot;HEAD&quot;, &quot;refs/heads/main&quot;)</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Reflog</span>
    <br>
    <p class="param-description">주어진 참조 이름에 대한 Reflog 인스턴스</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">reflog가 존재하지 않거나 열 수 없으면 오류를 던져요.</p>
  </li>
</ul>