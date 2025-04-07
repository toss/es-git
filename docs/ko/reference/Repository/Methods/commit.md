# commit

리포지토리에 새로운 커밋을 생성해요.

`updateRef`가 `null`이 아니면 해당 레퍼런스가 이 커밋을 가리키도록 업데이트돼요.  
레퍼런스가 직접 레퍼런스가 아니면 직접 레퍼런스로 변환돼요.  
`"HEAD"`를 전달하면 현재 브랜치의 HEAD를 이 커밋으로 업데이트해요.  
레퍼런스가 존재하지 않으면 새로 생성되며, 존재하는 경우 첫 번째 부모 커밋은 해당 브랜치의 최신 커밋이어야 해요.

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
    <p class="param-description">커밋 메시지예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CommitOptions</span>
    <br>
    <p class="param-description">커밋 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">author</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">
          작성자 서명이에요. 설정하지 않으면 리포지토리의 기본 서명을 사용해요.  
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
          커밋 작성자 서명이에요. 설정하지 않으면 리포지토리의 기본 서명을 사용해요.  
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
