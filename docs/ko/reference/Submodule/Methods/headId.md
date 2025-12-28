# headId

현재 `HEAD` 트리에서 서브모듈의 OID를 가져와요.

## 시그니처

```ts
class Submodule {
  headId(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">현재 <code>HEAD</code> 트리에서 서브모듈의 OID</p>
  </li>
</ul>