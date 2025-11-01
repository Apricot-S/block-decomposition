// Reference:
// 与えられた手牌をブロック（和了形）に分解するアルゴリズムについて（C++ 実装） #麻雀 - Qiita
// <https://qiita.com/Cryolite/items/e254379b6e99212e6704>
//
// 実装2（非重複型バックトラックⅠ）

use crate::block::{self, Block, Blocks};

fn decompose_1_impl(
    i: usize,
    single_color_hand: &mut [u8; 9],
    blocks: &mut Blocks,
    result: &mut Vec<Blocks>,
) {
    debug_assert!(i <= 9);

    if i == 9 {
        result.push(blocks.clone());
        return;
    }

    if single_color_hand[i] == 0 {
        decompose_1_impl(i + 1, single_color_hand, blocks, result);
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
        decompose_1_impl(i, single_color_hand, blocks, result);
        blocks.pop();
        single_color_hand[i + 2] += 1;
        single_color_hand[i + 1] += 1;
        single_color_hand[i] += 1;
    }

    if single_color_hand[i] >= 3 {
        single_color_hand[i] -= 3;
        blocks.push(Block::new(block::Type::Kezi, i as u8));
        decompose_1_impl(i + 1, single_color_hand, blocks, result);
        blocks.pop();
        single_color_hand[i] += 3;
    }
}

pub fn decompose_1(single_color_hand: &[u8; 9]) -> Vec<Blocks> {
    let mut result: Vec<Blocks> = Vec::new();

    {
        let mut hand = *single_color_hand;
        let mut blocks: Blocks = Vec::new();
        decompose_1_impl(0, &mut hand, &mut blocks, &mut result);
    }

    if result.is_empty() {
        for quetou in 0..9 {
            if single_color_hand[quetou] >= 2 {
                let mut hand = *single_color_hand;
                hand[quetou] -= 2;
                let mut blocks: Blocks = Vec::new();
                let mut tmp_result: Vec<Blocks> = Vec::new();
                decompose_1_impl(0, &mut hand, &mut blocks, &mut tmp_result);

                if !tmp_result.is_empty() {
                    for blocks in &mut tmp_result {
                        blocks.push(Block::new(block::Type::Quetou, quetou as u8));
                    }
                    result.extend(tmp_result);
                }
            }
        }
    }

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
    fn decompose_1_111222333() {
        let hand = [3, 3, 3, 0, 0, 0, 0, 0, 0];
        let ret = decompose_1(&hand);

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
