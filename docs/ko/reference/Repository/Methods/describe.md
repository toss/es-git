# describe

커밋을 설명해요

현재 커밋과 작업 트리에 대해 describe 연산을 수행해요.
`HEAD`에 대해 describe를 수행한 다음 status를 실행하고, 변경 사항이 있으면 설명이 더럽다고 간주해요.

## 시그니처

```ts
class Repository {
  describe(options?: DescribeOptions | null | undefined): Describe;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | DescribeOptions</span>
    <br>
    <p class="param-description">describe 연산에 대한 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">describeAll</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">참조 조회 전략을 설정해요. git-describe의 <code>--all</code> 옵션처럼 동작해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">describeTags</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">참조 조회 전략을 설정해요. git-describe의 <code>--tags</code> 옵션처럼 동작해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">maxCandidatesTags</span><span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">onlyFollowFirstParent</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">일치하는 태그나 참조로부터의 거리를 계산할 때, 첫 번째 부모 조상 경로만 따라갈지 여부를 나타내요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">pattern</span><span class="param-type">string</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">showCommitOidAsFallback</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">일치하는 태그나 참조를 찾지 못해 describe 연산이 일반적으로 실패할 경우에 대한 동작을 제어해요. 이 옵션이 설정되면, 대신 커밋의 전체 ID를 보여주도록 해요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Describe</span>
    <br>
    <p class="param-description">describe 인스턴스에요.</p>
  </li>
</ul>