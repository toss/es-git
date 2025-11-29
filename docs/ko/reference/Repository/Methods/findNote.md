# findNote

개체에 대한 노트를 읽어요.

`notesRef` 인수는 사용할 기준 참조 이름이고, 기본값은 "refs/notes/commits"예요.

지정된 id는 노트를 읽을 Git 개체의 Oid예요.

## 시그니처

```ts
class Repository {
  findNote(id: string, options?: FindNoteOptions | null | undefined): Note | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">id</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">노트를 읽을 Git 개체의 OID 값</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | FindNoteOptions</span>
    <br>
    <p class="param-description">노트를 찾을 때 사용할 옵션</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">notesRef</span><span class="param-type">string</span>
        <br>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Note</span>
    <br>
    <p class="param-description">노트 인스턴스. 존재하지 않으면 <code>null</code>을 반환하는 값</p>
  </li>
</ul>