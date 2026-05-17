# stashSave

로컬 변경 사항을 새 스태시에 저장해요.

이 메서드는 현재 작업 디렉터리와 인덱스 상태를 새 스태시 항목에 저장하여, 변경 사항을 임시로 보관하고 다른 작업을 할 수 있어요. 스태시 후 작업 디렉터리는 HEAD 커밋과 일치하도록 되돌아가요.

## 시그니처

```ts
class Repository {
  stashSave(options?: StashSaveOptions): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">StashSaveOptions | null</span>
    <br>
    <p class="param-description">스태시 저장 옵션.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">includeIgnored</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">무시된 파일을 스태시할지 여부. 기본값: false</p>
      </li>
      <li class="param-li">
        <span class="param-name">includeUntracked</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">추적되지 않은 파일을 스태시할지 여부. 기본값: false</p>
      </li>
      <li class="param-li">
        <span class="param-name">keepIndex</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">스태시 후 인덱스를 유지할지 여부. true이면 스태시 후에도 스테이징된 변경 사항이 인덱스에 남아 있어요. 기본값: false</p>
      </li>
      <li class="param-li">
        <span class="param-name">message</span><span class="param-type">string</span>
        <br>
        <p class="param-description">스태시 상태와 함께 저장되는 설명. 제공하지 않으면 기본 메시지가 생성돼요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">stasher</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">스태시를 수행하는 사람의 신원. 제공하지 않으면 리포지토리의 기본 서명을 사용해요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명의 이메일.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">서명의 이름.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
            <br>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">offset</span><span class="param-type">number</span>
                <br>
                <p class="param-description">시간대 오프셋(분 단위)</p>
              </li>
              <li class="param-li">
                <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
                <br>
                <p class="param-description">에포크 기준 시간(초 단위)</p>
              </li>
            </ul>
          </li>
        </ul>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">스태시 상태를 담고 있는 커밋의 개체 ID(40자 SHA1).</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">스태시할 로컬 변경 사항이 없거나 스태시 작업이 실패한 경우.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');

// Simple stash
const stashId = repo.stashSave({
  stasher: { name: 'Seokju Na', email: 'seokju.me@toss.im' },
  message: 'WIP: implementing new feature'
});

// Stash including untracked files
repo.stashSave({
  stasher: { name: 'Seokju Na', email: 'seokju.me@toss.im' },
  includeUntracked: true
});
```