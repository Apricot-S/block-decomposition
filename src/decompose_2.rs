// Reference:
// 与えられた手牌をブロック（和了形）に分解するアルゴリズムについて（C++ 実装） #麻雀 - Qiita
// <https://qiita.com/Cryolite/items/e254379b6e99212e6704>
//
// 実装3（非重複型バックトラックⅡ）

use crate::block::{self, Block, Blocks};
use crate::decomposition_element::{D_TABLE, N_TABLE};

fn decompose_2_impl(
    i: usize,
    has_head: bool,
    single_color_hand: &mut [u8; 9],
    blocks: &mut Blocks,
    result: &mut Vec<Blocks>,
) {
    debug_assert!(i <= 9);

    if i == 9 {
        result.push(blocks.clone());
        return;
    }

    let x = if i + 1 < 9 {
        single_color_hand[i + 1]
    } else {
        0
    };
    let y = if i + 2 < 9 {
        single_color_hand[i + 2]
    } else {
        0
    };

    for (d, n) in D_TABLE.iter().zip(N_TABLE) {
        if has_head && d.pairs == 1 {
            continue;
        }
        if n != single_color_hand[i] {
            continue;
        }
        if d.sequences > x {
            continue;
        }
        if d.sequences > y {
            continue;
        }

        single_color_hand[i] -= n;
        if i + 1 < 9 {
            single_color_hand[i + 1] -= d.sequences;
        }
        if i + 2 < 9 {
            single_color_hand[i + 2] -= d.sequences;
        }

        for _ in 0..d.sequences {
            blocks.push(Block::new(block::Type::Shunzi, i));
        }
        if d.triplets > 0 {
            blocks.push(Block::new(block::Type::Kezi, i));
        }
        if d.pairs > 0 {
            blocks.push(Block::new(block::Type::Quetou, i));
        }

        let h = has_head || d.pairs > 0;
        decompose_2_impl(i + 1, h, single_color_hand, blocks, result);

        if d.pairs > 0 {
            blocks.pop();
        }
        if d.triplets > 0 {
            blocks.pop();
        }
        for _ in 0..d.sequences {
            blocks.pop();
        }

        if i + 2 < 9 {
            single_color_hand[i + 2] += d.sequences;
        }
        if i + 1 < 9 {
            single_color_hand[i + 1] += d.sequences;
        }
        single_color_hand[i] += n;
    }
}

pub fn decompose_2(single_color_hand: &[u8; 9]) -> Vec<Blocks> {
    let mut hand = *single_color_hand;
    let mut blocks: Blocks = Vec::new();
    let mut result: Vec<Blocks> = Vec::new();
    decompose_2_impl(0, false, &mut hand, &mut blocks, &mut result);

    for blocks in &mut result {
        blocks.sort();
    }
    result.sort();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompose_2_111222333() {
        let hand = [3, 3, 3, 0, 0, 0, 0, 0, 0];
        let ret = decompose_2(&hand);

        let decomp_pattern = vec![
            vec![
                Block::new(block::Type::Shunzi, 0),
                Block::new(block::Type::Shunzi, 0),
                Block::new(block::Type::Shunzi, 0),
            ],
            vec![
                Block::new(block::Type::Kezi, 0),
                Block::new(block::Type::Kezi, 1),
                Block::new(block::Type::Kezi, 2),
            ],
        ];

        assert_eq!(ret, decomp_pattern);
    }
}
