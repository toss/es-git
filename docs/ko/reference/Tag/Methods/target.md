# target

태그가 가리키는 Git 개체를 가져와요.

이 메서드는 주어진 개체를 리포지토리에서 조회하고 반환해요.

## 시그니처

```ts
class Tag {
  target(): GitObject;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">GitObject</span>
    <br>
    <p class="param-description">태그가 가리키는 Git 개체예요.</p>
  </li>
</ul>