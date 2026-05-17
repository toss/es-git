# remove

가장 높은 수준의 구성 파일(보통 로컬 파일)에서 구성 변수를 삭제해요.

## 시그니처

```ts
class Config {
  remove(name: string): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">구성 항목의 이름.</p>
  </li>
</ul>