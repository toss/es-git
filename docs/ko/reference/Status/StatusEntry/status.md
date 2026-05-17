# status

이 파일의 상태에 액세스해요.

## 시그니처

```ts
class StatusEntry {
  status(): Status;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Status</span>
    <br>
    <p class="param-description">이 항목의 상태 데이터.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">conflicted</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">current</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">ignored</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexDeleted</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexModified</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexNew</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexRenamed</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexTypechange</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtDeleted</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtModified</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtNew</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtRenamed</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtTypechange</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtUnreadable</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
    </ul>
  </li>
</ul>