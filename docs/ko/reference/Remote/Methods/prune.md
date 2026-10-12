# prune

리모트에서 더 이상 존재하지 않는 추적 레퍼런스를 정리해요.

## 시그니처

```ts
class Remote {
  prune(options?: PruneOptions | null | undefined, signal?: AbortSignal | null | undefined): Promise<void>;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | PruneOptions</span>
    <br>
    <p class="param-description">리모트 정리(prune) 작업을 위한 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">credential</span><span class="param-type">Credential</span>
        <br>
        <p class="param-description">libgit2에서 Git 인증 정보를 나타내는 인터페이스예요.<br><br><code>SSHKeyFromPath</code>에는 <code>privateKeyPath</code>, <code>SSHKey</code>에는 <code>privateKey</code>, <code>Plain</code>에는 <code>password</code>가 필요해요. 빈 비밀번호도 허용하며, 사용자 이름의 기본값은 <code>&quot;git&quot;</code>이에요. 공개 키와 패스프레이즈는 선택 사항이에요.<br><br>인증 정보는 연결 전에 검증하며, 인증이 필요 없는 공개 또는 로컬 리모트에도 적용돼요. 인증이 필요하지 않으면 <code>credential</code>을 생략하세요.</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">요청을 중단할 때 사용할 <code>AbortSignal</code> 객체예요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description"><code>options.credential</code>에 <code>type</code>별 필수 필드(<code>SSHKeyFromPath</code>의 <code>privateKeyPath</code>, <code>SSHKey</code>의 <code>privateKey</code>, <code>Plain</code>의 <code>password</code>)가 없으면 <code>InvalidArg</code> 오류가 발생해요. 인증이 필요 없는 리모트에도 적용돼요. prune 작업이 실패해도 오류가 발생해요.</p>
  </li>
</ul>
