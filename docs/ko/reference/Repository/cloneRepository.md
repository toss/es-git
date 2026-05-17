# cloneRepository

원격 리포지토리를 로컬로 클론해요.

이 메서드는 지정된 옵션을 사용해 특정 URL의 리포지토리를 지정된 로컬 경로로 클론해요.

## 시그니처

```ts
function cloneRepository(
  url: string,
  path: string,
  options?: RepositoryCloneOptions | null | undefined,
  signal?: AbortSignal | null | undefined
): Promise<Repository>;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">url</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">
      클론할 원격 리포지토리의 URL이에요.
    </p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">
      리포지토리를 클론할 로컬 경로예요.
    </p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | RepositoryCloneOptions</span>
    <br>
    <p class="param-description">
      클론 옵션이에요.
    </p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">bare</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">
          리포지토리를 <code>bare</code> 모드로 클론할지 설정해요.  
          <code>true</code>로 설정하면 워킹 디렉터리 없이 개체 저장소만 클론돼요.
        </p>
      </li>
      <li class="param-li">
        <span class="param-name">branch</span><span class="param-type">string</span>
        <br>
        <p class="param-description">
          클론 후 체크아웃할 브랜치 이름이에요.  
          지정하지 않으면 원격 리포지토리의 기본 브랜치가 사용돼요.
        </p>
      </li>
      <li class="param-li">
        <span class="param-name">fetch</span><span class="param-type">FetchOptions</span>
        <br>
        <p class="param-description">
          <code>fetch</code> 작업에 사용할 옵션이에요.
        </p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">callbacks</span><span class="param-type">RemoteCallbacks</span>
            <br>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">packProgress</span><span class="param-type">(stage: PackBuilderStage, current: number, total: number) =&gt; void</span>
                <br>
                <p class="param-description">팩 빌드 중 진행 정보를 전달하기 위해 호출되는 함수예요. 이 함수는 팩 빌드 작업과 인라인으로 호출되므로 성능에 영향을 줄 수 있어요.</p>
              </li>
              <li class="param-li">
                <span class="param-name">pushNegotiation</span><span class="param-type">(update: PushUpdate[]) =&gt; void</span>
                <br>
                <p class="param-description">협상 단계와 업로드 사이에 한 번 호출되는 콜백이에요. 콜백의 인수는 목적지에 명령으로 전송될 업데이트를 포함하는 슬라이스예요.</p>
              </li>
              <li class="param-li">
                <span class="param-name">pushTransferProgress</span><span class="param-type">(current: number, total: number, bytes: number) =&gt; void</span>
                <br>
                <p class="param-description">push 전송 진행 상황을 모니터링하는 콜백.</p>
              </li>
              <li class="param-li">
                <span class="param-name">pushUpdateReference</span><span class="param-type">(refname: string, status: string | null) =&gt; void</span>
                <br>
                <p class="param-description">push 시 업데이트된 각 참조에 대해 호출되는 콜백을 설정해요. 콜백의 첫 번째 인수는 참조의 이름이고 두 번째는 서버에서 보낸 상태 메시지예요. 상태가 <code>null</code>이 아니면 push가 거부된 거예요.</p>
              </li>
              <li class="param-li">
                <span class="param-name">sidebandProgress</span><span class="param-type">(data: Uint8Array) =&gt; void</span>
                <br>
                <p class="param-description">원격의 텍스트 진행 상황이에요. 진행 사이드 밴드를 통해 전송된 텍스트가 이 함수로 전달돼요 (이것은 &#39;counting objects&#39; 출력이에요).</p>
              </li>
              <li class="param-li">
                <span class="param-name">transferProgress</span><span class="param-type">(data: RemoteTransferProgress) =&gt; void</span>
                <br>
                <p class="param-description">fetch 중 전송 진행 상황과 함께 호출돼요.</p>
              </li>
              <li class="param-li">
                <span class="param-name">updateTips</span><span class="param-type">(refname: string, oldId: string, newId: string) =&gt; void</span>
                <br>
                <p class="param-description">참조가 로컬에서 업데이트될 때마다 관련 정보와 함께 콜백이 호출돼요.</p>
              </li>
            </ul>
          </li>
          <li class="param-li">
            <span class="param-name">credential</span><span class="param-type">Credential</span>
            <br>
            <p class="param-description">
              인증 정보를 설정해요.
            </p>
          </li>
          <li class="param-li">
            <span class="param-name">customHeaders</span><span class="param-type">string[]</span>
            <br>
            <p class="param-description">
              <code>fetch</code> 요청에 추가할 HTTP 헤더 목록이에요.
            </p>
          </li>
          <li class="param-li">
            <span class="param-name">depth</span><span class="param-type">number</span>
            <br>
            <p class="param-description">클론할 커밋 깊이를 설정해요. 0 이하의 값이면 전체 이력을 클론해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">downloadTags</span><span class="param-type">AutotagOption</span>
            <br>
            <p class="param-description">
              태그 다운로드 방식을 설정해요.
            </p>
            <p class="param-description">
              - <code>Unspecified</code> : 원격 리포지토리의 설정을 따름<br>
              - <code>Auto</code> : 다운로드하는 개체와 연결된 태그만 가져옴 (기본값)<br>
              - <code>None</code> : 태그를 다운로드하지 않음<br>
              - <code>All</code> : 모든 태그를 다운로드함
            </p>
          </li>
<li class="param-li">
  <span class="param-name">followRedirects</span><span class="param-type">RemoteRedirect</span>
  <br>
  <p class="param-description">
    원격 리디렉션 설정을 지정해요.  
    다른 호스트로의 리디렉션을 허용할지 여부를 설정할 수 있어요.  
    기본적으로 Git은 첫 번째 요청(<code>/info/refs</code>)에서는 리디렉션을 따르지만,  
    이후 요청에서는 리디렉션을 따르지 않아요.
  </p>
  <p class="param-description">
    - <code>None</code> : <code>fetch</code> 또는 <code>push</code> 과정에서 어떤 리디렉션도 따르지 않음<br>
    - <code>Initial</code> : 첫 번째 요청에서만 리디렉션을 허용 (기본값)<br>
    - <code>All</code> : <code>fetch</code> 또는 <code>push</code> 과정 중 언제든지 리디렉션을 허용
  </p>
</li>
<li class="param-li">
  <span class="param-name">proxy</span><span class="param-type">ProxyOptions</span>
  <br>
  <p class="param-description">
    `fetch` 작업에 사용할 프록시(proxy) 설정이에요.
  </p>
  <ul class="param-ul">
    <li class="param-li">
      <span class="param-name">auto</span><span class="param-type">boolean</span>
      <br>
      <p class="param-description">
        Git 설정에서 프록시를 자동으로 감지할지 여부를 설정해요.  
        활성화하면 이전에 설정한 <code>url</code> 값을 덮어써요.
      </p>
    </li>
    <li class="param-li">
      <span class="param-name">url</span><span class="param-type">string</span>
      <br>
      <p class="param-description">
        사용할 프록시의 정확한 URL을 지정해요.  
        설정하면 이전에 활성화한 <code>auto</code> 옵션을 덮어써요.
      </p>
    </li>
  </ul>
</li>
          <li class="param-li">
            <span class="param-name">prune</span><span class="param-type">FetchPrune</span>
            <br>
            <p class="param-description">
              <code>fetch</code> 후 불필요한 원격 브랜치를 제거할지 설정해요.
            </p>
            <p class="param-description">
              - <code>Unspecified</code> : 설정을 따름<br>
              - <code>On</code> : 자동으로 삭제<br>
              - <code>Off</code> : 삭제하지 않음
            </p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">recursive</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">
          서브모듈을 재귀적으로 초기화하고 업데이트할지 설정해요.  
          <code>git clone --recursive</code>와 같은 동작을 해요.
        </p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">
      클론 작업을 취소할 수 있도록 <code>AbortSignal</code>을 설정할 수 있어요.
    </p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Promise&lt;Repository&gt;</span>
    <br>
    <p class="param-description">클론된 리포지토리를 반환해요.</p>
  </li>
</ul>

## 예제

`https://` 프로토콜을 사용해 리포지토리 클론해요.

```ts
import { cloneRepository } from 'es-git';

const repo = await cloneRepository(
  'https://github.com/toss/es-git',
  '/path/to/clone',
);
```

`git://` 프로토콜을 사용해 리포지토리 클론해요.

```ts
import { cloneRepository } from 'es-git';

const repo = await cloneRepository(
  'git@github.com:toss/es-git',
  '/path/to/clone',
);
```

인증 정보를 사용해 리포지토리 클론해요.

```ts
import { cloneRepository } from 'es-git';

// SSH 에이전트를 사용해 인증
const repo = await cloneRepository('git@github.com:toss/es-git', '.', {
  fetch: {
    credential: {
      type: 'SSHKeyFromAgent',
    },
  },
});
```
