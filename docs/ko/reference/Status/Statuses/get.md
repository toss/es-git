# get

지정된 인덱스에서 이 목록의 상태 항목을 가져와요.

## 시그니처

```ts
class Statuses {
  get(index: number): StatusEntry | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">가져올 상태 항목의 인덱스.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">StatusEntry | null</span>
    <br>
    <p class="param-description">지정된 인덱스에 있는 이 목록의 상태 항목. 상태 항목이 존재하지 않으면 <code>null</code>을 반환해요.</p>
  </li>
</ul>