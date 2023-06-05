use crate::block::Block;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameMode {
    Normal {
        /// ブロック数がこの値以下になったら面クリアの必要条件を満たす。
        block_count_target: u8,
    },
    Advance {
        /// 初期保持ブロック。
        block_holding: Block,
        /// 残り手数。
        move_count_remain: u8,
    },
}

impl GameMode {
    pub const fn is_normal(self) -> bool {
        matches!(self, Self::Normal { .. })
    }

    pub const fn is_advance(self) -> bool {
        matches!(self, Self::Advance { .. })
    }

    pub const fn block_holding(self) -> Block {
        match self {
            Self::Normal { .. } => Block::Wild,
            Self::Advance { block_holding, .. } => block_holding,
        }
    }

    pub const fn block_count_target(self) -> u8 {
        match self {
            Self::Normal { block_count_target } => block_count_target,
            Self::Advance { .. } => 3,
        }
    }

    pub const fn move_count_remain(self) -> u8 {
        match self {
            Self::Normal { .. } => u8::MAX, // 便宜上、無限大に相当する値を返す。
            Self::Advance {
                move_count_remain, ..
            } => move_count_remain,
        }
    }
}
