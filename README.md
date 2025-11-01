# block-decomposition

This project is a Rust port of [Cryolite](https://github.com/Cryolite)'s C++ implementation of the algorithms for decomposing a given hand into blocks (winning form).

Original article (Japanese): [与えられた手牌をブロック（和了形）に分解するアルゴリズムについて（C++ 実装） #麻雀 - Qiita](https://qiita.com/Cryolite/items/e254379b6e99212e6704)

## Overview

This repository provides a standalone application for demonstrating mahjong hand decomposition algorithms.

## Build and Run

Requires Rust 1.90+ and Cargo.

```sh
block-decomposition$ cargo run
```

## Example Output

```txt
number of single color hands: 1723
max number of decomposition patterns: 4

map0 == map1
map0 == map2
map0 == map3
```

These results confirm that multiple algorithmic approaches produce equivalent decompositions.

## License

This Rust port is licensed under the [MIT No Attribution license](LICENSE).

The original algorithms and C++ implementation were described by [Cryolite](https://github.com/Cryolite) in the article linked above.
