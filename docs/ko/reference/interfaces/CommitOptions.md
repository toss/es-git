[es-git](../globals.md) / CommitOptions

# 인터페이스: CommitOptions

## 속성

| 속성                                  | 유형                                        | 설명                                                                         |
|-------------------------------------|-------------------------------------------|----------------------------------------------------------------------------|
| <a id="updateref"></a> `updateRef?` | `string`                                  | -                                                                          |
| <a id="author"></a> `author?`       | [`SignaturePayload`](SignaturePayload.md) | 저자 서명. 만약 입력되지 않으면 이 리포지토리의 기본 서명을 사용해요. 기본 서명이 설정되지 않은 리포지토리라면 오류가 발생해요.  |
| <a id="committer"></a> `committer?` | [`SignaturePayload`](SignaturePayload.md) | 커미터 서명. 만약 입력되지 않으면 이 리포지토리의 기본 서명을 사용해요. 기본 서명이 설정되지 않은 리포지토리라면 오류가 발생해요. |
| <a id="parents"></a> `parents?`     | `string`[]                                | -                                                                          |
