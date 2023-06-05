use arrayvec::ArrayVec;

use crate::block::BlocksSquare;
use crate::ground::GroundRow;

/// ブロックの移動方向。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Move {
    src: GroundRow,
    /// 最初に当たるブロックの位置。
    dst: BlocksSquare,
    dir: MoveDirection,
}

impl Move {
    pub fn new(src: GroundRow, dst: BlocksSquare, dir: MoveDirection) -> Self {
        Self { src, dst, dir }
    }

    pub fn src(self) -> GroundRow {
        self.src
    }

    pub fn dst(self) -> BlocksSquare {
        self.dst
    }

    pub fn direction(self) -> MoveDirection {
        self.dir
    }
}

pub type Moves = ArrayVec<Move, 12>;
