# workdir

현재 리포지토리의 작업 디렉터리 경로를 가져와요.

## 시그니처

```ts
class Repository {
  workdir(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">리포지토리의 작업 디렉터리 경로를 반환해요. <code>bare</code> 리포지토리인 경우 <code>null</code>을 반환해요.</p>
  </li>
</ul>