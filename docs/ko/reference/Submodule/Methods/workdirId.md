# workdirId

현재 작업 디렉터리의 서브모듈에 대한 OID를 가져와요.

이 메서드는 체크아웃된 서브모듈에서 `HEAD`를 조회하는 것에 해당하는 OID를 반환해요. 인덱스에 보류 중인 변경 사항이 있거나 다른 어떤 것이 있더라도, 이 메서드는 그런 상태를 알아차리지 못해요.

## 시그니처

```ts
class Submodule {
  workdirId(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">현재 작업 디렉터리의 서브모듈에 대한 OID</p>
  </li>
</ul>