# findGlobalConfigPath

전역 구성 파일의 경로를 찾아요.

사용자 또는 전역 구성 파일은 보통 `$HOME/.gitconfig`에 위치해요.

이 메서드는 파일이 존재하는 경우 해당 파일의 전체 경로를 추측하려고 시도해요. 반환된 경로는 전역 구성 파일을 로드하기 위한 모든 메서드 호출에 사용될 수 있어요.

이 메서드는 XDG 호환 구성 파일(`.config/git/config`)의 경로를 추측하지 않아요.

## 시그니처

```ts
function findGlobalConfigPath(): string | null;
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string | null</span>
    <br>
    <p class="param-description">전역 구성 파일의 경로.</p>
  </li>
</ul>