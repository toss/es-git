# setVersion

디스크에 기록된 인덱스 버전을 설정해요.

## 시그니처

```ts
class Index {
  setVersion(version: number): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">version</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">유효한 값은 2, 3 또는 4예요. 만약 2가 주어지면, 인덱스를 정확하게 표현하기 위해 필요한 경우 git_index_write가 버전 3의 인덱스를 기록할 수도 있어요.</p>
  </li>
</ul>
