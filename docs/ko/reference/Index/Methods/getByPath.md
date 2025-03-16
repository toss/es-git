# getByPath

경로를 기준으로 인덱스 내의 항목 중 하나를 가져와요.

## 시그니처

```ts
class Index {
  getByPath(path: string, stage?: IndexStage): IndexEntry | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">가져올 항목의 경로를 지정해요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">stage</span><span class="param-type">null | IndexStage</span>
    <br>
    <p class="param-description">가져올 인덱스 항목의 스테이지를 설정해요.</p>
    <p class="param-description">- <code>Any</code> : 모든 인덱스 스테이지와 일치.<br>- <code>Normal</code> : 인덱스에 정상적으로 스테이징된 파일.<br>- <code>Ancestor</code> : 충돌 시 조상 측 파일.<br>- <code>Ours</code> : 충돌 시 &quot;우리&quot; 측 파일.<br>- <code>Theirs</code> : 충돌 시 &quot;상대&quot; 측 파일.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | IndexEntry</span>
    <br>
    <p class="param-description">Index entry for the path.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">ctime</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Date</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">dev</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">fileSize</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">flags</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">flagsExtended</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">gid</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">id</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">ino</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">mode</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">mtime</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Date</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Buffer</span>
        <br>
        <p class="param-description">이 인덱스 항목의 경로를 바이트 벡터로 나타냅니다. 현재 운영 체제와 관계없이 디렉터리 구분자는 ASCII 슬래시(<code>0x2F</code>)를 사용합니다. 문자열 끝을 나타내는 NUL 문자나 내부에 포함된 NUL 문자는 없으며, 경로 끝에 슬래시가 붙지 않습니다. 대부분의 경우 경로는 유효한 UTF-8이지만, 항상 그렇지는 않습니다. 경로 저장 형식에 대한 자세한 내용은 <a href="https://github.com/git/git/blob/a08a83db2bf27f015bec9a435f6d73e223c21c5e/Documentation/technical/index-format.txt#L107-L124">Git 문서</a>를 참고하세요. 또한, <code>libgit2</code>는 해당 문서에서 설명하는 접두사 압축(prefix compression)을 자동으로 처리합니다.</p>
      </li>
      <li class="param-li">
        <span class="param-name">uid</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
    </ul>
  </li>
</ul>