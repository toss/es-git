# blameFile

지정된 경로의 파일에 대한 blame 개체를 생성해요

## 시그니처

```ts
class Repository {
  blameFile(path: string, options?: BlameOptions): Blame;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">blame할 파일의 경로</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | BlameOptions</span>
    <br>
    <p class="param-description">blame 동작을 제어하는 옵션</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">firstParent</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">첫 번째 부모만 따라가며 도달 가능한 커밋으로 검색을 제한해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">공백 차이를 무시해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">maxLine</span><span class="param-type">number</span>
        <br>
        <p class="param-description">blame할 최대 줄 번호 (1부터 시작하는 인덱스)</p>
      </li>
      <li class="param-li">
        <span class="param-name">minLine</span><span class="param-type">number</span>
        <br>
        <p class="param-description">blame할 최소 줄 번호 (1부터 시작하는 인덱스)</p>
      </li>
      <li class="param-li">
        <span class="param-name">newestCommit</span><span class="param-type">string</span>
        <br>
        <p class="param-description">고려할 가장 최신 커밋의 oid. blame 알고리즘은 이 커밋에 도달하면 중단해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">oldestCommit</span><span class="param-type">string</span>
        <br>
        <p class="param-description">고려할 가장 오래된 커밋의 oid. blame 알고리즘은 이 커밋에 도달하면 중단해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">path</span><span class="param-type">string</span>
        <br>
        <p class="param-description">작업 중인 파일의 경로. 경로는 리포지토리 루트에 상대적이어야 해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">trackCopiesAnyCommitCopies</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">어떤 커밋에든 존재하는 다른 파일에서 복사된 줄을 추적해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">trackCopiesSameCommitCopies</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">같은 커밋에 존재하는 다른 파일에서 복사된 줄을 추적해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">trackCopiesSameCommitMoves</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">같은 커밋에서 파일 간에 이동한 줄을 추적해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">trackLinesMovement</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">파일 내에서 이동한 줄을 추적해요. 이것은 git-blame -M 옵션이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">useMailmap</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">mailmap 파일을 사용하여 작성자와 커미터의 이름과 이메일 주소를 정식 실명과 이메일 주소로 매핑해요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Blame</span>
    <br>
    <p class="param-description">지정된 파일에 대한 Blame 개체</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">파일이 존재하지 않거나 열 수 없는 경우</p>
  </li>
</ul>

## 예제

```ts
// 전체 파일을 blame
const blame = repo.blameFile('path/to/file.js');

// 한 줄만 blame
const lineBlame = repo.blameFile('path/to/file.js', { minLine: 10, maxLine: 10 });

// 줄 범위를 blame
const rangeBlame = repo.blameFile('path/to/file.js', { minLine: 5, maxLine: 15 });
```