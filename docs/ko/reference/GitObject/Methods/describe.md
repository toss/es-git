# describe

커밋을 설명해요

이 commitish Git 개체에 대해 describe 연산을 수행해요.

## 시그니처

```ts
class Object {
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
        <p class="param-description">태그 후보의 최대 개수예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">onlyFollowFirstParent</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">일치하는 태그나 참조로부터 거리를 계산할 때, 첫 번째 부모 계보만 따라갈지 여부를 나타내는 값이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">pattern</span><span class="param-type">string</span>
        <br>
        <p class="param-description">태그나 참조를 필터링할 때 사용할 패턴 문자열이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">showCommitOidAsFallback</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">일치하는 태그나 참조를 찾지 못해 describe 연산이 보통 실패하는 경우에 대한 동작을 결정해요. 이 옵션이 설정되면 실패하는 대신 커밋의 전체 id를 보여줘요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Describe</span>
    <br>
    <p class="param-description">describe 인스턴스예요.</p>
  </li>
</ul>