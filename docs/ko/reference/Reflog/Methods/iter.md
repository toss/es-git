# iter

reflog 항목에 대한 이터레이터를 만들어요.

## 시그니처

```ts
class Reflog {
  iter(): ReflogIter;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">ReflogIter</span>
    <br>
    <p class="param-description">reflog 항목에 대한 이터레이터</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">reflog에 접근할 수 없으면 오류를 던져요.</p>
  </li>
</ul>