# rename

기존 레퍼런스의 이름을 변경해요.

이 메서드는 직접 레퍼런스(direct reference)와 심볼릭 레퍼런스(symbolic reference) 모두에서 작동해요.

## 시그니처

```ts
class Reference {
  rename(newName: string, options?: RenameReferenceOptions | null | undefined): Reference;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">newName</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">변경할 레퍼런스의 새 이름이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | RenameReferenceOptions</span>
    <br>
    <p class="param-description">레퍼런스 이름 변경 시 사용할 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">
          <code>force</code> 플래그가 활성화되지 않았고, 같은 이름의 레퍼런스가 이미 존재하면 이름 변경이 실패해요.
        </p>
      </li>
      <li class="param-li">
        <span class="param-name">logMessage</span><span class="param-type">string</span>
        <br>
        <p class="param-description">변경 기록에 남길 메시지예요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Reference</span>
    <br>
    <p class="param-description">이름이 변경된 레퍼런스를 반환해요.</p>
  </li>
</ul>
