# getUpstream

해당 브랜치의 업스트림을 조회해요.

## 시그니처

```ts
class Branch {
  getUpstream(): Branch;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Branch</span>
    <br>
    <p class="param-description">이 브랜치의 업스트림이에요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">업스트림이 존재하지 않으면 오류를 던져요.</p>
  </li>
</ul>