# name

레퍼런스의 전체 이름을 가져와요.

## 시그니처

```ts
class Reference {
  name(): string;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">레퍼런스의 전체 이름이에요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">이름이 유효한 UTF-8 문자열이 아니면 오류를 발생시켜요.</p>
  </li>
</ul>