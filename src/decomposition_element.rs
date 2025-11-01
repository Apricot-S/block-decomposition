// Reference:
// 与えられた手牌をブロック（和了形）に分解するアルゴリズムについて（C++ 実装） #麻雀 - Qiita
// <https://qiita.com/Cryolite/items/e254379b6e99212e6704>

/// An element that represents the number of blocks (meld, pair) made up of
/// certain tiles in a winning hand.
#[derive(Debug, Clone, Copy, Default)]
pub struct DecompositionElement {
    pub num_sequence: u8,
    pub num_triplet: u8,
    pub num_pair: u8,
}

/// Table of decomposition elements.
#[rustfmt::skip]
pub const D_TABLE: [DecompositionElement; 10] = [
    DecompositionElement { num_sequence: 4, num_triplet: 0, num_pair: 0 },
    DecompositionElement { num_sequence: 2, num_triplet: 0, num_pair: 1 },
    DecompositionElement { num_sequence: 1, num_triplet: 1, num_pair: 0 },
    DecompositionElement { num_sequence: 3, num_triplet: 0, num_pair: 0 },
    DecompositionElement { num_sequence: 1, num_triplet: 0, num_pair: 1 },
    DecompositionElement { num_sequence: 0, num_triplet: 1, num_pair: 0 },
    DecompositionElement { num_sequence: 2, num_triplet: 0, num_pair: 0 },
    DecompositionElement { num_sequence: 0, num_triplet: 0, num_pair: 1 },
    DecompositionElement { num_sequence: 1, num_triplet: 0, num_pair: 0 },
    DecompositionElement { num_sequence: 0, num_triplet: 0, num_pair: 0 },
];

/// Table of number of melds included in decomposition elements.
pub const M_TABLE: [u8; 10] = [
    D_TABLE[0].num_sequence + D_TABLE[0].num_triplet,
    D_TABLE[1].num_sequence + D_TABLE[1].num_triplet,
    D_TABLE[2].num_sequence + D_TABLE[2].num_triplet,
    D_TABLE[3].num_sequence + D_TABLE[3].num_triplet,
    D_TABLE[4].num_sequence + D_TABLE[4].num_triplet,
    D_TABLE[5].num_sequence + D_TABLE[5].num_triplet,
    D_TABLE[6].num_sequence + D_TABLE[6].num_triplet,
    D_TABLE[7].num_sequence + D_TABLE[7].num_triplet,
    D_TABLE[8].num_sequence + D_TABLE[8].num_triplet,
    D_TABLE[9].num_sequence + D_TABLE[9].num_triplet,
];

/// Table of number of tiles included in decomposition elements.
pub const N_TABLE: [u8; 10] = [
    D_TABLE[0].num_sequence + 3 * D_TABLE[0].num_triplet + 2 * D_TABLE[0].num_pair,
    D_TABLE[1].num_sequence + 3 * D_TABLE[1].num_triplet + 2 * D_TABLE[1].num_pair,
    D_TABLE[2].num_sequence + 3 * D_TABLE[2].num_triplet + 2 * D_TABLE[2].num_pair,
    D_TABLE[3].num_sequence + 3 * D_TABLE[3].num_triplet + 2 * D_TABLE[3].num_pair,
    D_TABLE[4].num_sequence + 3 * D_TABLE[4].num_triplet + 2 * D_TABLE[4].num_pair,
    D_TABLE[5].num_sequence + 3 * D_TABLE[5].num_triplet + 2 * D_TABLE[5].num_pair,
    D_TABLE[6].num_sequence + 3 * D_TABLE[6].num_triplet + 2 * D_TABLE[6].num_pair,
    D_TABLE[7].num_sequence + 3 * D_TABLE[7].num_triplet + 2 * D_TABLE[7].num_pair,
    D_TABLE[8].num_sequence + 3 * D_TABLE[8].num_triplet + 2 * D_TABLE[8].num_pair,
    D_TABLE[9].num_sequence + 3 * D_TABLE[9].num_triplet + 2 * D_TABLE[9].num_pair,
];
