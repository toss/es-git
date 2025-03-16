# peel

태그가 아닌 Git 개체가 나올 때까지 태그를 재귀적으로 벗겨내요.

## 시그니처

```ts
class Tag {
  peel(): GitObject;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">GitObject</span>
    <br>
    <p class="param-description">이 태그의 Git 개체를 반환해요.</p>
  </li>
</ul>
