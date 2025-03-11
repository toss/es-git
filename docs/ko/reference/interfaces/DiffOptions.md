[es-git](../globals.md) / DiffOptions

# 인터페이스: DiffOptions

`diff`를 실행하는 방법을 설정하는 옵션이에요.

## 속성

| 속성                                                                        | 유형         | 설명                                                                                                              |
|---------------------------------------------------------------------------|------------|-----------------------------------------------------------------------------------------------------------------|
| <a id="reverse"></a> `reverse?`                                           | `boolean`  | `diff`의 비교 방향을 반대로 설정할지 여부를 나타내는 값이에요.                                                                          |
| <a id="includeignored"></a> `includeIgnored?`                             | `boolean`  | 무시된 파일을 `diff`에 포함할지 여부를 설정해요.                                                                                  |
| <a id="recurseignoreddirs"></a> `recurseIgnoredDirs?`                     | `boolean`  | 무시된 디렉터리를 재귀적으로 탐색할지 여부를 설정해요.                                                                                  |
| <a id="includeuntracked"></a> `includeUntracked?`                         | `boolean`  | 추적되지 않은 파일을 `diff`에 포함할지 여부를 설정해요.                                                                              |
| <a id="recurseuntrackeddirs"></a> `recurseUntrackedDirs?`                 | `boolean`  | 추적되지 않은 디렉터리를 재귀적으로 탐색할지 여부를 설정해요.                                                                              |
| <a id="includeunmodified"></a> `includeUnmodified?`                       | `boolean`  | 수정되지 않은 파일을 `diff`에 포함할지 여부를 설정해요.                                                                              |
| <a id="includetypechange"></a> `includeTypechange?`                       | `boolean`  | 이 옵션을 활성화하면 타입 변경(`Typechange`) 차이를 기록해요.                                                                       |
| <a id="includetypechangetrees"></a> `includeTypechangeTrees?`             | `boolean`  | `includeTypechange` 옵션과 함께 사용하면 삭제된 블롭(blob) 대신 트리 변경을 올바르게 기록해요. 다만, 트리 SHA는 제공되지 않아요.                         |
| <a id="ignorefilemode"></a> `ignoreFilemode?`                             | `boolean`  | 파일 권한 변경을 무시할지 여부를 설정해요.                                                                                        |
| <a id="ignoresubmodules"></a> `ignoreSubmodules?`                         | `boolean`  | 모든 서브모듈을 변경되지 않은 것으로 처리할지 여부를 설정해요.                                                                             |
| <a id="ignorecase"></a> `ignoreCase?`                                     | `boolean`  | 파일 이름을 대소문자를 구분하지 않고 비교할지 여부를 설정해요.                                                                             |
| <a id="disablepathspecmatch"></a> `disablePathspecMatch?`                 | `boolean`  | `pathspecs`가 지정된 경우, 패턴 일치가 아닌 정확한 경로 일치 방식으로 적용할지 설정해요.                                                        |
| <a id="skipbinarycheck"></a> `skipBinaryCheck?`                           | `boolean`  | `binary` 플래그 업데이트를 비활성화해요. 파일을 완전히 로드할 필요가 없을 때 유용해요.                                                           |
| <a id="enablefastuntrackeddirs"></a> `enableFastUntrackedDirs?`           | `boolean`  | 기본적으로 Git은 추적되지 않은 디렉터리를 스캔해 포함 여부를 결정해요. 이 옵션을 활성화하면 스캔을 생략하고 즉시 디렉터리를 추적되지 않은 것으로 표시해요.                       |
| <a id="updateindex"></a> `updateIndex?`                                   | `boolean`  | 워킹 디렉터리의 파일이 `index`와 다르지만 OID가 같을 경우, 올바른 `stat` 정보를 `index`에 기록해요. 기본적으로 `diff`는 `index`를 수정하지 않아요.           |
| <a id="includeunreadable"></a> `includeUnreadable?`                       | `boolean`  | 읽을 수 없는 파일을 `diff`에 포함할지 여부를 설정해요.                                                                              |
| <a id="includeunreadableasuntracked"></a> `includeUnreadableAsUntracked?` | `boolean`  | 읽을 수 없는 파일을 추적되지 않은 파일로 간주할지 설정해요.                                                                              |
| <a id="forcetext"></a> `forceText?`                                       | `boolean`  | 모든 파일을 텍스트로 처리하고 바이너리 속성과 감지를 비활성화해요.                                                                           |
| <a id="forcebinary"></a> `forceBinary?`                                   | `boolean`  | 모든 파일을 바이너리로 처리하고 텍스트 비교를 비활성화해요.                                                                               |
| <a id="ignorewhitespace"></a> `ignoreWhitespace?`                         | `boolean`  | 모든 공백 변경을 무시할지 여부를 설정해요.                                                                                        |
| <a id="ignorewhitespacechange"></a> `ignoreWhitespaceChange?`             | `boolean`  | 공백 수량 변경을 무시할지 여부를 설정해요.                                                                                        |
| <a id="ignorewhitespaceeol"></a> `ignoreWhitespaceEol?`                   | `boolean`  | 줄 끝(`EOL`)의 공백 변경을 무시할지 여부를 설정해요.                                                                               |
| <a id="ignoreblanklines"></a> `ignoreBlankLines?`                         | `boolean`  | 빈 줄을 무시할지 여부를 설정해요.                                                                                             |
| <a id="showuntrackedcontent"></a> `showUntrackedContent?`                 | `boolean`  | 패치(`patch`) 생성 시 추적되지 않은 파일의 내용을 포함할지 설정해요. `includeUntracked`를 자동으로 활성화하지만, `recurseUntrackedDirs`는 활성화하지 않아요. |
| <a id="showunmodified"></a> `showUnmodified?`                             | `boolean`  | 출력에서 수정되지 않은 파일 이름을 포함할지 설정해요. 기본적으로 `Diff`의 일부 형식에서는 이를 생략해요.                                                  |
| <a id="patience"></a> `patience?`                                         | `boolean`  | "Patience diff" 알고리즘을 사용할지 설정해요.                                                                                |
| <a id="minimal"></a> `minimal?`                                           | `boolean`  | 최소한의 차이를 찾도록 추가 시간을 들일지 여부를 설정해요.                                                                               |
| <a id="showbinary"></a> `showBinary?`                                     | `boolean`  | `git-apply`가 바이너리 파일 변경을 적용할 수 있도록 필요한 정보를 포함할지 설정해요.                                                           |
| <a id="indentheuristic"></a> `indentHeuristic?`                           | `boolean`  | 들여쓰기와 공백을 고려하는 휴리스틱 알고리즘을 사용해 더 나은 `diff` 결과를 생성할지 설정해요.                                                        |
| <a id="contextlines"></a> `contextLines?`                                 | `number`   | 변경 부분 앞뒤로 표시할 변경되지 않은 줄 수를 설정해요. 기본값은 `3`이에요.                                                                   |
| <a id="interhunklines"></a> `interhunkLines?`                             | `number`   | 별도 변경 블록을 합칠 기준이 되는 최대 변경되지 않은 줄 수를 설정해요. 기본값은 `0`이에요.                                                          |
| <a id="idabbrev"></a> `idAbbrev?`                                         | `number`   | `core.abbrev` 값 또는 기본값 `7`로 설정해요.                                                                               |
| <a id="maxsize"></a> `maxSize?`                                           | `number`   | 지정한 크기(바이트 단위)를 초과하는 블롭(blob)을 자동으로 바이너리로 간주해요. 음수를 지정하면 이 기능을 비활성화할 수 있어요. 기본값은 `512MB`예요.                     |
| <a id="oldprefix"></a> `oldPrefix?`                                       | `string`   | `diff`의 기존 파일 이름 앞에 붙일 가상 "디렉터리"를 설정해요. 기본값은 `"a"`예요.                                                           |
| <a id="newprefix"></a> `newPrefix?`                                       | `string`   | `diff`의 새로운 파일 이름 앞에 붙일 가상 "디렉터리"를 설정해요. 기본값은 `"b"`예요.                                                          |
| <a id="pathspecs"></a> `pathspecs?`                                       | `string`[] | `diff`에 포함할 경로나 패턴 목록을 지정해요.                                                                                    |
