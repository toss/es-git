# format

이 describe 결과를 출력하고, 결과를 문자열로 반환해요.

## 시그니처

```ts
class Describe {
  format(options?: DescribeFormatOptions | null | undefined): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | DescribeFormatOptions</span>
    <br>
    <p class="param-description">describe를 포맷하기 위한 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">abbreviatedSize</span><span class="param-type">number</span>
        <br>
        <p class="param-description">사용할 축약된 커밋 ID의 길이를 설정해요. 이 값은 축약 문자열 길이의 하한이고, 기본값은 7이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">alwaysUseLongFormat</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">더 짧은 이름을 사용할 수 있는 경우에도 항상 긴 형식을 사용할지 여부를 설정해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">dirtySuffix</span><span class="param-type">string</span>
        <br>
        <p class="param-description">작업 디렉터리가 dirty 상태이고 이 값이 설정된 경우, 이 문자열을 description 문자열 끝에 덧붙여요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">이 describe에 대한 포맷된 문자열이에요.</p>
  </li>
</ul>