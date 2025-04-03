# fetch

새 데이터를 다운로드하고 리모트 추적 브랜치를 업데이트해요.

이 메서드는 리모트에 연결하고 데이터를 다운로드한 후, 연결을 종료하고  
리모트 추적 브랜치를 업데이트하는 편리한 기능을 제공해요.

## 시그니처

```ts
class Remote {
  fetch(
    refspecs: string[],
    options?: FetchRemoteOptions | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<void>;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">refspecs</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string[]</span>
    <br>
    <p class="param-description">리모트에서 가져올 레퍼런스 사양(refspecs)이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | FetchRemoteOptions</span>
    <br>
    <p class="param-description">리모트에서 데이터를 가져올 때 사용할 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">fetch</span><span class="param-type">FetchOptions</span>
        <br>
        <p class="param-description">다양한 fetch 작업을 위한 옵션이에요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">credential</span><span class="param-type">Credential</span>
            <br>
            <p class="param-description">Git 인증 정보를 나타내는 인터페이스예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">customHeaders</span><span class="param-type">string[]</span>
            <br>
            <p class="param-description">이 fetch 작업에 추가할 HTTP 헤더 목록이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">depth</span><span class="param-type">number</span>
            <br>
            <p class="param-description">
              가져올 커밋의 깊이를 설정해요.  
              0 이하의 값이면 모든 커밋을 가져와요(제한 없음).
            </p>
          </li>
          <li class="param-li">
            <span class="param-name">downloadTags</span><span class="param-type">AutotagOption</span>
            <br>
            <p class="param-description">태그 다운로드 방식을 설정해요.</p>
            <p class="param-description">
              - <code>Unspecified</code> : 리모트 설정을 따름<br>
              - <code>Auto</code> : 다운로드하는 개체에 연결된 태그만 가져옴 (기본값)<br>
              - <code>None</code> : refspec에서 지정된 태그 외에는 가져오지 않음<br>
              - <code>All</code> : 모든 태그를 가져옴
            </p>
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
            <span class="param-name">proxy</span><span class="param-type">ProxyOptions</span>
            <br>
            <p class="param-description">fetch 작업에서 사용할 프록시 설정이에요.</p>
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
            <span class="param-name">prune</span><span class="param-type">FetchPrune</span>
            <br>
            <p class="param-description">fetch 이후 불필요한 브랜치를 정리(prune)할지 설정해요.</p>
            <p class="param-description">
              - <code>Unspecified</code> : 설정값을 따름<br>
              - <code>On</code> : 정리를 강제 수행<br>
              - <code>Off</code> : 정리를 강제 비활성화
            </p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">reflogMsg</span><span class="param-type">string</span>
        <br>
        <p class="param-description">reflog에 남길 메시지예요.</p>
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

// "main" 브랜치 데이터를 가져와요.
await remote.fetch(['main']);

// 빈 배열을 전달하면, 리모트에 설정된 기본 refspec을 사용해 데이터를 가져와요.
await remote.fetch([]);
```
