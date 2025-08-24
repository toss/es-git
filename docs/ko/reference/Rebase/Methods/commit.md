# commit

현재 패치를 커밋해요. 리베이스 다음 호출에서 적용된 패치 중에 생긴 충돌을 모두 해결해야 해요.

## 시그니처

```ts
class Rebase {
  commit(options: RebaseCommitOptions): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">RebaseCommitOptions</span>
    <br>
    <p class="param-description">패치를 커밋하기 위한 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">author</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">저자의 서명이에요. 원본 커밋에서 저자를 유지하려면 이 항목을 비워 두면 돼요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에서 이메일이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에서 이름이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
            <br>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">offset</span><span class="param-type">number</span>
                <br>
                <p class="param-description">시간대 오프셋, 분 단위</p>
              </li>
              <li class="param-li">
                <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
                <br>
                <p class="param-description">초 단위, 에포크에서부터의 시간</p>
              </li>
            </ul>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">committer</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">커미터의 서명이에요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에서 이메일이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명에서 이름이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
            <br>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">offset</span><span class="param-type">number</span>
                <br>
                <p class="param-description">시간대 오프셋, 분 단위</p>
              </li>
              <li class="param-li">
                <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
                <br>
                <p class="param-description">초 단위, 에포크에서부터의 시간</p>
              </li>
            </ul>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">message</span><span class="param-type">string</span>
        <br>
        <p class="param-description">원본 커밋에서 메시지를 유지하려면 이 항목을 비워 두면 돼요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">생성된 커밋의 커밋 ID에요.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const rebase = repo.rebase(...);
const sig = { name: 'Seokju Na', email: 'seokju.me@toss.im' };
for (const op of rebase) {
  rebase.commit({ committer: sig });
}
```