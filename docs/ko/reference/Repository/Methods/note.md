# note

개체에 대한 노트를 추가해요

`notesRef` 인수는 사용할 참조의 정규 이름이에요.
지정하지 않으면 기본값은 "refs/notes/commits"예요. `force`가 지정되면
이전에 존재하던 노트가 덮어써져요.

## 시그니처

```ts
class Repository {
  note(oid: string, note: string, options?: CreateNoteOptions | null | undefined): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">장식할 Git 개체의 OID 값이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">note</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">해당 개체 OID에 추가할 노트의 내용이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CreateNoteOptions</span>
    <br>
    <p class="param-description">노트를 생성하기 위한 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">author</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">노트 커밋 작성자의 서명이에요. 제공되지 않으면 리포지토리의 기본 서명을 사용해요. 리포지토리에 기본 서명이 설정되어 있지 않으면 오류가 발생해요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에 사용되는 이메일이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에 사용되는 이름이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
            <br>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">offset</span><span class="param-type">number</span>
                <br>
                <p class="param-description">시간대 오프셋(분 단위)이에요.</p>
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
        <p class="param-description">노트 커밋 커미터의 서명이에요. 제공되지 않으면 리포지토리의 기본 서명을 사용해요. 리포지토리에 기본 서명이 설정되어 있지 않으면 오류가 발생해요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에 사용되는 이메일이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에 사용되는 이름이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
            <br>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">offset</span><span class="param-type">number</span>
                <br>
                <p class="param-description">시간대 오프셋(분 단위)이에요.</p>
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
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">기존 노트를 덮어쓸지 여부예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">notesRef</span><span class="param-type">string</span>
        <br>
        <p class="param-description">사용할 참조의 정규 이름이에요. 기본값은 &quot;refs/notes/commits&quot;예요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">노트의 OID 값이에요.</p>
  </li>
</ul>