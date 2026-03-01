# getAttr

경로에 대한 git 속성 값을 가져와요.

## 시그니처

```ts
class Repository {
  getAttr(
    path: string,
    name: string,
    options?: AttrOptions | null | undefined
  ): boolean | string | Buffer | null;
```


### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">속성을 확인할 경로예요. 상대 경로는 리포지토리 루트를 기준으로 해석돼요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">조회할 속성 이름이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | AttrOptions</span>
    <br>
    <p class="param-description">속성 조회를 위한 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">checkFileThenIndex</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">작업 디렉터리를 확인한 다음 인덱스를 확인해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">checkIndexOnly</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">인덱스만 확인해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">checkIndexThenFile</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">인덱스를 확인한 다음 작업 디렉터리를 확인해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">checkNoSystem</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">시스템 gitattributes 파일을 사용하지 않아요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string | boolean | Buffer&lt;ArrayBufferLike&gt;</span>
    <br>
    <p class="param-description">속성 값의 출력</p>
  </li>
</ul>