# getHunkByIndex

지정된 인덱스에 대한 blame 정보를 가져와요.

## 시그니처

```ts
class Blame {
  getHunkByIndex(index: number): BlameHunk;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">가져올 hunk의 인덱스 (0부터 시작)</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">BlameHunk</span>
    <br>
    <p class="param-description">지정된 인덱스에 대한 blame 정보</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">finalCommitId</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">이 줄이 마지막으로 변경된 커밋의 oid.</p>
      </li>
      <li class="param-li">
        <span class="param-name">finalSignature</span><span class="param-type">Signature</span>
        <br>
        <p class="param-description">이 줄이 마지막으로 변경된 커밋의 서명.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명의 이메일.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명의 이름.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
            <br>
            <p class="param-description">epoch부터의 시간(초)</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">finalStartLineNumber</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">이 hunk가 시작되는 최종 파일의 1부터 시작하는 줄 번호.</p>
      </li>
      <li class="param-li">
        <span class="param-name">isBoundary</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
        <p class="param-description">hunk가 경계 커밋(파일이 리포지토리에 처음 도입된 커밋)으로 확인된 경우 true.</p>
      </li>
      <li class="param-li">
        <span class="param-name">linesInHunk</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">이 hunk의 줄 수.</p>
      </li>
      <li class="param-li">
        <span class="param-name">origCommitId</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">이 줄이 처음 작성된 커밋의 oid.</p>
      </li>
      <li class="param-li">
        <span class="param-name">origSignature</span><span class="param-type">Signature</span>
        <br>
        <p class="param-description">이 줄이 처음 작성된 커밋의 서명.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명의 이메일.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명의 이름.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
            <br>
            <p class="param-description">epoch부터의 시간(초)</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">origStartLineNumber</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">이 hunk가 시작되는 원본 파일의 1부터 시작하는 줄 번호.</p>
      </li>
      <li class="param-li">
        <span class="param-name">path</span><span class="param-type">string</span>
        <br>
        <p class="param-description">이 줄이 처음 작성된 파일의 경로.</p>
      </li>
    </ul>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">인덱스에서 hunk를 찾을 수 없는 경우</p>
  </li>
</ul>