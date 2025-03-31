# body

git 커밋 메시지의 긴 "본문"을 가져와요.

반환되는 메시지는 커밋의 본문으로, 메시지의 첫 번째 단락을 제외한 모든 내용을 포함하며, 선행 및 후행 공백이 제거돼요.

## 시그니처

```ts
class Commit {
  body(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">커밋 메시지의 긴 본문을 반환해요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">올바른 utf-8이 아닐 경우 에러를 발생시켜요.</p>
  </li>
</ul>
