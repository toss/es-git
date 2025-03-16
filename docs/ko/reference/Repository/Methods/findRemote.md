# findRemote

리포지토리에서 특정 리모트를 찾아요.

## 시그니처

```ts
class Repository {
  findRemote(name: string): Remote | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">조회할 리모트의 이름이에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Remote</span>
    <br>
    <p class="param-description">주어진 이름에 해당하는 리모트를 반환해요. 리모트가 존재하지 않으면 <code>null</code>을 반환해요.</p>
  </li>
</ul>