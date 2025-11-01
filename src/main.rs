// Reference:
// 与えられた手牌をブロック（和了形）に分解するアルゴリズムについて（C++ 実装） #麻雀 - Qiita
// <https://qiita.com/Cryolite/items/e254379b6e99212e6704>

mod block;
mod decompose_0;
mod decompose_1;
mod decompose_2;
mod decompose_3;
mod decomposition_element;
mod enumerate;

use block::Blocks;
use decompose_0::decompose_0;
use decompose_1::decompose_1;
use decompose_2::decompose_2;
use decompose_3::decompose_3;
use enumerate::enumerate_single_color_winning_hand;
use std::collections::HashMap;

type Map = HashMap<[u8; 9], Vec<Blocks>>;

fn run_decomposition<F>(decompose: F, verbose: bool) -> Map
where
    F: Fn(&[u8; 9]) -> Vec<Blocks>,
{
    let mut map = Map::new();

    let mut single_color_hand = [0u8; 9];
    let mut callback = |hand: &[u8; 9]| {
        let blocks = decompose(hand);
        map.insert(*hand, blocks);
    };
    enumerate_single_color_winning_hand(0, 0, false, &mut single_color_hand, &mut callback);

    if verbose {
        println!("number of single color hands: {}", map.len());
        println!(
            "max number of decomposition pattern: {}\n",
            map.values().map(|v| v.len()).max().unwrap_or(0)
        );
    }

    map
}

fn main() {
    let map0 = run_decomposition(decompose_0, true);
    let map1 = run_decomposition(decompose_1, false);
    let map2 = run_decomposition(decompose_2, false);
    let map3 = run_decomposition(decompose_3, false);

    if map0 == map1 {
        println!("map0 == map1");
    }

    if map0 == map2 {
        println!("map0 == map2");
    }

    if map0 == map3 {
        println!("map0 == map3");
    }
}
