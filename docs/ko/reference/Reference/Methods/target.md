# target

직접 레퍼런스가 가리키는 OID(Object ID)를 가져와요.

이 메서드는 직접 레퍼런스(개체 ID를 가리키는 레퍼런스)에서만 사용할 수 있어요.  
심볼릭 레퍼런스에서는 사용할 수 없어요.


## 시그니처

```ts
class Reference {
  target(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">직접 레퍼런스가 가리키는 OID를 반환해요.</p>
  </li>
</ul>