# amend

입력한 값을 사용해서 이 기존 커밋을 수정해요

이 메서드는 기존 커밋과 완전히 동일하지만 값들이 업데이트된 새 커밋을 만들어요. 새 커밋은 이전 커밋과 동일한 부모를 가져요.

## 시그니처

```ts
class Commit {
  amend(options?: AmendOptions, tree?: Tree): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | AmendOptions</span>
    <br>
    <p class="param-description">커밋을 수정하기 위한 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">author</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">작성자 서명이에요.</p>
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
                <p class="param-description">에포크부터의 시간(초 단위)이에요.</p>
              </li>
            </ul>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">committer</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">커미터 서명이에요.</p>
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
                <p class="param-description">에포크부터의 시간(초 단위)이에요.</p>
              </li>
            </ul>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">message</span><span class="param-type">string</span>
        <br>
        <p class="param-description">이 커밋의 전체 메시지에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">messageEncoding</span><span class="param-type">string</span>
        <br>
        <p class="param-description">커밋 메시지에 사용할 인코딩이에요. 표준 인코딩 이름으로 표현해요. 예: &quot;UTF-8&quot;이에요. NULL이면 인코딩 헤더를 쓰지 않고 UTF-8로 간주해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">updateRef</span><span class="param-type">string</span>
        <br>
        <p class="param-description">NULL이 아니면 이 커밋을 가리키도록 업데이트할 참조 이름이에요. 참조가 직접 참조가 아니면 직접 참조로 해석돼요. 현재 브랜치의 HEAD를 이 커밋을 가리키도록 업데이트하려면 &quot;HEAD&quot;를 사용해요. 참조가 아직 없으면 새로 만들어지고, 이미 있으면 이 브랜치의 최신 커밋이 첫 번째 부모여야 해요.</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">tree</span><span class="param-type">null | Tree</span>
    <br>
    <p class="param-description">커밋을 수정할 때 사용할 트리예요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">수정된 커밋의 ID(SHA1)예요.</p>
  </li>
</ul>