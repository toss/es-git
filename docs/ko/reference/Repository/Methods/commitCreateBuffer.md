# commitCreateBuffer

외부에서 서명할 커밋 객체의 원본 내용을 만들어요.

서명되지 않은 커밋 내용을 만들지만 객체 데이터베이스에는 저장하지 않아요.
반환된 문자열의 UTF-8 바이트를 그대로 서명한 뒤 `commit`에 `signature`와 `updateRef`를
전달하면 서명된 커밋을 저장하고 레퍼런스를 갱신할 수 있어요.
레퍼런스를 갱신하지 않고 객체만 저장하려면 `commitSigned`를 사용하세요.
공백이나 줄바꿈을 포함해 내용을 변경하면 서명이 유효하지 않게 돼요.

`commit`을 사용할 때는 두 메서드에 같은 트리, 메시지, 부모 커밋, 작성자, 커미터를
전달하고 작성자와 커미터의 `timeOptions`를 고정하세요. 그렇지 않으면 `commit`이
내용을 다시 만들 때 타임스탬프가 달라질 수 있어요. 두 저장 메서드 모두 서명을 검증하지 않아요.

## 시그니처

```ts
class Repository {
  commitCreateBuffer(
    tree: Tree,
    message: string,
    options?: CommitCreateBufferOptions | null | undefined,
  ): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">tree</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Tree</span>
    <br>
    <p class="param-description">커밋 내용을 만들 트리 객체예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">message</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">커밋 메시지예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">CommitCreateBufferOptions | null</span>
    <br>
    <p class="param-description">커밋 내용 생성 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">author</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">작성자 정보(이름, 이메일, 시간)예요. 지정하지 않으면 리포지토리의 기본 서명을 사용해요. 기본 서명이 설정되어 있지 않으면 오류가 발생해요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에 사용할 이메일 주소예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에 사용할 이름이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
            <br>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">offset</span><span class="param-type">number</span>
                <br>
                <p class="param-description">분 단위 시간대 오프셋이에요.</p>
              </li>
              <li class="param-li">
                <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
                <br>
                <p class="param-description">Unix epoch 기준 초 단위 시간이에요.</p>
              </li>
            </ul>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">committer</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">커미터 정보(이름, 이메일, 시간)예요. 지정하지 않으면 리포지토리의 기본 서명을 사용해요. 기본 서명이 설정되어 있지 않으면 오류가 발생해요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에 사용할 이메일 주소예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에 사용할 이름이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
            <br>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">offset</span><span class="param-type">number</span>
                <br>
                <p class="param-description">분 단위 시간대 오프셋이에요.</p>
              </li>
              <li class="param-li">
                <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
                <br>
                <p class="param-description">Unix epoch 기준 초 단위 시간이에요.</p>
              </li>
            </ul>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">parents</span><span class="param-type">string[]</span>
        <br>
        <p class="param-description">부모 커밋 ID 목록이에요. 첫 번째 부모는 이 커밋이 바로 뒤를 잇는 커밋이에요. 루트 커밋을 만들려면 생략하세요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">외부에서 서명할 커밋 내용이에요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">작성자나 커미터 정보가 유효하지 않거나, 기본 서명이 설정되어 있지 않거나,<br>부모 커밋이 존재하지 않을 때 발생해요.</p>
  </li>
</ul>

## 예제

```ts
import { execFileSync } from 'node:child_process';

// Requires GPG with a signing key configured.
const identity = {
  name: 'Seokju Na',
  email: 'seokju.me@toss.im',
  timeOptions: { timestamp: Math.floor(Date.now() / 1000), offset: 0 },
};
const options = { author: identity, committer: identity, parents: [repo.head().target()!] };
const content = repo.commitCreateBuffer(tree, 'signed commit', options);
const signature = execFileSync('gpg', ['--detach-sign', '--armor'], {
  input: Buffer.from(content, 'utf8'),
}).toString('utf8');
const oid = repo.commit(tree, 'signed commit', { ...options, signature, updateRef: 'HEAD' });
```