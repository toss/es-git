[es-git](../globals.md) / DiffFlags

# 열거형: DiffFlags

## 포함된 값

| 포함된 값 | 값 | 설명 |
| ------ | ------ | ------ |
| <a id="binary"></a> `Binary` | `1` | File(s) treated as binary data. (1 << 0) |
| <a id="notbinary"></a> `NotBinary` | `2` | File(s) treated as text data. (1 << 1) |
| <a id="validid"></a> `ValidId` | `4` | `id` value is known correct. (1 << 2) |
| <a id="exists"></a> `Exists` | `8` | File exists at this side of the delta. (1 << 3) |
