# createSignature

새로운 작업 서명(Signature)을 생성해요.

## 시그니처

```ts
function createSignature(
  name: string,
  email: string,
  timeOptions?: SignatureTimeOptions | null | undefined,
): Signature;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">서명에 사용할 이름이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">서명에 사용할 이메일이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">timeOptions</span><span class="param-type">null | SignatureTimeOptions</span>
    <br>
    <p class="param-description">서명에 사용할 시간 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">offset</span><span class="param-type">number</span>
        <br>
        <p class="param-description">시간대 오프셋(offset) 값이에요. (분 단위)</p>
      </li>
      <li class="param-li">
        <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">1970년 1월 1일(유닉스 에폭)으로부터 지나온 시간이에요. (초 단위)</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Signature</span>
    <br>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">서명에 사용된 이메일이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">서명에 사용된 이름이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">1970년 1월 1일(유닉스 에폭)으로부터 지나온 시간이에요. (초 단위)</p>
      </li>
    </ul>
  </li>
</ul>


## 예제

```ts
import { createSignature } from 'es-git';

const author = createSignature('Seokju Na', 'seokju.me@toss.im');
```
