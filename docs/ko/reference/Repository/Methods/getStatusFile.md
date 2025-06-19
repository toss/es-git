# getStatusFile

단일 파일의 파일 상태를 가져와요.

이것은 제공한 파일 이름의 상태를 가져오려고 시도해요. 해당 이름과 일치하는 파일이 없으면 (HEAD, 인덱스 또는 작업 디렉토리에서) NotFound를 반환해요.

이름이 여러 파일과 일치하는 경우 (예를 들어, 경로가 디렉토리를 지정하거나 대소문자를 구분하지 않는 파일 시스템에서 실행 중이고 HEAD에 경로와 일치하는 두 개의 항목이 있는 경우), 올바른 결과를 제공할 수 없으므로 Ambiguous를 반환해요.

이것은 어떤 종류의 이름 변경 감지도 수행하지 않아요. 이름 변경에는 대상 집합이 필요하며 경로 필터링 때문에 이름 변경을 올바르게 확인할 수 있는 충분한 정보가 없어요. 이름 변경 감지와 함께 파일 상태를 확인하려면 전체 `statuses`를 수행하고 관심 있는 경로를 찾아 스캔하는 것 외에는 선택의 여지가 없어요.

## 시그니처

```ts
class Repository {
  getStatusFile(path: string): Status;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">단일 파일 경로</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Status</span>
    <br>
    <p class="param-description">이 단일 파일의 <code>Status</code></p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">conflicted</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">current</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">ignored</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexDeleted</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexModified</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexNew</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexRenamed</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexTypechange</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtDeleted</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtModified</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtNew</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtRenamed</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtTypechange</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
    </ul>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">상태 파일이 존재하지 않으면 오류를 던져요.</p>
  </li>
</ul>