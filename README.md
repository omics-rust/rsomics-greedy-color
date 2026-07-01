# rsomics-greedy-color

Greedy graph coloring CLI — a value-exact port of `networkx.greedy_color`.

Reads an undirected edge list from stdin (`u v` per line, `#` comments and
blank lines skipped, string nodes, parallel edges deduplicated). Assigns
each node a non-negative integer color such that no two adjacent nodes share
a color. Output: `node<TAB>color` sorted lexicographically by node name.

## Usage

```
echo -e "0 1\n1 2\n2 3\n3 4" | rsomics-greedy-color
echo -e "0 1\n1 2\n2 3\n3 4" | rsomics-greedy-color --strategy saturation_largest_first
echo -e "0 1\n1 2" | rsomics-greedy-color --json
```

## Supported strategies

| Flag value | Description | Deterministic for string nodes |
|---|---|---|
| `largest_first` (default) | Stable descending sort by degree; ties keep edge-list insertion order | Yes |
| `saturation_largest_first` | DSATUR — max saturation, tie-break by degree then insertion order | Yes |

**Not supported** (deliberately omitted due to hash-dependent tie-breaking that
cannot be reproduced portably for string nodes):
- `smallest_last` — uses CPython `set.pop()` which varies with `PYTHONHASHSEED`
- `random_sequential` — explicitly random
- `independent_set` — uses `set.pop()` internally
- `connected_sequential_bfs` / `connected_sequential_dfs` — uses
  `arbitrary_element` which is hash-dependent

## Performance

Pure-Rust, O(n + m) adjacency representation. networkx is pure Python;
Rust wins by a wide margin on any graph.

## Origin

This crate is an independent Rust reimplementation of
[`networkx.greedy_color`](https://networkx.org/documentation/stable/reference/algorithms/generated/networkx.algorithms.coloring.greedy_color.html)
based on:

- The public networkx source (BSD-3-Clause): `networkx/algorithms/coloring/greedy_coloring.py`
- Kosowski & Manuszewski, "Classical Coloring of Graphs", 2004
- Matula & Beck, "Smallest-last ordering and clustering", J. ACM 30(3), 1983

License: MIT OR Apache-2.0.
Upstream credit: NetworkX (<https://networkx.org>, BSD-3-Clause).
