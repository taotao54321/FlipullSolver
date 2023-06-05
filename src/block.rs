use std::fmt::Write as _;

use anyhow::{anyhow, ensure};

use crate::macros::assert_unchecked;
use crate::move_::{Move, MoveDirection, Moves};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Block {
    Normal1 = 1,
    Normal2,
    Normal3,
    Normal4,
    Wild,
}

impl Block {
    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 5;

    pub const fn from_inner(inner: u8) -> Option<Self> {
        if Self::is_valid(inner) {
            Some(unsafe { Self::from_inner_unchecked(inner) })
        } else {
            None
        }
    }

    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub const unsafe fn from_inner_unchecked(inner: u8) -> Self {
        assert_unchecked!(Self::is_valid(inner));

        std::mem::transmute(inner)
    }

    pub const fn to_inner(self) -> u8 {
        self as u8
    }

    pub const fn is_normal(self) -> bool {
        !self.is_wild()
    }

    pub const fn is_wild(self) -> bool {
        matches!(self, Self::Wild)
    }

    pub const fn can_erase(self, other: Self) -> bool {
        match (self, other) {
            (Self::Wild, _) => true,
            (_, Self::Wild) => true,
            _ => self.to_inner() == other.to_inner(),
        }
    }

    pub const fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }
}

/// `Blocks` の列。
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BlocksCol {
    ColA = 1,
    ColB,
    ColC,
    ColD,
    ColE,
    ColF,
}

pub const BLOCKS_COL_A: BlocksCol = BlocksCol::ColA;
pub const BLOCKS_COL_B: BlocksCol = BlocksCol::ColB;
pub const BLOCKS_COL_C: BlocksCol = BlocksCol::ColC;
pub const BLOCKS_COL_D: BlocksCol = BlocksCol::ColD;
pub const BLOCKS_COL_E: BlocksCol = BlocksCol::ColE;
pub const BLOCKS_COL_F: BlocksCol = BlocksCol::ColF;

impl BlocksCol {
    pub const NUM: usize = 6;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 6;

    pub const fn from_inner(inner: u8) -> Option<Self> {
        if Self::is_valid(inner) {
            Some(unsafe { Self::from_inner_unchecked(inner) })
        } else {
            None
        }
    }

    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub const unsafe fn from_inner_unchecked(inner: u8) -> Self {
        assert_unchecked!(Self::is_valid(inner));

        std::mem::transmute(inner)
    }

    pub const fn to_inner(self) -> u8 {
        self as u8
    }

    pub const fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }

    pub const fn all() -> [Self; Self::NUM] {
        [
            BLOCKS_COL_A,
            BLOCKS_COL_B,
            BLOCKS_COL_C,
            BLOCKS_COL_D,
            BLOCKS_COL_E,
            BLOCKS_COL_F,
        ]
    }
}

impl std::fmt::Debug for BlocksCol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            BLOCKS_COL_A => "BLOCKS_COL_A",
            BLOCKS_COL_B => "BLOCKS_COL_B",
            BLOCKS_COL_C => "BLOCKS_COL_C",
            BLOCKS_COL_D => "BLOCKS_COL_D",
            BLOCKS_COL_E => "BLOCKS_COL_E",
            BLOCKS_COL_F => "BLOCKS_COL_F",
        };
        f.write_str(s)
    }
}

/// `Blocks` の行。
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BlocksRow {
    Row1 = 1,
    Row2,
    Row3,
    Row4,
    Row5,
    Row6,
}

pub const BLOCKS_ROW_1: BlocksRow = BlocksRow::Row1;
pub const BLOCKS_ROW_2: BlocksRow = BlocksRow::Row2;
pub const BLOCKS_ROW_3: BlocksRow = BlocksRow::Row3;
pub const BLOCKS_ROW_4: BlocksRow = BlocksRow::Row4;
pub const BLOCKS_ROW_5: BlocksRow = BlocksRow::Row5;
pub const BLOCKS_ROW_6: BlocksRow = BlocksRow::Row6;

impl BlocksRow {
    pub const NUM: usize = 6;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 6;

    pub const fn from_inner(inner: u8) -> Option<Self> {
        if Self::is_valid(inner) {
            Some(unsafe { Self::from_inner_unchecked(inner) })
        } else {
            None
        }
    }

    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub const unsafe fn from_inner_unchecked(inner: u8) -> Self {
        assert_unchecked!(Self::is_valid(inner));

        std::mem::transmute(inner)
    }

    pub const fn to_inner(self) -> u8 {
        self as u8
    }

    pub const fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }

    pub const fn all() -> [Self; Self::NUM] {
        [
            BLOCKS_ROW_1,
            BLOCKS_ROW_2,
            BLOCKS_ROW_3,
            BLOCKS_ROW_4,
            BLOCKS_ROW_5,
            BLOCKS_ROW_6,
        ]
    }
}

impl std::fmt::Debug for BlocksRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            BLOCKS_ROW_1 => "BLOCKS_ROW_1",
            BLOCKS_ROW_2 => "BLOCKS_ROW_2",
            BLOCKS_ROW_3 => "BLOCKS_ROW_3",
            BLOCKS_ROW_4 => "BLOCKS_ROW_4",
            BLOCKS_ROW_5 => "BLOCKS_ROW_5",
            BLOCKS_ROW_6 => "BLOCKS_ROW_6",
        };
        f.write_str(s)
    }
}

/// `Blocks` のマス。
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlocksSquare {
    A1 = 9,
    B1,
    C1,
    D1,
    E1,
    F1,
    A2 = 17,
    B2,
    C2,
    D2,
    E2,
    F2,
    A3 = 25,
    B3,
    C3,
    D3,
    E3,
    F3,
    A4 = 33,
    B4,
    C4,
    D4,
    E4,
    F4,
    A5 = 41,
    B5,
    C5,
    D5,
    E5,
    F5,
    A6 = 49,
    B6,
    C6,
    D6,
    E6,
    F6,
}

impl BlocksSquare {
    pub const MIN_VALUE: u8 = 9;
    pub const MAX_VALUE: u8 = 54;

    pub const fn from_inner(inner: u8) -> Option<Self> {
        if Self::is_valid(inner) {
            Some(unsafe { Self::from_inner_unchecked(inner) })
        } else {
            None
        }
    }

    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub const unsafe fn from_inner_unchecked(inner: u8) -> Self {
        assert_unchecked!(Self::is_valid(inner));

        std::mem::transmute(inner)
    }

    pub const fn new(col: BlocksCol, row: BlocksRow) -> Self {
        let inner = 8 * row.to_inner() + col.to_inner();

        unsafe { Self::from_inner_unchecked(inner) }
    }

    pub const fn to_inner(self) -> u8 {
        self as u8
    }

    pub const fn to_index(self) -> usize {
        self.to_inner() as usize
    }

    pub const fn col(self) -> BlocksCol {
        let col = self.to_inner() % 8;

        unsafe { BlocksCol::from_inner_unchecked(col) }
    }

    pub const fn row(self) -> BlocksRow {
        let row = self.to_inner() / 8;

        unsafe { BlocksRow::from_inner_unchecked(row) }
    }

    fn add_direction(self, dir: Direction) -> Option<Self> {
        #[rustfmt::skip]
        const SENTINELS: [bool; 8 * 8] = [
            true, true,  true,  true,  true,  true,  true,  true,
            true, false, false, false, false, false, false, true,
            true, false, false, false, false, false, false, true,
            true, false, false, false, false, false, false, true,
            true, false, false, false, false, false, false, true,
            true, false, false, false, false, false, false, true,
            true, false, false, false, false, false, false, true,
            true, true,  true,  true,  true,  true,  true,  true,
        ];

        let inner = unsafe {
            self.to_index()
                .checked_add_signed(dir.displacement())
                .unwrap_unchecked()
        };

        (!SENTINELS[inner]).then(|| unsafe { Self::from_inner_unchecked(inner as u8) })
    }

    pub const fn is_valid(inner: u8) -> bool {
        let col = inner % 8;
        let row = inner / 8;

        matches!(col, 1..=6) && matches!(row, 1..=6)
    }
}

/// 盤面左下 6x6 のブロック領域。
///
/// 上下左右に番兵を設けている(内部配列を 8x8 にする意味もある):
///
/// ```text
///    ABCDEF
///   ########
/// 1 #......#
/// 2 #......#
/// 3 #......#
/// 4 #......#
/// 5 #......#
/// 6 #......#
///   ########
/// ```
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blocks([Option<Block>; 8 * 8]);

impl Default for Blocks {
    fn default() -> Self {
        Self::new()
    }
}

impl Blocks {
    const CHAR_NONE: char = '.';
    const CHAR_NORMAL_1: char = '1';
    const CHAR_NORMAL_2: char = '2';
    const CHAR_NORMAL_3: char = '3';
    const CHAR_NORMAL_4: char = '4';
    const CHAR_WILD: char = '5';

    pub fn new() -> Self {
        Self([None; 8 * 8])
    }

    pub fn as_array(&self) -> &[Option<Block>; 8 * 8] {
        &self.0
    }

    pub fn block_count(&self) -> u8 {
        self.0.iter().copied().filter(Option::is_some).count() as u8
    }

    /// 着手の集合を更新する。
    /// 返される着手の `dst` には必ずブロックがあることが保証される。
    pub fn update_moves(&self, moves: &[Move]) -> Moves {
        moves
            .iter()
            .copied()
            .filter_map(|mv| {
                Self::move_square_direction_iter(mv.dst(), mv.direction())
                    .find_map(|(sq, mv_dir)| self[sq].map(|_| Move::new(mv.src(), sq, mv_dir)))
            })
            .collect()
    }

    /// 着手の集合と保持ブロックを与え、実際の合法手の集合を返す。
    /// 着手の集合は `update_moves()` で返されたものと仮定している。
    ///
    /// 直接ワイルドカードに当てる手しかない場合、空集合を返す。
    ///
    /// NOTE: FC 版フリップルでは、直接ワイルドカードに当てる手しかない局面は手詰まりとみなされる。
    /// (そうでない局面で直接ワイルドカードに当てることは可能)
    pub fn legal_moves(&self, moves: &[Move], block_holding: Block) -> Moves {
        let mut moves_legal = Moves::new();
        let mut all_wild = true;

        for &mv in moves {
            unsafe { assert_unchecked!(self[mv.dst()].is_some()) }
            let block = unsafe { self[mv.dst()].unwrap_unchecked() };
            if block_holding.can_erase(block) {
                moves_legal.push(mv);
                if block.is_normal() {
                    all_wild = false;
                }
            }
        }

        if all_wild {
            return Moves::new();
        }

        moves_legal
    }

    /// 着手と保持ブロックを与え、合法手かどうかを返す。
    pub fn is_legal_move(&self, mv: Move, block_holding: Block) -> bool {
        self[mv.dst()].is_some_and(|block| block_holding.can_erase(block))
    }

    /// 着手を行い、(結果、次の保持ブロック、置換前に最後にブロックが通った位置, 同時消し数) を返す。
    /// 着手は合法だと仮定している。
    pub fn do_move(&self, mv: Move, block_holding: Block) -> (Self, Block, BlocksSquare, u8) {
        // mv.dst() には必ずブロックがあるはず。
        unsafe { assert_unchecked!(self[mv.dst()].is_some()) }
        let block_first = unsafe { self[mv.dst()].unwrap_unchecked() };

        // 投げたブロックにより block_first が消せるはず。
        unsafe { assert_unchecked!(block_holding.can_erase(block_first)) }

        // ワイルドカードに直接当たった場合、投げた方のブロックを block_first とする。
        // NOTE: ノーミス前提ならワイルドカードがワイルドカードに直接当たるケースはありえない。
        let block_first = match block_first {
            Block::Wild => block_holding,
            _ => block_first,
        };
        unsafe { assert_unchecked!(block_first != Block::Wild) }

        let mut blocks_res = self.clone();
        let mut block_holding_nxt = block_first;
        let mut sq_last = mv.dst();
        let mut erase_count = 0;

        macro_rules! erase {
            ($sq:expr) => {{
                if mv.direction() == MoveDirection::Vertical {
                    blocks_res[$sq] = None;
                } else {
                    blocks_res.erase_shift($sq);
                }
                erase_count += 1;
            }};
        }

        // 最初に当たったブロックを消す。
        erase!(mv.dst());

        // その後の移動の処理。
        let sqs = Self::move_square_direction_iter(mv.dst(), mv.direction())
            .map(|(sq, _)| sq)
            .skip(1);
        for sq in sqs {
            if let Some(block) = self[sq] {
                if block_first.can_erase(block) {
                    // 当たったブロックが消去可能なら単に消す。
                    // (block がワイルドカードであるか、または block_first と同種であるケースが該当)
                    erase!(sq);
                } else {
                    // 当たったブロックが消去不能なら置換を行い、そこで止まる。
                    blocks_res[sq] = Some(block_first);
                    block_holding_nxt = block;
                    break;
                }
            } else {
                // ブロックに当たらないなら素通り。
            }
            sq_last = sq;
        }

        (blocks_res, block_holding_nxt, sq_last, erase_count)
    }

    /// 着手によるブロックの動きをシミュレートする。
    fn move_square_direction_iter(
        start: BlocksSquare,
        mv_dir: MoveDirection,
    ) -> impl Iterator<Item = (BlocksSquare, MoveDirection)> + std::iter::FusedIterator {
        std::iter::successors(Some((start, mv_dir)), |&(sq, mv_dir)| {
            let dir = match mv_dir {
                MoveDirection::Horizontal => Direction::Left,
                MoveDirection::Vertical => Direction::Down,
            };
            if let Some(sq_nxt) = sq.add_direction(dir) {
                Some((sq_nxt, mv_dir))
            } else if mv_dir == MoveDirection::Horizontal {
                sq.add_direction(Direction::Down)
                    .map(|sq_nxt| (sq_nxt, MoveDirection::Vertical))
            } else {
                None
            }
        })
    }

    fn erase_shift(&mut self, mut sq: BlocksSquare) {
        while let Some(sq_nxt) = sq.add_direction(Direction::Up) {
            self[sq] = self[sq_nxt];
            sq = sq_nxt;
        }
        self[sq] = None;
    }

    fn block_to_char(block: Option<Block>) -> char {
        match block {
            None => Self::CHAR_NONE,
            Some(Block::Normal1) => Self::CHAR_NORMAL_1,
            Some(Block::Normal2) => Self::CHAR_NORMAL_2,
            Some(Block::Normal3) => Self::CHAR_NORMAL_3,
            Some(Block::Normal4) => Self::CHAR_NORMAL_4,
            Some(Block::Wild) => Self::CHAR_WILD,
        }
    }

    fn char_to_block(ch: char) -> anyhow::Result<Option<Block>> {
        match ch {
            Self::CHAR_NONE => Ok(None),
            Self::CHAR_NORMAL_1 => Ok(Some(Block::Normal1)),
            Self::CHAR_NORMAL_2 => Ok(Some(Block::Normal2)),
            Self::CHAR_NORMAL_3 => Ok(Some(Block::Normal3)),
            Self::CHAR_NORMAL_4 => Ok(Some(Block::Normal4)),
            Self::CHAR_WILD => Ok(Some(Block::Wild)),
            _ => Err(anyhow!("無効な Blocks 内ブロック文字: '{ch}'")),
        }
    }
}

impl std::ops::Index<(BlocksCol, BlocksRow)> for Blocks {
    type Output = Option<Block>;

    fn index(&self, (col, row): (BlocksCol, BlocksRow)) -> &Self::Output {
        self.index(BlocksSquare::new(col, row))
    }
}

impl std::ops::IndexMut<(BlocksCol, BlocksRow)> for Blocks {
    fn index_mut(&mut self, (col, row): (BlocksCol, BlocksRow)) -> &mut Self::Output {
        self.index_mut(BlocksSquare::new(col, row))
    }
}

impl std::ops::Index<BlocksSquare> for Blocks {
    type Output = Option<Block>;

    fn index(&self, sq: BlocksSquare) -> &Self::Output {
        &self.0[sq.to_index()]
    }
}

impl std::ops::IndexMut<BlocksSquare> for Blocks {
    fn index_mut(&mut self, sq: BlocksSquare) -> &mut Self::Output {
        &mut self.0[sq.to_index()]
    }
}

impl std::str::FromStr for Blocks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        ensure!(
            lines.len() == 6,
            "Blocks 文字列はちょうど 6 行でなければならない"
        );

        let mut this = Self::new();

        for (row, line) in itertools::zip_eq(BlocksRow::all(), lines) {
            let chars: Vec<_> = line.chars().collect();
            ensure!(
                chars.len() == 6,
                "Blocks 文字列の行はちょうど 6 文字でなければならない"
            );

            for (col, ch) in itertools::zip_eq(BlocksCol::all(), chars) {
                let block = Self::char_to_block(ch)?;
                this[(col, row)] = block;
            }
        }

        Ok(this)
    }
}

impl std::fmt::Display for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in BlocksRow::all() {
            for col in BlocksCol::all() {
                let block = self[(col, row)];
                f.write_char(Self::block_to_char(block))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
}

impl Direction {
    fn displacement(self) -> isize {
        match self {
            Self::Up => -8,
            Self::Left => -1,
            Self::Down => 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::ground::*;

    use super::*;

    fn parse_blocks(s: impl AsRef<str>) -> Blocks {
        s.as_ref().parse().unwrap()
    }

    fn make_moves(moves: impl IntoIterator<Item = Move>) -> Moves {
        moves.into_iter().collect()
    }

    #[test]
    fn test_square_new() {
        for (row, col) in itertools::iproduct!(BlocksRow::all(), BlocksCol::all()) {
            let sq = BlocksSquare::new(col, row);
            assert_eq!(sq.col(), col);
            assert_eq!(sq.row(), row);
        }
    }

    #[test]
    fn test_blocks_io() {
        let cases = [
            indoc! {"
                ......
                ......
                ......
                ......
                ......
                ......
            "},
            indoc! {"
                1.1.1.
                111111
                222222
                333333
                445444
                444444
            "},
        ];

        for case in cases {
            let blocks = parse_blocks(case);
            assert_eq!(blocks.to_string(), case);
        }
    }

    #[test]
    fn test_blocks_update_moves() {
        let cases = [(
            indoc! {"
                ......
                1.....
                1.....
                1..3..
                12.34.
                12.344
            "},
            [
                Move::new(GROUND_ROW_1, BlocksSquare::F1, MoveDirection::Vertical),
                Move::new(GROUND_ROW_2, BlocksSquare::E1, MoveDirection::Vertical),
                Move::new(GROUND_ROW_3, BlocksSquare::D1, MoveDirection::Vertical),
                Move::new(GROUND_ROW_4, BlocksSquare::C1, MoveDirection::Vertical),
                Move::new(GROUND_ROW_5, BlocksSquare::B1, MoveDirection::Vertical),
                Move::new(GROUND_ROW_6, BlocksSquare::A1, MoveDirection::Vertical),
                Move::new(GROUND_ROW_7, BlocksSquare::F1, MoveDirection::Horizontal),
                Move::new(GROUND_ROW_8, BlocksSquare::F2, MoveDirection::Horizontal),
                Move::new(GROUND_ROW_9, BlocksSquare::F3, MoveDirection::Horizontal),
                Move::new(GROUND_ROW_10, BlocksSquare::F4, MoveDirection::Horizontal),
                Move::new(GROUND_ROW_11, BlocksSquare::F5, MoveDirection::Horizontal),
                Move::new(GROUND_ROW_12, BlocksSquare::F6, MoveDirection::Horizontal),
            ],
            make_moves([
                Move::new(GROUND_ROW_1, BlocksSquare::F6, MoveDirection::Vertical),
                Move::new(GROUND_ROW_2, BlocksSquare::E5, MoveDirection::Vertical),
                Move::new(GROUND_ROW_3, BlocksSquare::D4, MoveDirection::Vertical),
                Move::new(GROUND_ROW_5, BlocksSquare::B5, MoveDirection::Vertical),
                Move::new(GROUND_ROW_6, BlocksSquare::A2, MoveDirection::Vertical),
                Move::new(GROUND_ROW_7, BlocksSquare::A2, MoveDirection::Vertical),
                Move::new(GROUND_ROW_8, BlocksSquare::A2, MoveDirection::Horizontal),
                Move::new(GROUND_ROW_9, BlocksSquare::A3, MoveDirection::Horizontal),
                Move::new(GROUND_ROW_10, BlocksSquare::D4, MoveDirection::Horizontal),
                Move::new(GROUND_ROW_11, BlocksSquare::E5, MoveDirection::Horizontal),
                Move::new(GROUND_ROW_12, BlocksSquare::F6, MoveDirection::Horizontal),
            ]),
        )];

        for (blocks, moves_before, moves_after) in cases {
            let blocks = parse_blocks(blocks);
            assert_eq!(blocks.update_moves(&moves_before), moves_after);
        }
    }

    #[test]
    fn test_blocks_do_move_horizontal() {
        const HORI: MoveDirection = MoveDirection::Horizontal;

        let cases = [
            (
                indoc! {"
                    2.1111
                    333311
                    222222
                    333333
                    444444
                    333333
                "},
                Move::new(GROUND_ROW_7, BlocksSquare::F1, HORI),
                Block::Normal1,
                indoc! {"
                    1.....
                    333311
                    222222
                    333333
                    444444
                    333333
                "},
                Block::Normal2,
                BlocksSquare::B1,
                4,
            ),
            (
                indoc! {"
                    211111
                    333311
                    222222
                    333333
                    444444
                    333333
                "},
                Move::new(GROUND_ROW_12, BlocksSquare::F6, HORI),
                Block::Wild,
                indoc! {"
                    ......
                    211111
                    333311
                    222222
                    333333
                    444444
                "},
                Block::Normal3,
                BlocksSquare::A6,
                6,
            ),
            (
                indoc! {"
                    ......
                    ......
                    22222.
                    33333.
                    344444
                    411111
                "},
                Move::new(GROUND_ROW_10, BlocksSquare::E4, HORI),
                Block::Normal3,
                indoc! {"
                    ......
                    ......
                    ......
                    .2222.
                    244444
                    311111
                "},
                Block::Normal4,
                BlocksSquare::A5,
                6,
            ),
            (
                indoc! {"
                    ......
                    ......
                    222225
                    333333
                    344444
                    411111
                "},
                Move::new(GROUND_ROW_9, BlocksSquare::F3, HORI),
                Block::Normal3,
                indoc! {"
                    ......
                    ......
                    22223.
                    333333
                    344444
                    411111
                "},
                Block::Normal2,
                BlocksSquare::F3,
                1,
            ),
            (
                indoc! {"
                    ......
                    ......
                    222222
                    333335
                    344444
                    411111
                "},
                Move::new(GROUND_ROW_10, BlocksSquare::F4, HORI),
                Block::Normal3,
                indoc! {"
                    ......
                    ......
                    ......
                    .22222
                    244444
                    311111
                "},
                Block::Normal4,
                BlocksSquare::A5,
                7,
            ),
            (
                indoc! {"
                    ......
                    ......
                    222222
                    335333
                    344444
                    411111
                "},
                Move::new(GROUND_ROW_10, BlocksSquare::F4, HORI),
                Block::Normal3,
                indoc! {"
                    ......
                    ......
                    ......
                    .22222
                    244444
                    311111
                "},
                Block::Normal4,
                BlocksSquare::A5,
                7,
            ),
            (
                indoc! {"
                    ......
                    .5....
                    .22222
                    .35333
                    .44444
                    .11111
                "},
                Move::new(GROUND_ROW_8, BlocksSquare::B2, HORI),
                Block::Normal1,
                indoc! {"
                    ......
                    ......
                    .22222
                    .35333
                    .44444
                    .11111
                "},
                Block::Normal1,
                BlocksSquare::A6,
                1,
            ),
        ];

        for (before, mv, block, after, block_res, sq_res, erase_count) in cases {
            let before = parse_blocks(before);
            let after = parse_blocks(after);
            assert_eq!(
                before.do_move(mv, block),
                (after, block_res, sq_res, erase_count)
            );
        }
    }

    #[test]
    fn test_blocks_do_move_vertical() {
        const VERT: MoveDirection = MoveDirection::Vertical;

        let cases = [
            (
                indoc! {"
                    .....1
                    ....21
                    ...322
                    ..4322
                    ..4322
                    ..4322
                "},
                Move::new(GROUND_ROW_1, BlocksSquare::F1, VERT),
                Block::Normal1,
                indoc! {"
                    ......
                    ....2.
                    ...321
                    ..4322
                    ..4322
                    ..4322
                "},
                Block::Normal2,
                BlocksSquare::F2,
                2,
            ),
            (
                indoc! {"
                    ......
                    11....
                    11....
                    11....
                    11222.
                    112223
                "},
                Move::new(GROUND_ROW_1, BlocksSquare::A2, VERT),
                Block::Wild,
                indoc! {"
                    ......
                    .1....
                    .1....
                    .1....
                    .1222.
                    .12223
                "},
                Block::Normal1,
                BlocksSquare::A6,
                5,
            ),
            (
                indoc! {"
                    ......
                    15....
                    12....
                    11....
                    11222.
                    112223
                "},
                Move::new(GROUND_ROW_1, BlocksSquare::B2, VERT),
                Block::Normal1,
                indoc! {"
                    ......
                    1.....
                    11....
                    11....
                    11222.
                    112223
                "},
                Block::Normal2,
                BlocksSquare::B2,
                1,
            ),
            (
                indoc! {"
                    ......
                    15....
                    11....
                    11....
                    11222.
                    122223
                "},
                Move::new(GROUND_ROW_1, BlocksSquare::B2, VERT),
                Block::Normal1,
                indoc! {"
                    ......
                    1.....
                    1.....
                    1.....
                    1.222.
                    112223
                "},
                Block::Normal2,
                BlocksSquare::B5,
                4,
            ),
            (
                indoc! {"
                    ......
                    11....
                    11....
                    15....
                    11222.
                    112223
                "},
                Move::new(GROUND_ROW_1, BlocksSquare::B2, VERT),
                Block::Normal1,
                indoc! {"
                    ......
                    1.....
                    1.....
                    1.....
                    1.222.
                    1.2223
                "},
                Block::Normal1,
                BlocksSquare::B6,
                5,
            ),
            (
                indoc! {"
                    ......
                    1.....
                    1.....
                    1.....
                    1.222.
                    152223
                "},
                Move::new(GROUND_ROW_1, BlocksSquare::B6, VERT),
                Block::Normal1,
                indoc! {"
                    ......
                    1.....
                    1.....
                    1.....
                    1.222.
                    1.2223
                "},
                Block::Normal1,
                BlocksSquare::B6,
                1,
            ),
        ];

        for (before, mv, block, after, block_res, sq_res, erase_count) in cases {
            let before = parse_blocks(before);
            let after = parse_blocks(after);
            assert_eq!(
                before.do_move(mv, block),
                (after, block_res, sq_res, erase_count)
            );
        }
    }
}
