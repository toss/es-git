# removeAll

일치하는 모든 인덱스 항목들을 제거해요.

## 시그니처

```ts
class Index {
  removeAll(pathspecs: string[], options?: IndexRemoveAllOptions): void;
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
    <span class="param-name">options</span><span class="param-type">null | IndexRemoveAllOptions</span>
    <br>
    <p class="param-description">인덱스 항목을 제거할 때 사용할 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">onMatch</span><span class="param-type">(args: IndexOnMatchCallbackArgs) =&gt; number</span>
        <br>
        <p class="param-description">콜백 함수를 제공하면, 작업 디렉터리에서 일치하는 각 항목을 <code>index</code>에 추가하거나 업데이트하기 직전에 실행돼요. 콜백이 <code>0</code>을 반환하면 해당 항목이 <code>index</code>에 추가되고, <code>0</code>보다 큰 값을 반환하면 건너뛰며, <code>0</code>보다 작은 값을 반환하면 스캔을 중단하고 에러를 반환해요.</p>
      </li>
    </ul>
  </li>
</ul>