# symbolicTarget

심볼릭 레퍼런스가 가리키는 레퍼런스의 전체 이름을 가져와요.

이 메서드는 레퍼런스가 심볼릭일 때만 사용할 수 있어요.

## 시그니처

```ts
class Reference {
  symbolicTarget(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">심볼릭 레퍼런스가 가리키는 레퍼런스의 전체 이름을 반환해요.</p>
  </li>
</ul>