# commitCreateBuffer

외부에서 서명할 커밋 내용을 만들어요.

서명되지 않은 커밋 객체 내용을 만들지만 객체 데이터베이스에는 저장하지 않아요.
이 메서드가 반환한 문자열의 UTF-8 바이트를 그대로 서명한 다음, 커밋 내용과 서명을
`commitSigned()`에 전달하세요. 서명한 뒤 공백이나 줄바꿈을 변경하면 서명이 유효하지 않게 돼요.

## 시그니처

```ts
class Repository {
  commitCreateBuffer(
    tree: Tree,
    message: string,
    options?: CommitCreateBufferOptions | null | undefined,
  ): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">tree</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Tree</span>
    <br>
    <p class="param-description">커밋 내용을 만들 트리예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">message</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">커밋 메시지예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">CommitCreateBufferOptions | null</span>
    <br>
    <p class="param-description">커밋 내용 생성 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">author</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">작성자 서명이에요. 지정하지 않으면 리포지토리의 기본 서명을 사용해요. 기본 서명이 없으면 오류가 발생해요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에 사용할 이메일 주소예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에 사용할 이름이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
            <br>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">offset</span><span class="param-type">number</span>
                <br>
                <p class="param-description">분 단위 시간대 오프셋이에요.</p>
              </li>
              <li class="param-li">
                <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
                <br>
                <p class="param-description">Unix epoch 기준 초 단위 시간이에요.</p>
              </li>
            </ul>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">committer</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">커미터 서명이에요. 지정하지 않으면 리포지토리의 기본 서명을 사용해요. 기본 서명이 없으면 오류가 발생해요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에 사용할 이메일 주소예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에 사용할 이름이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
            <br>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">offset</span><span class="param-type">number</span>
                <br>
                <p class="param-description">분 단위 시간대 오프셋이에요.</p>
              </li>
              <li class="param-li">
                <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
                <br>
                <p class="param-description">Unix epoch 기준 초 단위 시간이에요.</p>
              </li>
            </ul>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">parents</span><span class="param-type">string[]</span>
        <br>
        <p class="param-description">의도한 순서대로 나열한 부모 커밋 ID예요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">외부에서 서명할 커밋 내용이에요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">서명을 확인할 수 없거나 부모 커밋이 존재하지 않을 때 발생해요.</p>
  </li>
</ul>

## 예제

```ts
const content = repo.commitCreateBuffer(tree, 'signed commit', {
  parents: [repo.head().target()!],
});
const signature = await signingBackend.sign(Buffer.from(content, 'utf8'));
const oid = repo.commitSigned(content, signature);
```
