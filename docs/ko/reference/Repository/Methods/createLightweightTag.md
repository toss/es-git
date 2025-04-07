# createLightweightTag

대상 개체를 가리키는 새로운 경량 태그(lightweight tag)를 생성해요.

이 메서드는 대상 개체를 가리키는 직접 레퍼런스를 생성해요.

## 시그니처

```ts
class Repository {
  createLightweightTag(
    name: string,
    target: GitObject,
    options?: CreateLightweightTagOptions | null | undefined,
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
    <span class="param-name">options</span><span class="param-type">null | CreateLightweightTagOptions</span>
    <br>
    <p class="param-description">태그 생성 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">
          <code>force</code>가 <code>true</code>이면,  
          동일한 이름의 레퍼런스가 이미 존재하더라도 덮어써요.
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
    <p class="param-description">생성된 태그의 OID(SHA-1)를 반환해요.</p>
  </li>
</ul>
