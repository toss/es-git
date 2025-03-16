# createAnnotationTag

리포지토리에서 개체를 기반으로 주석 태그(annotation tag)를 생성해요.

이 메서드는 레퍼런스를 생성하지 않고 태그 개체만 만들어요.  
태그 이름은 유효성을 검사하며, 아래 문자는 사용할 수 없어요.
- `~`, `^`, `:`, `\`, `?`, `[`, `*`
- `".."`, `"@{"` (특수한 의미를 가짐)

## 시그니처

```ts
class Repository {
  createAnnotationTag(
    name: string,
    target: GitObject,
    message: string,
    options?: CreateAnnotationTagOptions,
  ): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">태그 이름이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">target</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">GitObject</span>
    <br>
    <p class="param-description">이 태그가 가리킬 Git 개체예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">message</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">태그 메시지예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CreateAnnotationTagOptions</span>
    <br>
    <p class="param-description">태그 생성 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">tagger</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">
          태그 작성자의 서명이에요.  
          설정하지 않으면 리포지토리의 기본 서명을 사용해요.  
          기본 서명이 없으면 오류가 발생해요.
        </p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">태그 작성자의 이메일 주소예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">태그 작성자의 이름이에요.</p>
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
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">생성된 태그의 OID(SHA-1)를 반환해요.</p>
  </li>
</ul>