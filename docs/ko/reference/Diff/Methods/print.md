# print

포맷된 텍스트 출력을 생성하며 diff를 순회해요.

## 시그니처

```ts
class Diff {
  print(options?: DiffPrintOptions | null | undefined): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | DiffPrintOptions</span>
    <br>
    <p class="param-description">츌력에 사용할 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">format</span><span class="param-type">DiffFormat</span>
        <br>
        <p class="param-description">출력 포맷을 지정해요.<br>- <code>Patch</code>: 전체 <code>git diff</code> (기본)<br>- <code>PatchHeader</code> : 패치의 헤더만 출력<br>- <code>Raw</code> : <code>git diff --raw</code>와 동일하게 패치의 헤더 출력<br>- <code>NameOnly</code> : <code>git diff --name-only</code>와 동일한 출력<br>- <code>NameStatus</code> : <code>git diff --name-status</code>와 동일한 출력<br>- <code>PatchId</code> : <code>git patch-id</code>에서 사용하는 <code>git diff</code> 출력</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">포맷팅된 출력 값을 반환해요.</p>
  </li>
</ul>
