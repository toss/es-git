# findSimilar

diff 파일 내에서 파일 리네임, 복사 등을 반영하도록 변환해요.

이 메서드는 diff 파일 내 기존 항목들을 파일 리네임이나 복사처럼 보이는 항목들로 교체해요. 요청에 따라, 변경량이 임계치보다 큰 수정 파일은 추가/삭제 쌍으로 분리할 수도 있어요.

## 시그니처

```ts
class Diff {
  findSimilar(options?: DiffFindOptions): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | DiffFindOptions</span>
    <br>
    <p class="param-description">diff를 찾기 위한 옵션들을 지정해요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">all</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">모든 탐색 기능을 활성화해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">breakRewrites</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">크게 수정된 항목들을 실제로 삭제/추가 쌍으로 분리해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">breakRewritesForRenamesOnly</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">리네임에 기여하는 경우에만 수정 분할을 수행해요. 보통 <code>breakRewrites</code>와 <code>rewrites</code> 기능은 수정된 파일의 자기 유사도를 측정하여 큰 변경이 있을 경우 삭제/추가 쌍으로 분리한 후, 해당 쌍의 항목들을 리네임 혹은 복사 후보로 검토해요. 만약 이 플래그를 함께 사용했을 때 분리된 쌍이 실제 리네임이나 복사로 사용되지 않으면, 수정 기록이 분리되지 않은 일반 수정 항목으로 복원돼요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">breakRewriteThreshold</span><span class="param-type">number</span>
        <br>
        <p class="param-description">수정 분할 기준 유사도 (기본값: 60)</p>
      </li>
      <li class="param-li">
        <span class="param-name">copies</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">복사 여부를 탐색해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">copiesFromUnmodified</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">변경되지 않은 항목을 복사 원본으로 사용할지 결정해요. 이 기능을 제대로 사용하려면 초기 diff 생성 시 <code>includeUnmodified</code> 옵션을 사용해야 해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">copyThreshold</span><span class="param-type">number</span>
        <br>
        <p class="param-description">파일 복사로 인식할 기준 유사도 (기본값: 50)</p>
      </li>
      <li class="param-li">
        <span class="param-name">dontIgnoreWhitespace</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">모든 데이터를 포함하여 유사도를 측정해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">exactMatchOnly</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">SHA 비교만으로 유사도를 측정해요. 빠르고 간단해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">forUntracked</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">작업 디렉토리의 추적되지 않은 항목들에 대해 리네임/복사를 탐색해요. 이를 올바르게 사용하려면 초기 diff 생성 시 <code>includeUntracked</code> 옵션을 함께 사용해야 해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreLeadingWhitespace</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">앞쪽 공백을 무시하며 유사도를 측정해요(기본값).</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">모든 공백을 무시하며 유사도를 측정해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">removeUnmodified</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">findSimilar 실행 후 변경되지 않은 델타들을 제거해요. <code>copiesFromUnmodified</code>를 사용하여 <code>--find-copies-harder</code>와 유사한 동작을 하려면, 초기 diff 생성 시 <code>includeUnmodified</code> 옵션을 사용해야 해요. 만약 최종 결과에서 변경되지 않은 항목들을 제외하고 싶다면 이 플래그를 활성화하세요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">renameFromRewriteThreshold</span><span class="param-type">number</span>
        <br>
        <p class="param-description">정 항목이 리네임 원본이 되기 위한 유사도 기준 (기본값: 50)</p>
      </li>
      <li class="param-li">
        <span class="param-name">renameLimit</span><span class="param-type">number</span>
        <br>
        <p class="param-description">파일 당 탐색할 최대 리네임 원본 후보 수 (git-diff의 <code>-l</code> 옵션이나 <code>diff.renameLimit</code> 설정과 유사하며 기본값은 200입니다.)</p>
      </li>
      <li class="param-li">
        <span class="param-name">renames</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">리네임을 탐색해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">renamesFromRewrites</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">수정된 항목의 옛날 쪽을 리네임 후보로 고려해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">renameThreshold</span><span class="param-type">number</span>
        <br>
        <p class="param-description">파일 리네임으로 고려할 유사도 기준 (기본값: 50)</p>
      </li>
      <li class="param-li">
        <span class="param-name">rewrites</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">중요한 수정 사항을 표시하여 분리해요.</p>
      </li>
    </ul>
  </li>
</ul>