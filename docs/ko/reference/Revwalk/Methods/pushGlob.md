# pushGlob

주어진 glob 패턴과 일치하는 레퍼런스를 추가해요.

이 메서드를 사용하면 해당 레퍼런스가 가리키는 OID들이 Revwalk의 탐색 시작점으로 추가돼요.

## 시그니처

```ts
class Revwalk {
  pushGlob(glob: string): this;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">glob</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">
탐색할 레퍼런스를 지정하는 glob 패턴이에요.  
      <code>refs/</code>로 시작하지 않으면 자동으로 추가되며,  
      <code>?</code>, <code>*</code>, <code>[</code> 문자가 없으면  
      끝에 <code>/*</code>가 자동으로 붙어요.  
      커밋을 가리키지 않는 레퍼런스는 무시돼요.</p>
  </li>
</ul>