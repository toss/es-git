# createMailmapFromBuffer

문자열의 내용으로 mailmap을 생성해요.

문자열의 형식은 mailmap 파일의 규칙을 따라야 해요:
```
# Comment line (ignored)
Seokju Me <seokju.me@toss.im> Seokju Na <seokju.me@gmail.com>
```

## 시그니처

```ts
function createMailmapFromBuffer(content: string): Mailmap;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">content</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">mailmap 파일의 내용</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Mailmap</span>
    <br>
    <p class="param-description">새로운 mailmap 개체</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">작업이 실패하면 오류</p>
  </li>
</ul>