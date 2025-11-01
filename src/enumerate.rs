// Reference:
// 与えられた手牌をブロック（和了形）に分解するアルゴリズムについて（C++ 実装） #麻雀 - Qiita
// <https://qiita.com/Cryolite/items/e254379b6e99212e6704>

use crate::decomposition_element::{D_TABLE, M_TABLE, N_TABLE};

pub fn enumerate_single_color_winning_hand<F>(
    i: usize,
    num_melds: u8,
    has_head: bool,
    single_color_hand: &mut [u8; 9],
    callback: &mut F,
) where
    F: FnMut(&[u8; 9]),
{
    debug_assert!(i <= 9);
    debug_assert!(num_melds <= 4);

    if i == 9 {
        callback(single_color_hand);
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

    for ((d, m), n) in D_TABLE.iter().zip(M_TABLE).zip(N_TABLE) {
        if num_melds + m > 4 {
            continue;
        }
        if has_head && d.num_pair == 1 {
            continue;
        }
        if single_color_hand[i] + n > 4 {
            continue;
        }
        if d.num_sequence > x {
            continue;
        }
        if d.num_sequence > y {
            continue;
        }

        single_color_hand[i] += n;
        if i + 1 < 9 {
            single_color_hand[i + 1] += d.num_sequence;
        }
        if i + 2 < 9 {
            single_color_hand[i + 2] += d.num_sequence;
        }

        let mm = num_melds + m;
        let h = has_head || d.num_pair > 0;
        enumerate_single_color_winning_hand(i + 1, mm, h, single_color_hand, callback);

        if i + 2 < 9 {
            single_color_hand[i + 2] -= d.num_sequence;
        }
        if i + 1 < 9 {
            single_color_hand[i + 1] -= d.num_sequence;
        }
        single_color_hand[i] -= n;
    }
}
