# index

이 리포지토리의 인덱스(Index) 파일을 가져와요.

커스텀 인덱스가 설정되지 않았다면, 기본 인덱스(`.git/index`)가 반환돼요.

## 시그니처

```ts
class Repository {
  index(): Index;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Index</span>
    <br>
    <p class="param-description">리포지토리의 인덱스 파일을 반환해요.</p>
  </li>
</ul>