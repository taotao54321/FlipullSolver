use anyhow::{anyhow, ensure, Context as _};

use crate::block::{Block, Blocks};
use crate::cost::{calc_hero_move_cost, calc_throw_cost, Cost};
use crate::ground::{GroundRow, GROUND_ROW_12};
use crate::macros::assert_unchecked;
use crate::move_::{Move, Moves};

/// 局面。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    hero_row: GroundRow,
    blocks: Blocks,
    block_holding: Block,
    move_count_remain: u8,
}

impl Position {
    pub fn new(blocks: Blocks, block_holding: Block, move_count_remain: u8) -> Self {
        Self {
            hero_row: GROUND_ROW_12,
            blocks,
            block_holding,
            move_count_remain,
        }
    }

    pub fn hero_row(&self) -> GroundRow {
        self.hero_row
    }

    pub fn blocks(&self) -> &Blocks {
        &self.blocks
    }

    pub fn block_holding(&self) -> Block {
        self.block_holding
    }

    pub fn move_count_remain(&self) -> u8 {
        self.move_count_remain
    }

    pub fn block_count(&self) -> u8 {
        self.blocks.block_count()
    }

    pub fn update_moves(&self, moves: &[Move]) -> Moves {
        self.blocks.update_moves(moves)
    }

    pub fn legal_moves(&self, moves: &[Move]) -> Moves {
        self.blocks.legal_moves(moves, self.block_holding)
    }

    pub fn is_legal_move(&self, mv: Move) -> bool {
        self.blocks.is_legal_move(mv, self.block_holding)
    }

    /// 着手を行い、(結果, 自機の移動コスト, ブロック投げコスト, 同時消し数) を返す。
    /// 着手は合法だと仮定している。
    pub fn do_move(&self, mv: Move) -> (Self, Cost, Cost, u8) {
        unsafe { assert_unchecked!(self.move_count_remain > 0) }

        let cost_hero_move = calc_hero_move_cost(self.hero_row, mv.src());

        let hero_row = mv.src();
        let (blocks, block_holding, sq_last, erase_count) =
            self.blocks.do_move(mv, self.block_holding);
        let move_count_remain = self.move_count_remain - 1;

        let cost_throw = calc_throw_cost(mv.src(), sq_last);

        let pos = Self {
            hero_row,
            blocks,
            block_holding,
            move_count_remain,
        };

        (pos, cost_hero_move, cost_throw, erase_count)
    }
}

impl std::str::FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (line, s) = s
            .split_once('\n')
            .ok_or_else(|| anyhow!("局面文字列の最初の行がない: '{s}'"))?;

        let tokens: Vec<_> = line.split_ascii_whitespace().collect();
        ensure!(
            tokens.len() == 3,
            "局面文字列の最初の行はちょうど 3 つのトークンを持たねばならない: '{line}'"
        );

        let hero_row: u8 = tokens[0]
            .parse()
            .with_context(|| format!("自機位置が数値でない: '{}'", tokens[0]))?;
        let hero_row =
            GroundRow::from_inner(hero_row).ok_or_else(|| anyhow!("無効な自機位置: {hero_row}"))?;

        let block_holding: u8 = tokens[1]
            .parse()
            .with_context(|| format!("保持ブロックが数値でない: '{}'", tokens[1]))?;
        let block_holding = Block::from_inner(block_holding)
            .ok_or_else(|| anyhow!("無効な保持ブロック値: {block_holding}"))?;

        let move_count_remain: u8 = tokens[2]
            .parse()
            .with_context(|| format!("残り手数が数値でない: '{}'", tokens[2]))?;

        let blocks: Blocks = s.parse()?;

        Ok(Self {
            hero_row,
            blocks,
            block_holding,
            move_count_remain,
        })
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {}",
            self.hero_row.to_inner(),
            self.block_holding.to_inner(),
            self.move_count_remain
        )?;

        self.blocks.fmt(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::block::BlocksSquare;
    use crate::ground::*;
    use crate::move_::MoveDirection;

    use super::*;

    fn parse_position(s: impl AsRef<str>) -> Position {
        s.as_ref().parse().unwrap()
    }

    #[test]
    fn test_io() {
        let cases = [
            indoc! {"
                1 1 10
                ......
                ......
                ......
                ......
                ......
                ......
            "},
            indoc! {"
                12 5 255
                1.1.1.
                111111
                222222
                333333
                444444
                444444
            "},
        ];

        for case in cases {
            let pos = parse_position(case);
            assert_eq!(pos.to_string(), case);
        }
    }

    #[test]
    fn test_do_move() {
        let cases = [
            (
                indoc! {"
                    11 3 5
                    ......
                    ......
                    222222
                    333333
                    344444
                    311111
                "},
                Move::new(GROUND_ROW_10, BlocksSquare::F4, MoveDirection::Horizontal),
                indoc! {"
                    10 3 4
                    ......
                    ......
                    ......
                    .22222
                    .44444
                    211111
                "},
            ),
            (
                indoc! {"
                    1 1 5
                    ......
                    11....
                    11....
                    11....
                    11222.
                    312223
                "},
                Move::new(GROUND_ROW_6, BlocksSquare::A2, MoveDirection::Vertical),
                indoc! {"
                    6 3 4
                    ......
                    .1....
                    .1....
                    .1....
                    .1222.
                    112223
                "},
            ),
        ];

        for (before, mv, after) in cases {
            let before = parse_position(before);
            let after = parse_position(after);
            let (after_actual, _, _, _) = before.do_move(mv);
            assert_eq!(after_actual, after);
        }
    }
}
