# treeId

이 커밋이 가리키는 트리의 ID를 가져와요.

ODB에서 Git 개체를 가져오기 위한 시도는 하지 않아요.

## 시그니처

```ts
class Commit {
  treeId(): string;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">커밋이 가리키는 트리의 ID 값</p>
  </li>
</ul>