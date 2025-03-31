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
        <p class="param-description">Git 인증 정보를 나타내는 인터페이스예요.</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">요청을 중단할 때 사용할 <code>AbortSignal</code> 객체예요.</p>
  </li>
</ul>
