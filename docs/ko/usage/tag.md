# 태그

## 태그 조회하기

리포지토리에 존재하는 모든 태그의 이름을 불러오기 위해서 [`tagNames`](../api/classes/Repository.md#tagnames)을 사용할 수 있어요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const tagNames = repo.tagNames();
console.log(tagNames); // ["v0", "v1", "v2"] 
```

태그명과 함께 태그 해시를 획득하기 위해서 [`tagForeach`](../api/classes/Repository.md#tagforeach)로 모든 태그를 순회할 수 있어요.

```ts
const tags = [];
repo.tagForeach((sha, name) => {
  tags.push([name, sha]);
  return true;
});
console.log(tags);
// [['aa0040546ed22b8bb33f3bd621e8d10ed849b02c', 'refs/tags/v0'],
//  ['674e3327707fcf32a348ecfc0cb6b93e57398b8c', 'refs/tags/v1'],
//  ['567aa5c6b219312dc7758ab88ebb7a1e5d36d26b', 'refs/tags/v2']]

// 순회를 멈추고 싶다면 콜백에서 false를 반환하세요.
const v0to1Tags = [];
repo.tagForeach((sha, name) => {
  v0to1Tags.push([name, sha]);
  return name !== 'refs/tags/v1';
});
console.log(v0to1Tags);
// [['aa0040546ed22b8bb33f3bd621e8d10ed849b02c', 'refs/tags/v0'],
//  ['674e3327707fcf32a348ecfc0cb6b93e57398b8c', 'refs/tags/v1']]
```

이제 해시값으로 [`getTag`](../api/classes/Repository.md#gettag)을 사용해 태그 정보를 불러올 수 있어요.

```ts
const tag = repo.getTag('aa00405');
console.log(tag.id()); // "aa0040546ed22b8bb33f3bd621e8d10ed849b02c"
console.log(tag.name()); // "v0"
console.log(tag.message()); // "message for this tag"

// 태그가 가리키는 커밋 불러오기
const commit = tag.target().peelToCommit();
console.log(commit.id()); // "828954df9f08dc8e172447cdacf0ddea1adf9e63"
```

## 태그 만들기

[`createTag`](../api/classes/Repository.md#createtag)를 사용해 특정 커밋에 태그를 생성할 수 있어요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const commit = repo.getCommit('828954df9f08dc8e172447cdacf0ddea1adf9e63');

const tagSha = repo.createTag('mytag', commit.asObject(), 'this is my tag', {
  tagger: {
    name: 'Seokju Na',
    email: 'seokju.me@toss.im'
  },
});
const tag = repo.getTag(tagSha);
console.log(tag.name()); // "mytag"
console.log(tag.target().id()); // "828954df9f08dc8e172447cdacf0ddea1adf9e63"
```

만약 이미 동일한 이름의 태그가 존재하는 경우 오류가 발생해요. 기존 태그를 새로운 태그로 대체하고 싶다면 `force` 옵션을 사용해요.

```ts
const tagSha = repo.createTag('mytag', commit.asObject(), 'this is my tag', {
  // 기존 태그가 새롭게 만들어지는 태그로 대체돼요.
  force: true,
});
```
