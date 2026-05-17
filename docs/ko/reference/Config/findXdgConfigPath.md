# findXdgConfigPath

전역 XDG 호환 구성 파일의 경로를 찾아요.

XDG 호환 구성 파일은 보통 `$HOME/.config/git/config`에 위치해요.

## 시그니처

```ts
function findXdgConfigPath(): string | null;
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string | null</span>
    <br>
    <p class="param-description">XDG 호환 구성 파일의 경로.</p>
  </li>
</ul>