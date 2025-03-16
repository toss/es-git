# updateAll

작업 디렉터리와 일치하도록 모든 인덱스 항목들을 업데이트해요.

기존의 인덱스 항목들을 스캔하여 작업 디렉터리와 동기화하고, 해당 작업 디렉터리 파일이 더 이상 존재하지 않으면 삭제하며, 그렇지 않으면 정보를 업데이트해요 (필요한 경우 파일의 최신 버전을 ODB에 추가하는 것도 포함돼요).

## 시그니처

```ts
class Index {
  updateAll(pathspecs: string[], options?: IndexUpdateAllOptions): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">pathspecs</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string[]</span>
    <br>
    <p class="param-description">리포지토리의 작업 디렉터리의 파일들과 매칭될 파일 이름 또는 쉘 glob 패턴의 목록이에요. 매칭되는 각 파일은 인덱스에 추가돼요. (기존 항목은 업데이트되거나, 새 항목이 추가)</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | IndexUpdateAllOptions</span>
    <br>
    <p class="param-description">인덱스 항목을 업데이트하는데 사용할 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">onMatch</span><span class="param-type">(args: IndexOnMatchCallbackArgs) =&gt; number</span>
        <br>
        <p class="param-description">콜백 함수를 제공하면, 작업 디렉터리에서 일치하는 각 항목을 <code>index</code>에 추가하거나 업데이트하기 직전에 실행돼요. 콜백이 <code>0</code>을 반환하면 해당 항목이 <code>index</code>에 추가되고, <code>0</code>보다 큰 값을 반환하면 건너뛰며, <code>0</code>보다 작은 값을 반환하면 스캔을 중단하고 에러를 반환해요.</p>
      </li>
    </ul>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description"><code>bare</code> 인덱스 인스턴스에서는 오류를 발생시켜요.</p>
  </li>
</ul>