use std::fmt::Write as _;

use anyhow::{anyhow, ensure};

use crate::block::{BlocksCol, BlocksRow};
use crate::macros::assert_unchecked;

/// `Ground` の列。
#[repr(u8)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum GroundCol {
    ColA = 1,
    ColB,
    ColC,
    ColD,
    ColE,
    ColF,
    ColG,
    ColH,
}

pub const GROUND_COL_A: GroundCol = GroundCol::ColA;
pub const GROUND_COL_B: GroundCol = GroundCol::ColB;
pub const GROUND_COL_C: GroundCol = GroundCol::ColC;
pub const GROUND_COL_D: GroundCol = GroundCol::ColD;
pub const GROUND_COL_E: GroundCol = GroundCol::ColE;
pub const GROUND_COL_F: GroundCol = GroundCol::ColF;
pub const GROUND_COL_G: GroundCol = GroundCol::ColG;
pub const GROUND_COL_H: GroundCol = GroundCol::ColH;

impl GroundCol {
    pub const NUM: usize = 8;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 8;

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

    pub const fn to_index(self) -> usize {
        (self.to_inner() - 1) as usize
    }

    /// 1 つ右の列を返す。
    pub const fn next(self) -> Option<Self> {
        Self::from_inner(self.to_inner() + 1)
    }

    /// ブロック領域の列かどうかを返す。
    pub const fn is_blocks_area(self) -> bool {
        self.to_inner() <= GROUND_COL_F.to_inner()
    }

    pub const fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }

    pub const fn all() -> [Self; Self::NUM] {
        [
            GROUND_COL_A,
            GROUND_COL_B,
            GROUND_COL_C,
            GROUND_COL_D,
            GROUND_COL_E,
            GROUND_COL_F,
            GROUND_COL_G,
            GROUND_COL_H,
        ]
    }
}

impl From<BlocksCol> for GroundCol {
    fn from(bcol: BlocksCol) -> Self {
        match bcol {
            BlocksCol::ColA => GROUND_COL_A,
            BlocksCol::ColB => GROUND_COL_B,
            BlocksCol::ColC => GROUND_COL_C,
            BlocksCol::ColD => GROUND_COL_D,
            BlocksCol::ColE => GROUND_COL_E,
            BlocksCol::ColF => GROUND_COL_F,
        }
    }
}

impl TryFrom<GroundCol> for BlocksCol {
    type Error = anyhow::Error;

    fn try_from(gcol: GroundCol) -> Result<Self, Self::Error> {
        match gcol {
            GROUND_COL_A => Ok(BlocksCol::ColA),
            GROUND_COL_B => Ok(BlocksCol::ColB),
            GROUND_COL_C => Ok(BlocksCol::ColC),
            GROUND_COL_D => Ok(BlocksCol::ColD),
            GROUND_COL_E => Ok(BlocksCol::ColE),
            GROUND_COL_F => Ok(BlocksCol::ColF),
            _ => Err(anyhow!("ブロック領域の列ではない: {gcol:?}")),
        }
    }
}

impl std::fmt::Debug for GroundCol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            GROUND_COL_A => "GROUND_COL_A",
            GROUND_COL_B => "GROUND_COL_B",
            GROUND_COL_C => "GROUND_COL_C",
            GROUND_COL_D => "GROUND_COL_D",
            GROUND_COL_E => "GROUND_COL_E",
            GROUND_COL_F => "GROUND_COL_F",
            GROUND_COL_G => "GROUND_COL_G",
            GROUND_COL_H => "GROUND_COL_H",
        };
        f.write_str(s)
    }
}

/// `Ground` の行。
#[repr(u8)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum GroundRow {
    Row1 = 1,
    Row2,
    Row3,
    Row4,
    Row5,
    Row6,
    Row7,
    Row8,
    Row9,
    Row10,
    Row11,
    Row12,
}

pub const GROUND_ROW_1: GroundRow = GroundRow::Row1;
pub const GROUND_ROW_2: GroundRow = GroundRow::Row2;
pub const GROUND_ROW_3: GroundRow = GroundRow::Row3;
pub const GROUND_ROW_4: GroundRow = GroundRow::Row4;
pub const GROUND_ROW_5: GroundRow = GroundRow::Row5;
pub const GROUND_ROW_6: GroundRow = GroundRow::Row6;
pub const GROUND_ROW_7: GroundRow = GroundRow::Row7;
pub const GROUND_ROW_8: GroundRow = GroundRow::Row8;
pub const GROUND_ROW_9: GroundRow = GroundRow::Row9;
pub const GROUND_ROW_10: GroundRow = GroundRow::Row10;
pub const GROUND_ROW_11: GroundRow = GroundRow::Row11;
pub const GROUND_ROW_12: GroundRow = GroundRow::Row12;

impl GroundRow {
    pub const NUM: usize = 12;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 12;

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

    pub const fn to_index(self) -> usize {
        (self.to_inner() - 1) as usize
    }

    /// 1 つ上の行を返す。
    pub const fn prev(self) -> Option<Self> {
        Self::from_inner(self.to_inner() - 1)
    }

    /// ブロック領域の行かどうかを返す。
    pub const fn is_blocks_area(self) -> bool {
        self.to_inner() >= GROUND_ROW_7.to_inner()
    }

    pub const fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }

    pub const fn all() -> [Self; Self::NUM] {
        [
            GROUND_ROW_1,
            GROUND_ROW_2,
            GROUND_ROW_3,
            GROUND_ROW_4,
            GROUND_ROW_5,
            GROUND_ROW_6,
            GROUND_ROW_7,
            GROUND_ROW_8,
            GROUND_ROW_9,
            GROUND_ROW_10,
            GROUND_ROW_11,
            GROUND_ROW_12,
        ]
    }
}

impl From<BlocksRow> for GroundRow {
    fn from(brow: BlocksRow) -> Self {
        match brow {
            BlocksRow::Row1 => GROUND_ROW_7,
            BlocksRow::Row2 => GROUND_ROW_8,
            BlocksRow::Row3 => GROUND_ROW_9,
            BlocksRow::Row4 => GROUND_ROW_10,
            BlocksRow::Row5 => GROUND_ROW_11,
            BlocksRow::Row6 => GROUND_ROW_12,
        }
    }
}

impl TryFrom<GroundRow> for BlocksRow {
    type Error = anyhow::Error;

    fn try_from(grow: GroundRow) -> Result<Self, Self::Error> {
        match grow {
            GROUND_ROW_7 => Ok(BlocksRow::Row1),
            GROUND_ROW_8 => Ok(BlocksRow::Row2),
            GROUND_ROW_9 => Ok(BlocksRow::Row3),
            GROUND_ROW_10 => Ok(BlocksRow::Row4),
            GROUND_ROW_11 => Ok(BlocksRow::Row5),
            GROUND_ROW_12 => Ok(BlocksRow::Row6),
            _ => Err(anyhow!("ブロック領域の行ではない: {grow:?}")),
        }
    }
}

impl std::fmt::Debug for GroundRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            GROUND_ROW_1 => "GROUND_ROW_1",
            GROUND_ROW_2 => "GROUND_ROW_2",
            GROUND_ROW_3 => "GROUND_ROW_3",
            GROUND_ROW_4 => "GROUND_ROW_4",
            GROUND_ROW_5 => "GROUND_ROW_5",
            GROUND_ROW_6 => "GROUND_ROW_6",
            GROUND_ROW_7 => "GROUND_ROW_7",
            GROUND_ROW_8 => "GROUND_ROW_8",
            GROUND_ROW_9 => "GROUND_ROW_9",
            GROUND_ROW_10 => "GROUND_ROW_10",
            GROUND_ROW_11 => "GROUND_ROW_11",
            GROUND_ROW_12 => "GROUND_ROW_12",
        };
        f.write_str(s)
    }
}

/// `Ground` のマス。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GroundSquare {
    col: GroundCol,
    row: GroundRow,
}

impl GroundSquare {
    pub const fn new(col: GroundCol, row: GroundRow) -> Self {
        Self { col, row }
    }

    pub const fn to_index(self) -> usize {
        8 * self.row.to_index() + self.col.to_index()
    }

    /// 左下 6x6 のブロック領域かどうかを返す。
    pub const fn is_blocks_area(self) -> bool {
        self.col.is_blocks_area() && self.row.is_blocks_area()
    }
}

/// `Ground` の要素。
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GroundElement {
    Wall = 1,
    Pipe,
}

impl GroundElement {
    pub const fn is_wall(self) -> bool {
        matches!(self, Self::Wall)
    }

    pub const fn is_pipe(self) -> bool {
        matches!(self, Self::Pipe)
    }
}

/// ブロックを除いたマップ (8x12)。
///
/// ```text
///    ABCDEFGH
///  1 ........
///  2 ........
///  3 ........
///  4 ........
///  5 ........
///  6 ........
///  7 ........
///  8 ........
///  9 ........
/// 10 ........
/// 11 ........
/// 12 ........
/// ```
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ground([Option<GroundElement>; 8 * 12]);

impl Default for Ground {
    fn default() -> Self {
        Self::new()
    }
}

impl Ground {
    const CHAR_NONE: char = '.';
    const CHAR_WALL: char = '#';
    const CHAR_PIPE: char = '|';

    pub fn new() -> Self {
        Self([None; 8 * 12])
    }

    fn elem_to_char(elem: Option<GroundElement>) -> char {
        match elem {
            None => Self::CHAR_NONE,
            Some(GroundElement::Wall) => Self::CHAR_WALL,
            Some(GroundElement::Pipe) => Self::CHAR_PIPE,
        }
    }

    fn char_to_elem(ch: char) -> anyhow::Result<Option<GroundElement>> {
        match ch {
            Self::CHAR_NONE => Ok(None),
            Self::CHAR_WALL => Ok(Some(GroundElement::Wall)),
            Self::CHAR_PIPE => Ok(Some(GroundElement::Pipe)),
            _ => Err(anyhow!("無効な Ground 内要素文字: '{ch}'")),
        }
    }
}

impl std::ops::Index<(GroundCol, GroundRow)> for Ground {
    type Output = Option<GroundElement>;

    fn index(&self, (col, row): (GroundCol, GroundRow)) -> &Self::Output {
        self.index(GroundSquare::new(col, row))
    }
}

impl std::ops::IndexMut<(GroundCol, GroundRow)> for Ground {
    fn index_mut(&mut self, (col, row): (GroundCol, GroundRow)) -> &mut Self::Output {
        self.index_mut(GroundSquare::new(col, row))
    }
}

impl std::ops::Index<GroundSquare> for Ground {
    type Output = Option<GroundElement>;

    fn index(&self, sq: GroundSquare) -> &Self::Output {
        &self.0[sq.to_index()]
    }
}

impl std::ops::IndexMut<GroundSquare> for Ground {
    fn index_mut(&mut self, sq: GroundSquare) -> &mut Self::Output {
        &mut self.0[sq.to_index()]
    }
}

impl std::str::FromStr for Ground {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        ensure!(
            lines.len() == 12,
            "Ground 文字列はちょうど 12 行でなければならない"
        );

        let mut this = Self::new();

        for (row, line) in itertools::zip_eq(GroundRow::all(), lines) {
            let chars: Vec<_> = line.chars().collect();
            ensure!(
                chars.len() == 8,
                "Ground 文字列の行はちょうど 8 行でなければならない"
            );

            for (col, ch) in itertools::zip_eq(GroundCol::all(), chars) {
                let elem = Self::char_to_elem(ch)?;
                this[(col, row)] = elem;
            }
        }

        Ok(this)
    }
}

impl std::fmt::Display for Ground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in GroundRow::all() {
            for col in GroundCol::all() {
                let elem = self[(col, row)];
                f.write_char(Self::elem_to_char(elem))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    fn parse_ground(s: impl AsRef<str>) -> Ground {
        s.as_ref().parse().unwrap()
    }

    #[test]
    fn test_ground_io() {
        let cases = [
            indoc! {"
                ........
                ........
                ........
                ........
                ........
                ........
                ........
                ........
                ........
                ........
                ........
                ........
            "},
            indoc! {"
                #####...
                ###.....
                ###.....
                #.||....
                #.......
                |.......
                ........
                ........
                ........
                ........
                ........
                ........
            "},
        ];

        for case in cases {
            let ground = parse_ground(case);
            assert_eq!(ground.to_string(), case);
        }
    }
}
