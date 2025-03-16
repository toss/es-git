# type

레퍼런스의 타입을 가져와요.

## 시그니처

```ts
class Reference {
  type(): ReferenceType | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | ReferenceType</span>
    <br>
    <p class="param-description">
      타입을 알 수 없으면 <code>null</code>을 반환해요.
    </p>
    <p class="param-description">
      - <code>Direct</code> : 개체 ID를 가리키는 직접 레퍼런스예요.<br>
      - <code>Symbolic</code> : 다른 레퍼런스를 가리키는 심볼릭 레퍼런스예요.
    </p>
  </li>
</ul>