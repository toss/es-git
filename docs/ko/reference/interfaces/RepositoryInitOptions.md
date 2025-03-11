[es-git](../globals.md) / RepositoryInitOptions

# 인터페이스: RepositoryInitOptions

## 속성

| 속성                                                | 유형        | 설명                                                                                                                                                               |
|---------------------------------------------------|-----------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <a id="bare"></a> `bare?`                         | `boolean` | 작업 디렉터리가 없는 `bare` 저장소를 생성할지 설정해요. 기본값은 `false`예요.                                                                                                               |
| <a id="noreinit"></a> `noReinit?`                 | `boolean` | 지정한 경로에 이미 Git 저장소가 존재하는 경우 에러를 반환할지 설정해요. 기본값은 `false`예요.                                                                                                       |
| <a id="nodotgitdir"></a> `noDotgitDir?`           | `boolean` | 기본적으로 `bare` 저장소가 아닌 경우 저장소 경로에 `/.git/`을 자동으로 추가해요. 이 옵션을 설정하면 해당 동작을 방지할 수 있어요. 기본값은 `false`예요.                                                                |
| <a id="mkdir"></a> `mkdir?`                       | `boolean` | 필요한 경우 저장소 경로(및 작업 디렉터리 경로)를 생성할지 설정해요. 단, 이 값과 관계없이 `.git` 디렉터리는 항상 생성돼요. 기본값은 `true`예요.                                                                        |
| <a id="mkpath"></a> `mkpath?`                     | `boolean` | `mkdir`과 동일한 역할을 해요. 필요한 경우 저장소 경로(및 작업 디렉터리 경로)를 생성해요. 기본값은 `true`예요.                                                                                           |
| <a id="mode"></a> `mode?`                         | `number`  | `RepositoryInit` 상수 또는 사용자 정의 값을 설정할 수 있어요.                                                                                                                      |
| <a id="externaltemplate"></a> `externalTemplate?` | `boolean` | 외부 템플릿을 사용할지 여부를 설정해요. 활성화하면 `templatePath` 옵션을 먼저 확인하고, 설정되지 않았다면 글로벌 설정의 `init.templatedir`, 마지막으로 `/usr/share/git-core-templates`를 검색해요(존재하는 경우). 기본값은 `true`예요. |
| <a id="templatepath"></a> `templatePath?`         | `string`  | `externalTemplate` 옵션이 활성화된 경우, 템플릿 디렉터리를 먼저 찾을 경로를 지정해요. 이 값이 설정되지 않으면 기본 경로를 검색해요.                                                                             |
| <a id="workdirpath"></a> `workdirPath?`           | `string`  | 작업 디렉터리의 경로를 지정해요. 상대 경로를 입력하면 저장소 경로를 기준으로 해석돼요. 이 경로가 "기본" 작업 디렉터리가 아닌 경우, `.git` 내부에 `gitlink` 파일이 생성되어 저장소 경로를 참조해요.                                         |
| <a id="description"></a> `description?`           | `string`  | 저장소의 `description` 파일을 초기화할 때 사용할 내용을 설정해요. 설정하지 않으면 템플릿 내용을 사용해요.                                                                                               |
| <a id="initialhead"></a> `initialHead?`           | `string`  | `HEAD`가 가리킬 초기 브랜치 이름을 설정해요. 설정하지 않으면 Git 구성에서 해당 값을 가져와요. `refs/`로 시작하면 그대로 사용하고, 그렇지 않으면 `refs/heads/`가 자동으로 추가돼요.                                             |
| <a id="originurl"></a> `originUrl?`               | `string`  | 저장소 초기화가 완료된 후, `origin` 원격(remote) 저장소를 이 URL로 설정할지 지정해요.                                                                                                       |
