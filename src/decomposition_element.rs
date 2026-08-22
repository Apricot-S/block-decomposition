// Reference:
// 与えられた手牌をブロック（和了形）に分解するアルゴリズムについて（C++ 実装） #麻雀 - Qiita
// <https://qiita.com/Cryolite/items/e254379b6e99212e6704>

/// The numbers of sequences, triplets, and pairs made up of certain tiles in a winning hand.
#[derive(Debug, Clone, Copy, Default)]
pub struct DecompositionElement {
    pub sequences: u8,
    pub triplets: u8,
    pub pairs: u8,
}

/// Table of decomposition elements.
#[rustfmt::skip]
pub const D_TABLE: [DecompositionElement; 10] = [
    DecompositionElement { sequences: 4, triplets: 0, pairs: 0 },
    DecompositionElement { sequences: 2, triplets: 0, pairs: 1 },
    DecompositionElement { sequences: 1, triplets: 1, pairs: 0 },
    DecompositionElement { sequences: 3, triplets: 0, pairs: 0 },
    DecompositionElement { sequences: 1, triplets: 0, pairs: 1 },
    DecompositionElement { sequences: 0, triplets: 1, pairs: 0 },
    DecompositionElement { sequences: 2, triplets: 0, pairs: 0 },
    DecompositionElement { sequences: 0, triplets: 0, pairs: 1 },
    DecompositionElement { sequences: 1, triplets: 0, pairs: 0 },
    DecompositionElement { sequences: 0, triplets: 0, pairs: 0 },
];

/// Table of number of melds included in decomposition elements.
pub const M_TABLE: [u8; 10] = [
    D_TABLE[0].sequences + D_TABLE[0].triplets,
    D_TABLE[1].sequences + D_TABLE[1].triplets,
    D_TABLE[2].sequences + D_TABLE[2].triplets,
    D_TABLE[3].sequences + D_TABLE[3].triplets,
    D_TABLE[4].sequences + D_TABLE[4].triplets,
    D_TABLE[5].sequences + D_TABLE[5].triplets,
    D_TABLE[6].sequences + D_TABLE[6].triplets,
    D_TABLE[7].sequences + D_TABLE[7].triplets,
    D_TABLE[8].sequences + D_TABLE[8].triplets,
    D_TABLE[9].sequences + D_TABLE[9].triplets,
];

/// Table of number of tiles included in decomposition elements.
pub const N_TABLE: [u8; 10] = [
    D_TABLE[0].sequences + 3 * D_TABLE[0].triplets + 2 * D_TABLE[0].pairs,
    D_TABLE[1].sequences + 3 * D_TABLE[1].triplets + 2 * D_TABLE[1].pairs,
    D_TABLE[2].sequences + 3 * D_TABLE[2].triplets + 2 * D_TABLE[2].pairs,
    D_TABLE[3].sequences + 3 * D_TABLE[3].triplets + 2 * D_TABLE[3].pairs,
    D_TABLE[4].sequences + 3 * D_TABLE[4].triplets + 2 * D_TABLE[4].pairs,
    D_TABLE[5].sequences + 3 * D_TABLE[5].triplets + 2 * D_TABLE[5].pairs,
    D_TABLE[6].sequences + 3 * D_TABLE[6].triplets + 2 * D_TABLE[6].pairs,
    D_TABLE[7].sequences + 3 * D_TABLE[7].triplets + 2 * D_TABLE[7].pairs,
    D_TABLE[8].sequences + 3 * D_TABLE[8].triplets + 2 * D_TABLE[8].pairs,
    D_TABLE[9].sequences + 3 * D_TABLE[9].triplets + 2 * D_TABLE[9].pairs,
];
