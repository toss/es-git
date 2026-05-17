# findSystemConfigPath

시스템 구성 파일의 경로를 찾아요.

`/etc/gitconfig`가 존재하지 않으면 `%PROGRAMFILES%`를 찾아요.

## 시그니처

```ts
function findSystemConfigPath(): string | null;
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string | null</span>
    <br>
    <p class="param-description">시스템 구성 파일의 경로.</p>
  </li>
</ul>