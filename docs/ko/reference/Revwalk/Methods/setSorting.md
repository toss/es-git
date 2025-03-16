# setSorting

커밋을 방문할 순서를 설정해요.

## 시그니처

```ts
class Revwalk {
  setSorting(sort: number): this;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">sort</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">Revwalk 반복(iteration) 과정에서 사용할 정렬 방식을 지정해요
<br>
- <code>RevwalkSort.None</code> : 리포지토리 내의 내용을 특별한 정렬 없이 방문해요. 이 정렬 방식은 임의적이며, 구현에 따라 다르고 언제든지 변경될 수 있어요. 새로운 Revwalk의 기본값이에요.
<br>
- <code>RevwalkSort.Topological</code> : 리포지토리 내용을 위상(topological) 순서로 정렬해요 (자식이 부모보다 먼저 나와요). 이 정렬 방식은 시간 순서와 함께 사용할 수 있어요.
<br>
- <code>RevwalkSort.Time</code> : 리포지토리 내용을 커밋 시간 순서로 정렬해요. 이 정렬 방식은 위상 순서 정렬과 함께 사용할 수 있어요.
<br>
- <code>RevwalkSort.Reverse</code> : 리포지토리 내용을 역순으로 반복해요. 이 정렬 방식은 다른 정렬 방식들과 함께 사용할 수 있어요.
<br>
</p>
  </li>
</ul>

## 예제

```ts
import { openRepository, RevwalkSort } from 'es-git';

const repo = await openRepository('.');
const revwalk = repo.revwalk();
revwalk.setSorting(RevwalkSort.Time | RevwalkSort.Reverse);
```