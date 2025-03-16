# getRemote

리포지토리에서 특정 리모트를 가져와요.

## 시그니처

```ts
class Repository {
  getRemote(name: string): Remote;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">가져올 리모트의 이름이에요.</p>P
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Remote</span>
    <br>
    <p class="param-description">주어진 이름에 해당하는 리모트를 반한해요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">리모트가 존재하지 않으면 오류를 발생시켜요.</p>
  </li>
</ul>