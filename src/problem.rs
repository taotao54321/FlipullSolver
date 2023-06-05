use std::fmt::Write as _;

use anyhow::{anyhow, bail, ensure, Context as _};
use itertools::Itertools as _;

use crate::block::{Block, Blocks, BlocksRow};
use crate::game_mode::GameMode;
use crate::ground::{Ground, GroundCol, GroundElement, GroundRow, GroundSquare, GROUND_COL_F};

#[derive(Debug)]
pub struct Problem {
    game_mode: GameMode,
    ground: Ground,
    blocks: Blocks,
}

impl Problem {
    /// NORMAL モードの問題を作る。
    pub fn new_normal(
        ground: Ground,
        blocks: Blocks,
        block_count_target: u8,
    ) -> anyhow::Result<Self> {
        Self::validate_ground(&ground)?;

        Ok(Self {
            game_mode: GameMode::Normal { block_count_target },
            ground,
            blocks,
        })
    }

    /// ADVANCE モードの問題を作る。
    pub fn new_advance(
        ground: Ground,
        blocks: Blocks,
        block_holding: Block,
        move_count_remain: u8,
    ) -> anyhow::Result<Self> {
        Self::validate_ground(&ground)?;

        ensure!(
            !blocks.as_array().contains(&Some(Block::Wild)),
            "ADVANCE モードの盤面にワイルドカードが含まれている"
        );

        Ok(Self {
            game_mode: GameMode::Advance {
                block_holding,
                move_count_remain,
            },
            ground,
            blocks,
        })
    }

    /// `Ground` が正しいかどうかチェックする。
    fn validate_ground(ground: &Ground) -> anyhow::Result<()> {
        for (row, col) in itertools::iproduct!(GroundRow::all(), GroundCol::all()) {
            let sq = GroundSquare::new(col, row);
            let elem = ground[sq];

            if sq.is_blocks_area() && col != GROUND_COL_F {
                ensure!(
                    elem.is_none(),
                    "ground の左下 5x6 に壁/パイプがあってはならない"
                );
            }

            if elem.is_some_and(GroundElement::is_wall) {
                if let Some(row_pre) = row.prev() {
                    let elem_pre = ground[(col, row_pre)];
                    ensure!(
                        elem_pre.is_some_and(GroundElement::is_wall),
                        "壁の上には壁がなければならない"
                    );
                }
            }
        }

        Ok(())
    }

    pub fn game_mode(&self) -> GameMode {
        self.game_mode
    }

    pub fn ground(&self) -> &Ground {
        &self.ground
    }

    pub fn blocks(&self) -> &Blocks {
        &self.blocks
    }
}

impl std::str::FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let game_mode = {
            let line_mode = lines
                .next()
                .ok_or_else(|| anyhow!("問題文字列のモード行がない"))?;
            let line_param = lines
                .next()
                .ok_or_else(|| anyhow!("問題文字列のパラメータ行がない"))?;

            match line_mode {
                "normal" => {
                    let block_count_target: u8 = line_param
                        .parse()
                        .with_context(|| format!("目標ブロック数が数値でない: {line_param}"))?;
                    GameMode::Normal { block_count_target }
                }
                "advance" => {
                    let tokens: Vec<_> = line_param.split_ascii_whitespace().collect();
                    ensure!(
                        tokens.len() == 2,
                        "ADVANCE モードのパラメータ行はちょうど 2 つのトークンを持たねばならない"
                    );
                    let block_holding: u8 = tokens[0]
                        .parse()
                        .with_context(|| format!("保持ブロックが数値でない: {}", tokens[0]))?;
                    let block_holding = Block::from_inner(block_holding)
                        .ok_or_else(|| anyhow!("無効な保持ブロック値: {block_holding}"))?;
                    let move_count_remain: u8 = tokens[1]
                        .parse()
                        .with_context(|| format!("残り手数が数値でない: {}", tokens[1]))?;
                    GameMode::Advance {
                        block_holding,
                        move_count_remain,
                    }
                }
                _ => bail!("無効なゲームモード: {line_mode}"),
            }
        };

        let lines: Vec<_> = lines.collect();
        ensure!(
            lines.len() == 12,
            "問題の Ground/Blocks 文字列はちょうど 12 行でなければならない"
        );

        let s_ground = lines
            .iter()
            .map(|line| line.replace(is_block_char, "."))
            .join("\n");

        let s_blocks = lines[6..]
            .iter()
            .map(|line| line[..6].replace(|ch: char| !is_block_char(ch), "."))
            .join("\n");

        let ground: Ground = s_ground.parse()?;
        let blocks: Blocks = s_blocks.parse()?;

        let this = match game_mode {
            GameMode::Normal { block_count_target } => {
                Self::new_normal(ground, blocks, block_count_target)
            }
            GameMode::Advance {
                block_holding,
                move_count_remain,
            } => Self::new_advance(ground, blocks, block_holding, move_count_remain),
        }
        .context("問題が制約を満たしていない")?;

        Ok(this)
    }
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use itertools::EitherOrBoth;

        match self.game_mode {
            GameMode::Normal { block_count_target } => {
                writeln!(f, "normal")?;
                writeln!(f, "{block_count_target}")?;
            }
            GameMode::Advance {
                block_holding,
                move_count_remain,
            } => {
                writeln!(f, "advance")?;
                writeln!(f, "{} {move_count_remain}", block_holding.to_inner())?;
            }
        }

        let s_ground = self.ground.to_string();
        let s_blocks = self.blocks.to_string();

        let lines_ground: Vec<_> = s_ground.lines().collect();
        let lines_blocks: Vec<_> = s_blocks.lines().collect();

        for grow in GroundRow::all() {
            if grow.is_blocks_area() {
                let brow = BlocksRow::try_from(grow).unwrap();
                let it = lines_ground[grow.to_index()]
                    .chars()
                    .zip_longest(lines_blocks[usize::from(brow.to_inner() - 1)].chars());
                for chs in it {
                    match chs {
                        EitherOrBoth::Both(gch, bch) => match (gch, bch) {
                            ('|', '.') => f.write_char(gch)?,
                            ('.', _) => f.write_char(bch)?,
                            _ => unreachable!(
                                "Ground 文字と Blocks 文字の組み合わせが変: ('{gch}', '{bch}')"
                            ),
                        },
                        EitherOrBoth::Left(gch) => f.write_char(gch)?,
                        EitherOrBoth::Right(_) => {
                            unreachable!("Ground 文字列の行は Blocks 文字列の行より長いはず")
                        }
                    }
                }
            } else {
                write!(f, "{}", lines_ground[grow.to_index()])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn is_block_char(ch: char) -> bool {
    matches!(ch, '1'..='5')
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    fn parse_problem(s: impl AsRef<str>) -> Problem {
        s.as_ref().parse().unwrap()
    }

    #[test]
    fn test_io() {
        let cases = [
            indoc! {"
                advance
                2 33
                #####...
                ##......
                #.......
                ........
                ........
                ........
                311432..
                222242|.
                334422..
                422224|.
                344244..
                133344..
            "},
            indoc! {"
                normal
                7
                #####...
                ##......
                #.......
                ........
                ........
                ........
                311432..
                222242|.
                334422..
                422224|.
                344244..
                133344..
            "},
            indoc! {"
                normal
                8
                ####....
                ###.....
                ##......
                #.......
                ........
                ........
                13224...
                11144...
                32411...
                12223...
                32333|..
                34444...
            "},
        ];

        for case in cases {
            let problem = parse_problem(case);
            assert_eq!(problem.to_string(), case);
        }
    }
}
