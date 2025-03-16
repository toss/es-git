# merge

하나의 diff를 다른 diff에 병합해요.

이 메소드는 "from" 리스트의 항목들을 "self" 리스트에 병합해, 두 리스트 중 어느 한쪽에 나타나는 모든 항목들이 포함된 결과 diff를 만들어요.
만약 항목이 두 리스트 모두에 존재한다면, 해당 항목은 "onto" 리스트의 구버전과 "from" 리스트의 신버전이 병합된 것처럼 나타나요
(단, 해당 항목에 중간에 대기 중인 DELETE가 있으면 삭제된 것으로 표시돼요).

## 시그니처

```ts
class Diff {
  merge(diff: Diff): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">diff</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Diff</span>
    <br>
    <p class="param-description">병합할 다른 Diff예요.</p>
  </li>
</ul>