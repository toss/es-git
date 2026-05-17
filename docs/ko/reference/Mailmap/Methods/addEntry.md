# addEntry

새 Mailmap 항목을 추가해요.

작성자/커미터(`replace_name` 및 `replace_email`로 지정)를 지정된 실제 이름 및 이메일에 매핑해요. `replace_email`은 필수이지만 다른 매개변수는 null일 수 있어요.

`replace_name`과 `replace_email`이 모두 제공되면 항목은 두 가지 모두 일치하는 사람에게 적용돼요. `replace_name`만 제공되면 이메일에 관계없이 해당 이름을 가진 모든 사람에게 적용돼요. `replace_email`만 제공되면 이름에 관계없이 해당 이메일을 가진 모든 사람에게 적용돼요.

## 시그니처

```ts
class Mailmap {
  addEntry(entry: AddMailmapEntryData): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">entry</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">AddMailmapEntryData</span>
    <br>
    <p class="param-description">Mailmap 항목 데이터.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">realEmail</span><span class="param-type">string</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">realName</span><span class="param-type">string</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">replaceEmail</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">replaceName</span><span class="param-type">string</span>
        <br>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">void</span>
    <br>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">작업이 실패한 경우 오류.</p>
  </li>
</ul>