# path

디스크상의 인덱스 파일 전체 경로를 가져와요.

## 시그니처

```ts
class Index {
  path(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">만약 인메모리 인덱스라면 <code>null</code>을 반환해요.</p>
  </li>
</ul>