# iter

모든 blame 헝크를 이터레이터로 가져와요

## 시그니처

```ts
class Blame {
  iter(): Generator<BlameHunk>;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">BlameHunks</span>
    <br>
    <p class="param-description">모든 blame 헝크의 이터레이터</p>
  </li>
</ul>

## 예제

```ts
// for...of 루프 사용
for (const hunk of blame.iter()) {
  console.log(hunk.finalCommitId);
}

// 스프레드 연산자를 사용하여 모든 헝크 수집
const hunks = [...blame.iter()];
```