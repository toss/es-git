# inmemoryIndex

마지막 작업에 의해 생성된 인덱스를 가져와요. 이 인덱스는 `next()`의 결과이며, 다음에 `commit()`을 호출할 때 커밋될 것입니다. 이는 커밋을 하기 전에 메모리에서 리베이스 중 충돌을 해결하는 데 유용해요.

이것은 메모리 내 리베이스에만 해당되며, 작업 디렉토리 내에서의 리베이스의 경우 변경사항은 리포지토리의 인덱스에 적용돼요.

## 시그니처

```ts
class Rebase {
  inmemoryIndex(): Index;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">인덱스</span>
    <br>
    <p class="param-description">마지막 작업에 의해 생성된 인덱스.</p>
  </li>
</ul>