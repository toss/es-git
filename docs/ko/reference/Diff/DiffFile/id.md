# id

해당 항목의 Oid를 반환해요.

만약 이 항목이 diff의 존재하지 않는 쪽을 나타낸다면 (예: `Added` delta의 `oldFile`), 반환되는 oid는 0으로 채워져 있어요.

## 시그니처

```ts
class DiffFile {
  id(): string;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">Oid를 반환해요.</p>
  </li>
</ul>