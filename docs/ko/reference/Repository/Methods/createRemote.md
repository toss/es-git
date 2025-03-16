# createRemote

리포지토리 설정에 기본 fetch refspec을 가진 리모트를 추가해요.

## 시그니처

```ts
class Repository {
  createRemote(name: string, url: string, options?: CreateRemoteOptions): Remote;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">리모트의 이름이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">url</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">리모트의 URL이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CreateRemoteOptions</span>
    <br>
    <p class="param-description">리모트 생성 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">fetchRefspec</span><span class="param-type">string</span>
        <br>
        <p class="param-description">리모트에서 데이터를 가져올(fetch) refspec을 설정해요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Remote</span>
    <br>
    <p class="param-description">생성된 리모트를 반환해요.</p>
  </li>
</ul>