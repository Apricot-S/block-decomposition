// Reference:
// 与えられた手牌をブロック（和了形）に分解するアルゴリズムについて（C++ 実装） #麻雀 - Qiita
// <https://qiita.com/Cryolite/items/e254379b6e99212e6704>
//
// 実装1（重複型バックトラック）

use crate::block::{self, Block, Blocks};

fn decompose_0_impl(
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

    if single_color_hand[i] >= 3 {
        single_color_hand[i] -= 3;
        blocks.push(Block::new(block::Type::Kezi, i as u8));
        decompose_0_impl(i, has_head, single_color_hand, blocks, result);
        blocks.pop();
        single_color_hand[i] += 3;
    }

    if single_color_hand[i] >= 2 && !has_head {
        single_color_hand[i] -= 2;
        blocks.push(Block::new(block::Type::Quetou, i as u8));
        decompose_0_impl(i, true, single_color_hand, blocks, result);
        blocks.pop();
        single_color_hand[i] += 2;
    }

    if i + 2 < 9
        && single_color_hand[i] >= 1
        && single_color_hand[i + 1] >= 1
        && single_color_hand[i + 2] >= 1
    {
        single_color_hand[i] -= 1;
        single_color_hand[i + 1] -= 1;
        single_color_hand[i + 2] -= 1;
        blocks.push(Block::new(block::Type::Shunzi, i as u8));
        decompose_0_impl(i, has_head, single_color_hand, blocks, result);
        blocks.pop();
        single_color_hand[i + 2] += 1;
        single_color_hand[i + 1] += 1;
        single_color_hand[i] += 1;
    }

    if single_color_hand[i] == 0 {
        decompose_0_impl(i + 1, has_head, single_color_hand, blocks, result);
    }
}

pub fn decompose_0(single_color_hand: &[u8; 9]) -> Vec<Blocks> {
    let mut hand = *single_color_hand;
    let mut blocks: Blocks = Vec::new();
    let mut result: Vec<Blocks> = Vec::new();
    decompose_0_impl(0, false, &mut hand, &mut blocks, &mut result);

    for blocks in &mut result {
        blocks.sort();
    }
    result.sort();
    result.dedup();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompose_0_111222333() {
        let hand = [3, 3, 3, 0, 0, 0, 0, 0, 0];
        let ret = decompose_0(&hand);

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
