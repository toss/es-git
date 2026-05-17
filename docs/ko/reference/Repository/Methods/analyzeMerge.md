# analyzeMerge

주어진 브랜치를 분석하고 리포지토리의 HEAD에 병합할 수 있는 기회를 결정해요.

## 시그니처

```ts
class Repository {
  analyzeMergeFor(theirHeads: AnnotatedCommit[]): MergeAnalysisResult;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">theirHeads</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">AnnotatedCommit[]</span>
    <br>
    <p class="param-description">병합할 대상 헤드.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">MergeAnalysisResult</span>
    <br>
    <p class="param-description">병합 분석 결과.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">analysis</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">MergeAnalysis</span>
        <br>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">fastForward</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">주어진 병합 입력이 HEAD에서 fast-forward이며 병합을 수행할 필요가 없어요. 대신 클라이언트는 주어진 병합 입력을 체크아웃할 수 있어요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">none</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">병합이 불가능해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">normal</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">&quot;일반&quot; 병합이에요. HEAD와 주어진 병합 입력 모두 공통 조상에서 분기되었어요. 분기된 커밋은 병합되어야 해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">unborn</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">현재 리포지토리의 HEAD가 &quot;unborn&quot; 상태이며 유효한 커밋을 가리키지 않아요. 병합을 수행할 수 없지만, 호출자는 HEAD를 대상 커밋으로 설정할 수 있어요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">upToDate</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">주어진 모든 병합 입력이 HEAD에서 도달 가능해요. 이는 리포지토리가 최신 상태이며 병합을 수행할 필요가 없음을 의미해요.</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">preference</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">MergePreference</span>
        <br>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">fastForwardOnly</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description"><code>merge.ff=only</code> 구성 설정이 있으며, 사용자가 fast-forward 병합만 원한다는 것을 나타내요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">noFastForward</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description"><code>merge.ff=false</code> 구성 설정이 있으며, 사용자가 fast-forward 병합을 허용하지 않으려 한다는 것을 나타내요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">none</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">병합에 대한 선호 동작을 제안하는 구성이 없어요.</p>
          </li>
        </ul>
      </li>
    </ul>
  </li>
</ul>