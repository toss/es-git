# delete

레퍼런스를 삭제해요.

이 메서드는 직접 레퍼런스(direct reference)와 심볼릭 레퍼런스(symbolic reference) 모두에서 작동하며,  
레퍼런스는 즉시 디스크에서 제거돼요.

## 시그니처

```ts
class Reference {
  delete(): void;
}
```

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">레퍼런스를 조회한 이후 변경되었으면 이 메서드는 오류를 발생시켜요.</p>
  </li>
</ul>