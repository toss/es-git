# name

주어진 로컬 또는 원격 브랜치의 이름을 반환해요.

## 시그니처

```ts
class Branch {
  name(): string;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">주어진 로컬 또는 원격 브랜치의 이름이에요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">이름이 유효한 utf-8이 아니라면 오류가 발생해요.</p>
  </li>
</ul>