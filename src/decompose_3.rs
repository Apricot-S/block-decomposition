// Reference:
// 与えられた手牌をブロック（和了形）に分解するアルゴリズムについて（C++ 実装） #麻雀 - Qiita
// <https://qiita.com/Cryolite/items/e254379b6e99212e6704>
//
// 実装4（雀頭位置列挙 + 三連刻変換）

use crate::block::{self, Block, Blocks};
use crate::decomposition_element::DecompositionElement;

fn decompose_3_impl_(
    single_color_hand: &mut [u8; 9],
    quetou: Option<usize>,
    result: &mut [DecompositionElement; 9],
) -> bool {
    for i in 0..9 {
        match single_color_hand[i] {
            4 => {
                if let Some(q) = quetou
                    && q == i
                {
                    if i + 2 >= 9 || single_color_hand[i + 1] < 2 || single_color_hand[i + 2] < 2 {
                        return false;
                    }

                    single_color_hand[i] -= 4;
                    single_color_hand[i + 1] -= 2;
                    single_color_hand[i + 2] -= 2;
                    result[i].num_sequence += 2;
                    result[i].num_pair += 1;
                    continue;
                }

                if i + 2 >= 9 || single_color_hand[i + 1] == 0 || single_color_hand[i + 2] == 0 {
                    return false;
                }

                single_color_hand[i] -= 4;
                single_color_hand[i + 1] -= 1;
                single_color_hand[i + 2] -= 1;
                result[i].num_sequence += 1;
                result[i].num_triplet += 1;
                continue;
            }
            3 => {
                if let Some(q) = quetou
                    && q == i
                {
                    if i + 2 >= 9 || single_color_hand[i + 1] == 0 || single_color_hand[i + 2] == 0
                    {
                        return false;
                    }

                    single_color_hand[i] -= 3;
                    single_color_hand[i + 1] -= 1;
                    single_color_hand[i + 2] -= 1;
                    result[i].num_sequence += 1;
                    result[i].num_pair += 1;
                    continue;
                }

                single_color_hand[i] -= 3;
                result[i].num_triplet += 1;
                continue;
            }
            2 => {
                if let Some(q) = quetou
                    && q == i
                {
                    single_color_hand[i] -= 2;
                    result[i].num_pair += 1;
                    continue;
                }

                if i + 2 >= 9 || single_color_hand[i + 1] < 2 || single_color_hand[i + 2] < 2 {
                    return false;
                }

                single_color_hand[i] -= 2;
                single_color_hand[i + 1] -= 2;
                single_color_hand[i + 2] -= 2;
                result[i].num_sequence += 2;
                continue;
            }
            1 => {
                if i + 2 >= 9 || single_color_hand[i + 1] == 0 || single_color_hand[i + 2] == 0 {
                    return false;
                }

                single_color_hand[i] -= 1;
                single_color_hand[i + 1] -= 1;
                single_color_hand[i + 2] -= 1;
                result[i].num_sequence += 1;
                continue;
            }
            0 => (),
            _ => unreachable!(),
        }
    }

    true
}

fn decompose_3_impl(
    single_color_hand: &[u8; 9],
    quetou: Option<usize>,
    result: &mut Vec<Blocks>,
) -> bool {
    let mut hand = *single_color_hand;
    let mut result_ = [DecompositionElement::default(); 9];
    let success = decompose_3_impl_(&mut hand, quetou, &mut result_);

    for i in 0..9 {
        while result_[i].num_sequence >= 1 {
            for blocks in result.iter_mut() {
                blocks.push(Block::new(block::Type::Shunzi, i as u8));
            }
            result_[i].num_sequence -= 1;
        }

        if result_[i].num_triplet == 1 {
            if i + 3 < 9
                && result_[i + 1].num_triplet == 1
                && result_[i + 2].num_triplet == 1
                && result_[i + 3].num_triplet == 1
            {
                let original_result_size = result.len();
                result.reserve(3 * original_result_size);

                let clones: Vec<_> = result.to_vec();
                result.extend(clones.iter().cloned());
                result.extend(clones);

                for r in result.iter_mut().take(original_result_size) {
                    r.push(Block::new(block::Type::Kezi, i as u8));
                    r.push(Block::new(block::Type::Kezi, (i + 1) as u8));
                    r.push(Block::new(block::Type::Kezi, (i + 2) as u8));
                    r.push(Block::new(block::Type::Kezi, (i + 3) as u8));
                }

                for r in result
                    .iter_mut()
                    .skip(original_result_size)
                    .take(original_result_size)
                {
                    r.push(Block::new(block::Type::Kezi, i as u8));
                    r.push(Block::new(block::Type::Shunzi, (i + 1) as u8));
                    r.push(Block::new(block::Type::Shunzi, (i + 1) as u8));
                    r.push(Block::new(block::Type::Shunzi, (i + 1) as u8));
                }

                for r in result.iter_mut().skip(2 * original_result_size) {
                    r.push(Block::new(block::Type::Shunzi, i as u8));
                    r.push(Block::new(block::Type::Shunzi, i as u8));
                    r.push(Block::new(block::Type::Shunzi, i as u8));
                    r.push(Block::new(block::Type::Kezi, (i + 3) as u8));
                }

                result_[i].num_triplet -= 1;
                result_[i + 1].num_triplet -= 1;
                result_[i + 2].num_triplet -= 1;
                result_[i + 3].num_triplet -= 1;
            } else if i + 2 < 9
                && result_[i + 1].num_triplet == 1
                && result_[i + 2].num_triplet == 1
            {
                let original_result_size = result.len();
                result.reserve(2 * original_result_size);

                let clones: Vec<_> = result.to_vec();
                result.extend(clones);

                for r in result.iter_mut().take(original_result_size) {
                    r.push(Block::new(block::Type::Kezi, i as u8));
                    r.push(Block::new(block::Type::Kezi, (i + 1) as u8));
                    r.push(Block::new(block::Type::Kezi, (i + 2) as u8));
                }

                for r in result.iter_mut().skip(original_result_size) {
                    r.push(Block::new(block::Type::Shunzi, i as u8));
                    r.push(Block::new(block::Type::Shunzi, i as u8));
                    r.push(Block::new(block::Type::Shunzi, i as u8));
                }

                result_[i].num_triplet -= 1;
                result_[i + 1].num_triplet -= 1;
                result_[i + 2].num_triplet -= 1;
            } else {
                for blocks in result.iter_mut() {
                    blocks.push(Block::new(block::Type::Kezi, i as u8));
                }
                result_[i].num_triplet -= 1;
            }
        }

        if result_[i].num_pair == 1 {
            for blocks in result.iter_mut() {
                blocks.push(Block::new(block::Type::Quetou, i as u8));
            }
            result_[i].num_pair -= 1;
        }
    }

    success
}

pub fn decompose_3(single_color_hand: &[u8; 9]) -> Vec<Blocks> {
    let mut result: Vec<Blocks> = Vec::new();

    let n = single_color_hand.iter().sum::<u8>() as usize;
    if n.is_multiple_of(3) {
        result.push(Vec::new());
        let success = decompose_3_impl(single_color_hand, None, &mut result);
        debug_assert!(success);
    } else {
        let s = single_color_hand
            .iter()
            .enumerate()
            .map(|(i, &c)| i * c as usize)
            .sum::<usize>();

        for quetou in (((2 * s) % 3)..9).step_by(3) {
            let mut tmp_result: Vec<Blocks> = vec![Vec::new()];
            if decompose_3_impl(single_color_hand, Some(quetou), &mut tmp_result) {
                result.extend(tmp_result);
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
    fn decompose_3_111222333() {
        let hand = [3, 3, 3, 0, 0, 0, 0, 0, 0];
        let ret = decompose_3(&hand);

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

    #[test]
    fn decompose_3_11122233344455() {
        let hand = [3, 3, 3, 3, 2, 0, 0, 0, 0];
        let ret = decompose_3(&hand);

        let decomp_pattern = vec![
            vec![
                Block::new(block::Type::Shunzi, 0),
                Block::new(block::Type::Shunzi, 0),
                Block::new(block::Type::Shunzi, 0),
                Block::new(block::Type::Kezi, 3),
                Block::new(block::Type::Quetou, 4),
            ],
            vec![
                Block::new(block::Type::Shunzi, 1),
                Block::new(block::Type::Shunzi, 1),
                Block::new(block::Type::Shunzi, 1),
                Block::new(block::Type::Kezi, 0),
                Block::new(block::Type::Quetou, 4),
            ],
            vec![
                Block::new(block::Type::Shunzi, 1),
                Block::new(block::Type::Shunzi, 2),
                Block::new(block::Type::Shunzi, 2),
                Block::new(block::Type::Kezi, 0),
                Block::new(block::Type::Quetou, 1),
            ],
            vec![
                Block::new(block::Type::Kezi, 0),
                Block::new(block::Type::Kezi, 1),
                Block::new(block::Type::Kezi, 2),
                Block::new(block::Type::Kezi, 3),
                Block::new(block::Type::Quetou, 4),
            ],
        ];

        assert_eq!(ret, decomp_pattern);
    }
}
