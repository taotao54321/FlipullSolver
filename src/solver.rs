use anyhow::{anyhow, ensure, Context as _};
use itertools::Itertools as _;
use log::debug;

use crate::block::{BlocksCol, BlocksRow, BlocksSquare, BLOCKS_COL_F, BLOCKS_ROW_1};
use crate::cost::{calc_clear_cost, Cost, COST_INF};
use crate::game_mode::GameMode;
use crate::ground::{GroundCol, GroundRow, GROUND_COL_A, GROUND_COL_H};
use crate::move_::{Move, MoveDirection, Moves};
use crate::position::Position;
use crate::problem::Problem;
use crate::util::VecExt as _;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolverConfig {
    /// 最終面かどうか。
    pub last_stage: bool,
    /// 5 個以上の同時消しを禁止するかどうか。
    pub forbid_five: bool,
    /// just clear を禁止するかどうか(乱数調整のために必要なことがある)。
    pub forbid_just: bool,
    /// 最大コスト (枝刈り用)。
    pub max_cost: Cost,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            last_stage: false,
            forbid_five: false,
            forbid_just: false,
            max_cost: COST_INF,
        }
    }
}

#[derive(Debug)]
pub struct SolverArgument {
    game_mode: GameMode,
    pos: Position,
    moves: Moves,
    config: SolverConfig,
}

impl SolverArgument {
    pub fn new(problem: &Problem, config: &SolverConfig) -> Self {
        let game_mode = problem.game_mode();

        let pos = {
            let blocks = problem.blocks().clone();
            let block_holding = game_mode.block_holding();
            let move_count_remain = game_mode.move_count_remain();
            Position::new(blocks, block_holding, move_count_remain)
        };

        // 着手集合の初期値を求める。
        // このとき、下段から投げる手を優先するようにする。
        // (面開始時に自機は最下段にいるので、この方が最適解に早く到達しやすいだろうという予想)
        let moves: Moves = GroundRow::all()
            .into_iter()
            .rev()
            .filter_map(|grow| {
                if grow.is_blocks_area() {
                    // 下 6 行の場合、壁/パイプがなければ横に投げる着手が存在し、
                    // さもなくば着手は存在しない(壁/パイプは左下 5x6 内には存在しないことに注意)。
                    GroundCol::all()
                        .into_iter()
                        .all(|gcol| problem.ground()[(gcol, grow)].is_none())
                        .then(|| {
                            let brow = BlocksRow::try_from(grow).unwrap();
                            let dst = BlocksSquare::new(BLOCKS_COL_F, brow);
                            Move::new(grow, dst, MoveDirection::Horizontal)
                        })
                } else {
                    // 上 6 行の場合、壁/パイプに当たってから下に落ちる。
                    // まず最初に当たる壁/パイプの列を求める。
                    let gcol = GroundCol::all()
                        .into_iter()
                        .rev()
                        .find(|&gcol| problem.ground()[(gcol, grow)].is_some());
                    // 最初に当たった壁/パイプの 1 つ右の列に落ちることになる。
                    // 壁/パイプに当たらないなら A 列に落ちる。
                    // H 列の壁/パイプに当たった場合、H 列に落ちるものとみなす。
                    let gcol =
                        gcol.map_or(GROUND_COL_A, |gcol| gcol.next().unwrap_or(GROUND_COL_H));
                    // 左 6 列に落ちる場合のみ有効な着手とする。
                    BlocksCol::try_from(gcol).ok().map(|bcol| {
                        let dst = BlocksSquare::new(bcol, BLOCKS_ROW_1);
                        Move::new(grow, dst, MoveDirection::Vertical)
                    })
                }
            })
            .collect();

        // 着手集合を局面に合わせて更新。
        let moves = pos.update_moves(&moves);

        Self {
            game_mode,
            pos,
            moves,
            config: config.clone(),
        }
    }

    pub fn game_mode(&self) -> GameMode {
        self.game_mode
    }

    pub fn position(&self) -> &Position {
        &self.pos
    }

    pub fn moves(&self) -> &Moves {
        &self.moves
    }

    pub fn config(&self) -> &SolverConfig {
        &self.config
    }
}

#[derive(Debug)]
pub struct Solution(Vec<GroundRow>);

impl Solution {
    pub fn from_moves(moves: &[Move]) -> Self {
        let srcs: Vec<_> = moves.iter().copied().map(Move::src).collect();
        Self(srcs)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn srcs(&self) -> &[GroundRow] {
        &self.0
    }

    pub fn verify(&self, arg: &SolverArgument) -> anyhow::Result<Cost> {
        let mut pos = arg.pos.clone();
        let mut moves = arg.moves.clone();
        let mut cost_total = 0;

        for (i, &src) in self.srcs().iter().enumerate() {
            ensure!(
                pos.move_count_remain() > 0,
                "{i} 番目の着手前に残り手数が尽きた"
            );

            let mv = moves
                .iter()
                .copied()
                .find(|mv| mv.src() == src)
                .ok_or_else(|| anyhow!("{i} 番目の着手が不正: {src:?}"))?;
            ensure!(pos.is_legal_move(mv), "{i} 番目の着手が不正: {mv:?}");
            let (pos_nxt, cost_hero_move, cost_throw, erase_count) = pos.do_move(mv);
            if arg.config.forbid_five {
                ensure!(
                    erase_count < 5,
                    "{i} 番目の着手で 5 個以上の同時消しが起こった: {mv:?}"
                );
            }

            pos = pos_nxt;
            moves = pos.update_moves(&moves);

            // コストを加算する。
            // 最終面の場合、最終手のブロック投げコストは 1 とみなす。
            if arg.config.last_stage && i == self.len() - 1 {
                cost_total += cost_hero_move + 1;
            } else {
                cost_total += cost_hero_move + cost_throw;
            }
        }

        ensure!(
            pos.legal_moves(&moves).is_empty(),
            "最後の局面でまだ合法手がある:\n{pos}"
        );

        if arg.config.forbid_just {
            ensure!(
                pos.block_count() != arg.game_mode.block_count_target(),
                "just clear は禁止されている"
            );
        }

        ensure!(
            pos.block_count() <= arg.game_mode.block_count_target(),
            "最後の局面が解けていない:\n{pos}"
        );

        cost_total += calc_clear_cost(arg.game_mode, &pos, arg.config.last_stage);

        Ok(cost_total)
    }
}

impl std::str::FromStr for Solution {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut srcs = Vec::<GroundRow>::new();

        for (i, token) in s.split(',').enumerate() {
            let src: u8 = token
                .parse()
                .with_context(|| format!("{i} 番目の着手が数値でない: {token}"))?;
            let src =
                GroundRow::from_inner(src).ok_or_else(|| anyhow!("{i} 番目の着手が無効: {src}"))?;
            srcs.push(src);
        }

        Ok(Solution(srcs))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.srcs()
                .iter()
                .copied()
                .map(GroundRow::to_inner)
                .join(",")
        )
    }
}

pub fn solve_problem(arg: &SolverArgument) -> Option<(Solution, Cost)> {
    let mut solver = Solver::new(arg.game_mode, arg.config.clone());

    solver.solve(&arg.pos, &arg.moves, 0, 0);

    solver.best_solution.map(|moves| {
        let solution = Solution::from_moves(&moves);
        (solution, solver.best_cost)
    })
}

#[derive(Debug)]
struct Solver {
    game_mode: GameMode,
    config: SolverConfig,

    best_solution: Option<Vec<Move>>,
    best_cost: Cost,
    cur_solution: Vec<Move>,
}

impl Solver {
    fn new(game_mode: GameMode, config: SolverConfig) -> Self {
        let best_cost = config.max_cost + 1;

        Self {
            game_mode,
            config,

            best_solution: None,
            best_cost,
            cur_solution: vec![],
        }
    }

    fn solve(&mut self, pos: &Position, moves: &[Move], cost: Cost, cost_last_throw: Cost) {
        // 現局面が解けていると仮定したときの総コストを求める。
        let cost_solved = if self.config.last_stage {
            // 最終面の場合、最終手のブロック投げコストは 1 (A を押して放置するだけ) とみなす。
            // また、面クリア時の演出コストを加算しない。
            cost - cost_last_throw + 1
        } else {
            // 最終面でない場合、面クリア時の演出コストを加算する。
            cost + calc_clear_cost(self.game_mode, pos, self.config.last_stage)
        };

        // 現局面が解けていると仮定してもコストが改善しないなら枝刈り。
        if cost_solved >= self.best_cost {
            return;
        }

        // 合法手の集合を求める。
        let moves_legal = pos.legal_moves(moves);

        // 合法手がない場合、解けている/ミスのいずれかである。どちらにせよここで戻る。
        // (just clear で、かつそれが禁止されているなら単にミスとみなす)
        if moves_legal.is_empty() {
            // 解けているなら解を更新(コストが改善しないケースは事前に枝刈りされている)。
            let solved = if self.config.forbid_just {
                pos.block_count() < self.game_mode.block_count_target()
            } else {
                pos.block_count() <= self.game_mode.block_count_target()
            };
            if solved {
                self.best_solution = Some(self.cur_solution.clone());
                self.best_cost = cost_solved;
                debug!(
                    "improved: {} {}",
                    self.best_cost,
                    Solution::from_moves(&self.cur_solution)
                );
            }
            return;
        }

        for mv in moves_legal {
            let (pos_nxt, cost_hero_move, cost_throw, erase_count) = pos.do_move(mv);
            if self.config.forbid_five && erase_count >= 5 {
                continue;
            }

            let moves_nxt = pos_nxt.update_moves(moves);
            let cost_nxt = cost + cost_hero_move + cost_throw;

            self.cur_solution.push(mv);
            self.solve(&pos_nxt, &moves_nxt, cost_nxt, cost_throw);
            unsafe { self.cur_solution.remove_last_unchecked() }
        }
    }
}
