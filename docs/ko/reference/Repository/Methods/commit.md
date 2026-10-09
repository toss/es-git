# commit

리포지토리에 새로운 커밋을 생성해요.

`updateRef`가 `null`이 아니면 해당 레퍼런스가 이 커밋을 가리키도록 업데이트돼요.  
레퍼런스가 직접 레퍼런스가 아니면 직접 레퍼런스로 변환돼요.  
`"HEAD"`를 전달하면 현재 브랜치의 HEAD를 이 커밋으로 업데이트해요.  
레퍼런스가 존재하지 않으면 새로 생성되며, 존재하는 경우 첫 번째 부모 커밋은 해당 브랜치의 최신 커밋이어야 해요.

외부에서 서명하려면 `commitCreateBuffer`로 커밋 내용을 만들고, 두 메서드에 같은 트리,
메시지, 부모 커밋, 작성자, 커미터를 전달하세요. 작성자와 커미터의 `timeOptions`도 고정해야 해요.
이 메서드는 커밋 내용을 다시 만들며, `signature`가 그 내용과 일치하는지 검증하지 않아요.
타임스탬프나 내용이 달라지면 서명이 유효하지 않게 돼요.

## 시그니처

```ts
class Repository {
  commit(tree: Tree, message: string, options?: CommitOptions | null | undefined): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">tree</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Tree</span>
    <br>
    <p class="param-description">커밋할 트리예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">message</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">전체 커밋 메시지예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CommitOptions</span>
    <br>
    <p class="param-description">커밋을 생성할 때 사용할 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">author</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">
          작성자 정보(이름, 이메일, 시간)예요. 설정하지 않으면 리포지토리의 기본 서명을 사용해요.
          기본 서명이 없으면 오류가 발생해요.
        </p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">작성자의 이메일 주소예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">작성자의 이름이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
            <br>
            <p class="param-description">시간 설정 옵션이에요.</p>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">offset</span><span class="param-type">number</span>
                <br>
                <p class="param-description">시간대 오프셋(분 단위)이에요.</p>
              </li>
              <li class="param-li">
                <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
                <br>
                <p class="param-description">Unix epoch(초 단위) 기준의 시간이에요.</p>
              </li>
            </ul>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">committer</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">
          커미터 정보(이름, 이메일, 시간)예요. 설정하지 않으면 리포지토리의 기본 서명을 사용해요.
          기본 서명이 없으면 오류가 발생해요.
        </p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">커밋 작성자의 이메일 주소예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">커밋 작성자의 이름이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
            <br>
            <p class="param-description">시간 설정 옵션이에요.</p>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">offset</span><span class="param-type">number</span>
                <br>
                <p class="param-description">시간대 오프셋(분 단위)이에요.</p>
              </li>
              <li class="param-li">
                <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
                <br>
                <p class="param-description">Unix epoch(초 단위) 기준의 시간이에요.</p>
              </li>
            </ul>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">parents</span><span class="param-type">string[]</span>
        <br>
        <p class="param-description">부모 커밋 ID 목록이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">updateRef</span><span class="param-type">string</span>
        <br>
        <p class="param-description">
          이 커밋을 가리키도록 업데이트할 레퍼런스 이름이에요.  
          `"HEAD"`를 전달하면 현재 브랜치의 HEAD를 업데이트해요.
        </p>
      </li>
      <li class="param-li">
        <span class="param-name">signature</span><span class="param-type">string</span>
        <br>
        <p class="param-description">정확한 UTF-8 커밋 내용에 대한 ASCII-armored 형식의 서명이에요. 서명은 검증하지 않아요. <code>commitCreateBuffer</code>로 내용을 만들고, 두 메서드에 같은 작성자와 커미터 정보를 고정된 <code>timeOptions</code>와 함께 전달하세요. 서명을 저장하기 전에 마지막 줄바꿈 하나를 제거해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">signatureField</span><span class="param-type">string</span>
        <br>
        <p class="param-description">서명을 저장할 헤더 필드 이름이에요. 비어 있으면 안 되고, 공백 문자나 NUL 바이트를 포함할 수 없어요. 제공하지 않으면 기본 서명 필드(gpgsig)가 사용돼요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">
      생성된 커밋의 SHA-1 ID를 반환해요.
    </p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">명시적으로 지정한 작성자나 커미터 정보가 유효하지 않으면 오류가 발생해요. 예를 들어 이름이나 이메일이 비어 있거나 <code>&lt;</code>, <code>&gt;</code>, NUL 바이트를 포함하는 경우예요. 생략한 정보에 사용할 리포지토리 기본 서명이 없거나, 부모 커밋이 존재하지 않거나, <code>updateRef</code>를 업데이트할 수 없는 경우에도 오류가 발생해요. 서명된 커밋에서는 <code>signatureField</code>가 비어 있거나 공백 문자 또는 NUL 바이트를 포함해도 오류가 발생해요. 유효하지 않은 명시적 작성자나 커미터 정보를 리포지토리 기본값으로 대체하지 않아요.</p>
  </li>
</ul>
