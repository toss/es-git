# iterByLine

파일 라인을 반복자로 스캔하여 blame 청크를 수집해요

## 시그니처

```ts
class Blame {
  iterByLine(): Generator<BlameHunk>;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">BlameHunksByLine</span>
    <br>
    <p class="param-description">라인 스캔으로 수집된 blame 청크의 반복자</p>
  </li>
</ul>

## 예제

```ts
// for...of 루프 사용
for (const hunk of blame.iterByLine()) {
  console.log(hunk.finalCommitId);
}

// 스프레드 연산자를 사용하여 모든 청크 수집
const hunks = [...blame.iterByLine()];
```