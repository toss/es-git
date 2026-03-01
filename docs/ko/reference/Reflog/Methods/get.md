# get

인덱스로 reflog 항목을 가져와요.

## 시그니처

```ts
class Reflog {
  get(i: number): ReflogEntry | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">i</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">가져올 항목의 인덱스</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | ReflogEntry</span>
    <br>
    <p class="param-description">지정한 인덱스의 reflog 항목이에요. 인덱스가 범위를 벗어나면 <code>null</code>을 반환해요.</p>
  </li>
</ul>