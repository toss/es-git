[es-git](../globals.md) / IndexEntry

# Interface: IndexEntry

## Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="ctime"></a> `ctime` | `Date` | - |
| <a id="mtime"></a> `mtime` | `Date` | - |
| <a id="dev"></a> `dev` | `number` | - |
| <a id="ino"></a> `ino` | `number` | - |
| <a id="mode"></a> `mode` | `number` | - |
| <a id="uid"></a> `uid` | `number` | - |
| <a id="gid"></a> `gid` | `number` | - |
| <a id="filesize"></a> `fileSize` | `number` | - |
| <a id="id"></a> `id` | `string` | - |
| <a id="flags"></a> `flags` | `number` | - |
| <a id="flagsextended"></a> `flagsExtended` | `number` | - |
| <a id="path"></a> `path` | `Buffer` | The path of this index entry as a byte vector. Regardless of the current platform, the directory separator is an ASCII forward slash (`0x2F`). There are no terminating or internal NUL characters, and no trailing slashes. Most of the time, paths will be valid utf-8 — but not always. For more information on the path storage format, see [these git docs][git-index-docs]. Note that libgit2 will take care of handling the prefix compression mentioned there. [git-index-docs]: https://github.com/git/git/blob/a08a83db2bf27f015bec9a435f6d73e223c21c5e/Documentation/technical/index-format.txt#L107-L124 |
