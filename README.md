# block-decomposition

Rust port of [On the Algorithm for Decomposing a Given Hand into Blocks (Winning Form) (C++ Implementation)](https://qiita.com/Cryolite/items/e254379b6e99212e6704)  
(Original article in Japanese: 「与えられた手牌をブロック（和了形）に分解するアルゴリズムについて（C++ 実装） #麻雀 - Qiita」)

## Overview

This repository provides a standalone application for demonstrating mahjong hand decomposition algorithms.

## Build and Run

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

## License

This Rust code is licensed under the [MIT No Attribution license](LICENSE).

Original algorithm and C++ implementation were described by [Cryolite](https://github.com/Cryolite) in the following Qiita article:  
[与えられた手牌をブロック（和了形）に分解するアルゴリズムについて（C++ 実装） #麻雀 - Qiita](https://qiita.com/Cryolite/items/e254379b6e99212e6704)
