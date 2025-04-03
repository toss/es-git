# push

로컬 변경 사항을 리모트로 푸시(push)해요.

이 메서드는 푸시를 수행하는 모든 단계를 실행해요.  
만약 refspecs를 전달하지 않으면, 리모트에 설정된 기본 refspec이 사용돼요.

## 시그니처

```ts
class Remote {
  push(
    refspecs: string[],
    options?: PushOptions | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<void>;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">refspecs</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string[]</span>
    <br>
    <p class="param-description">리모트로 푸시할 refspec 목록이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | PushOptions</span>
    <br>
    <p class="param-description">푸시 작업을 위한 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">credential</span><span class="param-type">Credential</span>
        <br>
        <p class="param-description">Git 인증 정보를 나타내는 인터페이스예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">customHeaders</span><span class="param-type">string[]</span>
        <br>
        <p class="param-description">이 푸시 작업에 추가할 HTTP 헤더 목록이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">followRedirects</span><span class="param-type">RemoteRedirect</span>
        <br>
        <p class="param-description">리모트 리디렉션(다른 URL로 이동) 허용 여부를 설정해요.</p>
        <p class="param-description">
          - <code>None</code> : 어떤 경우에도 리디렉션을 따르지 않음<br>
          - <code>Initial</code> : 초기 요청(<code>/info/refs</code>)에서만 리디렉션 허용 (기본값)<br>
          - <code>All</code> : 모든 요청에서 리디렉션 허용
        </p>
      </li>
      <li class="param-li">
        <span class="param-name">pbParallelism</span><span class="param-type">number</span>
        <br>
        <p class="param-description">
          푸시할 데이터를 패킹할 때 사용할 병렬 작업 수를 설정해요.  
          0이면 자동으로 적절한 개수를 선택하며, 기본값은 1이에요.
        </p>
      </li>
      <li class="param-li">
        <span class="param-name">proxy</span><span class="param-type">ProxyOptions</span>
        <br>
        <p class="param-description">푸시 작업에서 사용할 프록시 설정이에요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">auto</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">
              Git 설정에서 자동으로 프록시를 감지할지 여부를 설정해요.  
              이 옵션을 사용하면 <code>url</code> 설정을 덮어써요.
            </p>
          </li>
          <li class="param-li">
            <span class="param-name">url</span><span class="param-type">string</span>
            <br>
            <p class="param-description">
              사용할 프록시의 URL을 명시적으로 지정해요.  
              이 옵션을 사용하면 <code>auto</code> 설정을 덮어써요.
            </p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">remoteOptions</span><span class="param-type">string[]</span>
        <br>
        <p class="param-description">리모트에 전달할 추가 푸시 옵션이에요.</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">요청을 중단할 때 사용할 <code>AbortSignal</code> 객체예요.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const remote = repo.getRemote('origin');

// 로컬 "main" 브랜치를 리모트 "other" 브랜치로 푸시해요.
await remote.push(['refs/heads/main:refs/heads/other']);

// 인증 정보를 포함해서 푸시해요.
await remote.push(['refs/heads/main:refs/heads/other'], {
  credential: {
    type: 'Plain',
    password: '<personal access token>',
  },
});
```
