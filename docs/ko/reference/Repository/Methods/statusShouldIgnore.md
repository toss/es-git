# statusShouldIgnore

주어진 파일에 무시 규칙이 적용되는지 테스트해요.

이 함수는 무시 규칙을 확인하여 주어진 파일에 적용되는지 확인해요. 이는 파일이 이미 인덱스에 있거나 리포지토리에 커밋되어 있는지와 관계없이 파일이 무시될지를 나타내요.

이를 이해하는 한 가지 방법은 파일이 포함된 디렉토리에서 "git add ."를 실행했을 때, 해당 파일이 추가될지 아닐지를 생각해보는 거예요.

## 시그니처

```ts
class Repository {
  statusShouldIgnore(path: string): boolean;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">주어진 파일 경로</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">무시 규칙이 주어진 파일에 적용되면 <code>true</code>를 반환</p>
  </li>
</ul>