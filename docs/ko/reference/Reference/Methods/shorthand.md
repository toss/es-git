# shorthand

레퍼런스의 축약된 이름을 가져와요.

이 메서드는 레퍼런스 이름을 사람이 읽기 쉬운 형태로 변환해요.  
적절한 축약형이 없으면 전체 이름을 반환해요.

## 시그니처

```ts
class Reference {
  shorthand(): string;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">레퍼런스의 전체 축약형을 반환해요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">유효한 UTF-8 문자열이 아니면 오류를 발생시켜요.</p>
  </li>
</ul>