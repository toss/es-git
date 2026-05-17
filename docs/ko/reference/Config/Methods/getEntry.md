# getEntry

구성 변수의 항목을 가져와요.

## 시그니처

```ts
class Config {
  getEntry(name: string): ConfigEntry;
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

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">ConfigEntry</span>
    <br>
    <p class="param-description"><code>Config</code> 인스턴스가 소유한 특정 항목을 나타내는 <code>ConfigEntry</code> 객체.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">includeDepth</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">이 변수가 발견된 포함 깊이.</p>
      </li>
      <li class="param-li">
        <span class="param-name">level</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">ConfigLevel</span>
        <br>
        <p class="param-description">이 항목의 구성 수준.</p>
        <p class="param-description">- <code>ProgramData</code> : 이식 가능한 git과의 호환성을 위해 Windows에서 시스템 전체에 적용돼요.<br>- <code>System</code> : 시스템 전체 구성 파일. (예: <code>/etc/gitconfig</code>)<br>- <code>XDG</code> : XDG 호환 구성 파일. (예: <code>~/.config/git/config</code>)<br>- <code>Global</code> : 사용자별 구성. (예: <code>~/.gitconfig</code>)<br>- <code>Local</code> : 리포지토리별 구성. (예: <code>$PWD/.git/config</code>)<br>- <code>Worktree</code> : 작업 트리별 구성 파일. (예: <code>$GIT_DIR/config.worktree</code>)<br>- <code>App</code> : 애플리케이션별 구성 파일.<br>- <code>Highest</code> : 사용 가능한 최고 수준.</p>
      </li>
      <li class="param-li">
        <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">이 항목의 이름.</p>
      </li>
      <li class="param-li">
        <span class="param-name">value</span><span class="param-type">string</span>
        <br>
        <p class="param-description">이 항목의 값. 값이 정의되지 않은 경우 값은 <code>null</code>이 돼요.</p>
      </li>
    </ul>
  </li>
</ul>