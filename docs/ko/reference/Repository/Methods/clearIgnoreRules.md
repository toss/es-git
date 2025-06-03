# clearIgnoreRules

명시적으로 추가된 무시 규칙을 지워요.

기본 내부 무시 규칙으로 재설정해요.
이는 파일 시스템에 실제로 존재하는 .gitignore 파일의 규칙은 끄지 않아요.
기본 내부 무시 규칙은 ".", ".." 및 ".git" 항목을 무시해요.

## 시그니처

```ts
class Repository {
  clearIgnoreRules(): void;
}
```

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
repo.addIgnoreRule("*.log");
// 나중에 추가된 모든 규칙을 지워요
repo.clearIgnoreRules();
```