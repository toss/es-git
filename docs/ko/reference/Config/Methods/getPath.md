# getPath

경로 구성 변수의 값을 가져와요.

선행 '~'는 전역 검색 경로로 확장돼요 (기본값은 사용자의 홈 디렉토리이지만 [`git_libgit2_opts`][1]를 통해 재정의할 수 있어요).

[1]: https://libgit2.org/docs/reference/v1.9.0/common/git_libgit2_opts.html

모든 구성 파일은 정의된 수준의 순서대로 검색돼요. 수준이 높을수록 우선순위가 높아요. 변수의 첫 번째 항목이 여기에 반환돼요.

## 시그니처

```ts
class Config {
  getPath(name: string): string;
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
    <span class="param-type">string</span>
    <br>
    <p class="param-description">경로 구성 변수의 값.</p>
  </li>
</ul>