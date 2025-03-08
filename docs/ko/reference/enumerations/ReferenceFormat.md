[es-git](../globals.md) / ReferenceFormat

# 열거형: ReferenceFormat

Options for normalize reference name.

## 포함된 값

| 포함된 값 | 값 | 설명 |
| ------ | ------ | ------ |
| <a id="normal"></a> `Normal` | `0` | No particular normalization. |
| <a id="allowonelevel"></a> `AllowOnelevel` | `1` | Control whether one-level refname are accepted (i.e., refnames that do not contain multiple `/`-separated components). Those are expected to be written only using uppercase letters and underscore (e.g. `HEAD`, `FETCH_HEAD`). |
| <a id="refspecpattern"></a> `RefspecPattern` | `2` | Interpret the provided name as a reference pattern for a refspec (as used with remote repositories). If this option is enabled, the name is allowed to contain a single `*` in place of a full pathname components (e.g., `foo/*/bar` but not `foo/bar*`). |
| <a id="refspecshorthand"></a> `RefspecShorthand` | `4` | Interpret the name as part of a refspec in shorthand form so the `AllowOnelevel` naming rules aren't enforced and `main` becomes a valid name. |
