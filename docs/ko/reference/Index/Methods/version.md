# version

디스크에 기록된 인데스 버전을 가져와요.

## 시그니처

```ts
class Index {
  version(): number;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">number</span>
    <br>
    <p class="param-description">유효한 반환 값은 2, 3 또는 4예요. 만약 3이 반환된다면, 확장 데이터가 필요 없을 경우 버전 2의 인덱스가 기록될 수도 있어요.</p>
  </li>
</ul>