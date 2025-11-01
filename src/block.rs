// Reference:
// 与えられた手牌をブロック（和了形）に分解するアルゴリズムについて（C++ 実装） #麻雀 - Qiita
// <https://qiita.com/Cryolite/items/e254379b6e99212e6704>
//
// ブロック分解のデータ表現

#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Shunzi = 0,
    Kezi = 1,
    Gangzi = 2,
    Quetou = 3,
}

impl Type {
    #[inline]
    const fn from_u8(v: u8) -> Self {
        match v {
            0 => Type::Shunzi,
            1 => Type::Kezi,
            2 => Type::Gangzi,
            3 => Type::Quetou,
            _ => panic!("invalid type tag"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Block {
    pack: u8,
}

impl Block {
    #[inline]
    pub const fn new(ty: Type, tile: u8) -> Self {
        debug_assert!(tile < 34);
        let pack = ((ty as u8) << 6) | tile;
        Self { pack }
    }

    #[inline]
    pub const fn get_type(&self) -> Type {
        Type::from_u8(self.pack >> 6 & 0x3)
    }

    #[inline]
    pub const fn is_shunzi(&self) -> bool {
        matches!(self.get_type(), Type::Shunzi)
    }

    #[inline]
    pub const fn is_kezi(&self) -> bool {
        matches!(self.get_type(), Type::Kezi)
    }

    #[inline]
    pub const fn is_gangzi(&self) -> bool {
        matches!(self.get_type(), Type::Gangzi)
    }

    #[inline]
    pub const fn is_quetou(&self) -> bool {
        matches!(self.get_type(), Type::Quetou)
    }

    #[inline]
    pub const fn get_tile(&self) -> u8 {
        self.pack & 0x3F
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.get_type();
        let tile = self.get_tile();
        match ty {
            Type::Shunzi => write!(f, "(shunzi, {tile})"),
            Type::Kezi => write!(f, "(kezi, {tile})"),
            Type::Gangzi => write!(f, "(gangzi, {tile})"),
            Type::Quetou => write!(f, "(quetou, {tile})"),
        }
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

pub type Blocks = Vec<Block>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_block_shunzi() {
        let b = Block::new(Type::Shunzi, 0);
        assert_eq!(format!("{b}"), "(shunzi, 0)");
    }
}
