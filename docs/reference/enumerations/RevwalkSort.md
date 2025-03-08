[es-git](../globals.md) / RevwalkSort

# Enumeration: RevwalkSort

Orderings that may be specified for Revwalk iteration.

## Enumeration Members

| Enumeration Member | Value | Description |
| ------ | ------ | ------ |
| <a id="none"></a> `None` | `0` | Sort the repository contents in no particular ordering. This sorting is arbitrary, implementation-specific, and subject to change at any time. This is the default sorting for new walkers. |
| <a id="topological"></a> `Topological` | `1` | Sort the repository contents in topological order (children before parents). This sorting mode can be combined with time sorting. |
| <a id="time"></a> `Time` | `2` | Sort the repository contents by commit time. This sorting mode can be combined with topological sorting. |
| <a id="reverse"></a> `Reverse` | `4` | Iterate through the repository contents in reverse order. This sorting mode can be combined with any others. |
