# commitSigned

외부에서 서명한 커밋 내용으로 서명된 커밋을 만들어요.

서명된 커밋을 객체 데이터베이스에 저장하지만 `HEAD`나 다른 레퍼런스는 갱신하지 않아요.
레퍼런스도 갱신하려면 `commit`에 `signature`와 `updateRef`를 전달하세요.
`signatureField`를 생략하면 Git의 기본 필드인 `gpgsig`를 사용해요. 서명 자체는 검증하지 않아요.
서명을 저장하기 전에 마지막 줄바꿈 하나를 제거해요.

## 시그니처

```ts
class Repository {
  commitSigned(commitContent: string, signature: string, signatureField?: string | null | undefined): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">commitContent</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description"><code>commitCreateBuffer</code>가 반환한 커밋 내용이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signature</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description"><code>commitContent</code>에 대한 ASCII-armored 형식의 서명이에요. <code>gpg --detach-sign --armor</code>나 <code>ssh-keygen -Y sign -n git</code>의 출력이 여기에 해당해요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signatureField</span><span class="param-type">string | null</span>
    <br>
    <p class="param-description">헤더 필드 이름이에요. 기본값은 <code>gpgsig</code>예요. 비어 있으면 안 되고, 공백 문자나 NUL 바이트를 포함할 수 없어요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">생성한 커밋의 ID(SHA1)예요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">커밋 내용을 파싱할 수 없거나, 트리나 부모 커밋이 존재하지 않거나,<br>인자에 NUL 바이트가 있거나, 서명 필드가 비어 있거나 공백 문자를 포함할 때 발생해요.</p>
  </li>
</ul>

## 예제

```ts
import { execFileSync } from 'node:child_process';

// Requires GPG with a signing key configured.
const content = repo.commitCreateBuffer(tree, 'signed commit', {
  parents: [repo.head().target()!],
});
const signature = execFileSync('gpg', ['--detach-sign', '--armor'], {
  input: Buffer.from(content, 'utf8'),
}).toString('utf8');
const oid = repo.commitSigned(content, signature); // HEAD is unchanged.
```