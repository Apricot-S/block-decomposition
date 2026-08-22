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

fn run_decomposition<F>(decompose: F) -> Map
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

    map
}

fn print_decomposition_statistics(map: &Map) {
    let mut hand_counts = [[0usize; 5]; 2];
    for hand in map.keys() {
        let tile_count = hand.iter().map(|&count| usize::from(count)).sum::<usize>();
        let has_head = tile_count % 3 == 2;
        let num_melds = (tile_count - if has_head { 2 } else { 0 }) / 3;
        hand_counts[usize::from(has_head)][num_melds] += 1;
    }

    println!("number of single color hands: {}\n", map.len());

    for (has_head, counts) in hand_counts.iter().enumerate() {
        for (num_melds, count) in counts.iter().enumerate() {
            let num_heads = u8::from(has_head == 1);
            println!("number of hands (head: {num_heads}, melds: {num_melds}): {count}");
        }
    }

    let max_patterns = map.values().map(Vec::len).max().expect("map is non-empty");
    println!("\nmax number of decomposition patterns: {max_patterns}");
}

fn main() {
    let map0 = run_decomposition(decompose_0);

    let map1 = run_decomposition(decompose_1);
    if map0 == map1 {
        println!("map0 == map1");
    }

    let map2 = run_decomposition(decompose_2);
    if map0 == map2 {
        println!("map0 == map2");
    }

    let map3 = run_decomposition(decompose_3);
    if map0 == map3 {
        println!("map0 == map3");
    }

    println!();
    print_decomposition_statistics(&map0);
}
