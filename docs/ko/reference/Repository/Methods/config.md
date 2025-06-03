# config

이 리포지토리의 구성 파일을 가져와요.

## 시그니처

```ts
class Repository {
  config(): Config;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Config</span>
    <br>
    <p class="param-description">구성 파일이 설정되지 않은 경우, 전역 및 시스템 구성(사용 가능한 경우)을 포함하여 리포지토리에 설정된 기본 구성이 반환돼요.</p>
  </li>
</ul>