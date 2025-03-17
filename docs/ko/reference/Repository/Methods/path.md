# path

일반 리포지토리의 경우 `.git` 폴더의 경로를 반환하고, `bare` 리포지토리의 경우 리포지토리 자체의 경로를 반환해요.

## 시그니처

```ts
class Repository {
  path(): string;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">일반 리포지토리의 경우 <code>.git</code> 폴더의 경로를 반환하고, <code>bare</code> 리포지토리의 경우 리포지토리 자체의 경로를 반환해요.</p>
  </li>
</ul>