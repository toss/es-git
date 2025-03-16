# addAll

작업 디렉터리 내의 파일과 일치하는 인덱스 항목들을 추가 또는 업데이트해요.

## 시그니처

```ts
class Index {
  addAll(pathspecs: string[], options?: IndexAddAllOptions): void;
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
    <span class="param-name">options</span><span class="param-type">null | IndexAddAllOptions</span>
    <br>
    <p class="param-description">인덱스 항목을 추가하거나 업데이트하는데 사용할 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">checkPathspec</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description"><code>git add -A</code>와 동일한 동작을 구현하려면 이 옵션을 설정하세요. <code>force</code>를 사용하지 않는 경우, <code>pathspecs</code> 목록에 무시된 파일의 정확한 경로가 포함돼 있다면 에러를 발생시켜요. <code>pathspecs</code>의 각 항목이 디스크상의 파일과 정확히 일치하는 경우, 그 파일이 무시되지 않았거나 이미 <code>index</code>에 있어야 해요. 이 조건을 충족하지 않으면 에러가 반환돼요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description"><code>pathspecs</code>는 파일 이름이나 셸 와일드카드 패턴으로, 작업 디렉터리에서 일치하는 파일을 찾아 <code>index</code>에 추가해요(기존 항목을 업데이트하거나 새 항목을 추가). <code>disablePathspecMatch</code> 플래그를 설정하면 와일드카드 패턴을 비활성화하고 정확한 경로 일치 방식으로 동작하게 할 수 있어요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">기본적으로 무시된 파일은 추가되지 않아요(<code>addPath</code>와 다름). 하지만 파일이 이미 <code>index</code>에 추적되고 있다면 무시된 상태라도 업데이트돼요. <code>force</code> 플래그를 설정하면 무시 규칙을 검사하지 않고 강제로 추가할 수 있어요.</p>
      </li>
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
    <p class="param-description"><code>bare</code> 인덱스 인스턴스인 경우 오류가 발생해요.</p>
  </li>
</ul>

## 예제

`git add *`를 수행:

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const index = repo.index();
index.addAll(['*']);
index.write();
```