# resolve

심볼릭 레퍼런스를 직접 레퍼런스로 변환해요.

이 메서드는 심볼릭 레퍼런스를 반복적으로 펼쳐(OID가 가리키는 대상을 따라감)  
직접 레퍼런스로 변환할 때까지 실행돼요.

만약 직접 레퍼런스를 인자로 전달하면, 해당 레퍼런스의 복사본을 반환해요.

## 시그니처

```ts
class Reference {
  resolve(): Reference;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Reference</span>
    <br>
    <p class="param-description">변환된 직접 레퍼런스를 반환해요.</p>
  </li>
</ul>