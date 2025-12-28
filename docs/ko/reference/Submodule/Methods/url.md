# url

서브모듈의 URL을 가져와요.

## 시그니처

```ts
class Submodule {
  url(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">서브모듈의 URL. URL이 없으면 <code>null</code>을 반환해요.</p>
  </li>
</ul>