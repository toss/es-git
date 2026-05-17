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
        <span class="param-name">callbacks</span><span class="param-type">RemoteCallbacks</span>
        <br>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">packProgress</span><span class="param-type">(stage: PackBuilderStage, current: number, total: number) =&gt; void</span>
            <br>
            <p class="param-description">팩 빌드 중 진행 정보를 전달하기 위해 호출할 함수예요. 팩 빌드 작업과 인라인으로 호출되므로 성능에 영향을 줄 수 있어요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">pushNegotiation</span><span class="param-type">(update: PushUpdate[]) =&gt; void</span>
            <br>
            <p class="param-description">협상 단계와 업로드 사이에 한 번 호출되는 콜백이에요. 콜백에 전달되는 인수는 대상에 명령으로 전송될 업데이트를 포함하는 슬라이스예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">pushTransferProgress</span><span class="param-type">(current: number, total: number, bytes: number) =&gt; void</span>
            <br>
            <p class="param-description">푸시 전송 진행 상황을 모니터링하는 콜백.</p>
          </li>
          <li class="param-li">
            <span class="param-name">pushUpdateReference</span><span class="param-type">(refname: string, status: string | null) =&gt; void</span>
            <br>
            <p class="param-description">푸시 시 업데이트된 각 참조에 대해 호출되는 콜백을 설정해요. 콜백의 첫 번째 인수는 참조 이름이고, 두 번째는 서버가 보낸 상태 메시지예요. 상태가 <code>null</code>이 아니면 푸시가 거부된 거예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">sidebandProgress</span><span class="param-type">(data: Uint8Array) =&gt; void</span>
            <br>
            <p class="param-description">원격으로부터의 텍스트 진행 정보예요. 진행 사이드 밴드를 통해 전송된 텍스트가 이 함수에 전달돼요 (이는 &#39;counting objects&#39; 출력이에요).</p>
          </li>
          <li class="param-li">
            <span class="param-name">transferProgress</span><span class="param-type">(data: RemoteTransferProgress) =&gt; void</span>
            <br>
            <p class="param-description">페치 중 전송 진행 상황과 함께 호출돼요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateTips</span><span class="param-type">(refname: string, oldId: string, newId: string) =&gt; void</span>
            <br>
            <p class="param-description">로컬에서 참조가 업데이트될 때마다 해당 정보와 함께 콜백이 호출돼요.</p>
          </li>
        </ul>
      </li>
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
