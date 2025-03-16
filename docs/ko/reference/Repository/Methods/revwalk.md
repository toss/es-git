# revwalk

커밋 그래프를 탐색할 수 있는 Revwalk를 생성해요.

## 시그니처

```ts
class Repository {
  revwalk(): Revwalk;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Revwalk</span>
    <br>
    <p class="param-description">이 리포지토리의 커밋 그래프를 순회하는 Revwalk를 반환해요.</p>
  </li>
</ul>