# hideGlob

주어진 glob 패턴과 일치하는 레퍼런스를 숨겨요.

이 메서드를 사용하면 해당 레퍼런스가 가리키는 OID와 그 조상 커밋들이 Revwalk 결과에서 제외돼요.

## 시그니처

```ts
class Revwalk {
  hideGlob(glob: string): this;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">glob</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">
      앞에 <code>refs/</code>가 없으면 자동으로 추가되며,  
      <code>?</code>, <code>*</code>, <code>[</code> 문자가 없으면  
      끝에 <code>/*</code>가 자동으로 붙어요.  
      이 glob과 일치하는 레퍼런스 중 커밋을 가리키지 않는 것은 무시돼요.
</p>
  </li>
</ul>