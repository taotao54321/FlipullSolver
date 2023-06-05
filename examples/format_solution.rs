use std::path::PathBuf;

use anyhow::Context as _;
use clap::{Parser, ValueEnum};

use flipull_solver::*;

/// 問題に対する解を整形して出力する。
#[derive(Debug, Parser)]
struct Cli {
    /// 最終面かどうか (コスト計算に影響)。
    #[arg(long)]
    last_stage: bool,

    /// 出力形式。
    #[arg(long, value_enum, default_value_t = Format::Pretty)]
    format: Format,

    /// 問題ファイル。
    path_problem: PathBuf,

    /// 解ファイル。
    path_solution: PathBuf,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum Format {
    /// 着手ごとに途中経過を出力する。
    Pretty,

    /// Neshawk の TAStudio にペーストできるムービーを出力する。
    Neshawk,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let problem = std::fs::read_to_string(&cli.path_problem).with_context(|| {
        format!(
            "問題ファイル '{}' を読み取れない",
            cli.path_problem.display()
        )
    })?;
    let problem: Problem = problem.parse()?;

    let solution = std::fs::read_to_string(&cli.path_solution).with_context(|| {
        format!(
            "解ファイル '{}' を読み取れない",
            cli.path_solution.display()
        )
    })?;
    let solution: Solution = solution.trim_end().parse()?; // 改行があるとパースに失敗する。

    let config = SolverConfig {
        last_stage: cli.last_stage,
        ..Default::default()
    };

    let solver_arg = SolverArgument::new(&problem, &config);

    match cli.format {
        Format::Pretty => format_pretty(&solver_arg, &solution),
        Format::Neshawk => format_neshawk(&solver_arg, &solution),
    }

    Ok(())
}

fn format_pretty(solver_arg: &SolverArgument, solution: &Solution) {
    let mut pos = solver_arg.position().clone();
    let mut moves = solver_arg.moves().clone();
    let mut cost_total = 0;

    println!("{pos}");

    for (i, &src) in solution.srcs().iter().enumerate() {
        assert_ne!(
            pos.move_count_remain(),
            0,
            "{i} 番目の着手前に残り手数が尽きた"
        );

        let mv = moves
            .iter()
            .copied()
            .find(|mv| mv.src() == src)
            .unwrap_or_else(|| panic!("{i} 番目の着手が不正: {src:?}"));
        assert!(pos.is_legal_move(mv), "{i} 番目の着手が不正: {mv:?}");
        let (pos_nxt, cost_hero_move, cost_throw, _erase_count) = pos.do_move(mv);

        pos = pos_nxt;
        moves = pos.update_moves(&moves);

        let cost_mv = if solver_arg.config().last_stage && i == solution.len() - 1 {
            cost_hero_move + 1
        } else {
            cost_hero_move + cost_throw
        };
        cost_total += cost_mv;

        println!("着手 {i}: {src:?} (cost={cost_mv})");
        println!("{pos}");
    }

    assert!(
        pos.legal_moves(&moves).is_empty(),
        "最後の局面でまだ合法手がある:\n{pos}"
    );

    assert!(
        pos.block_count() <= solver_arg.game_mode().block_count_target(),
        "最後の局面が解けていない:\n{pos}"
    );

    cost_total += calc_clear_cost(solver_arg.game_mode(), &pos, solver_arg.config().last_stage);

    println!("総コスト: {cost_total}");
}

fn format_neshawk(solver_arg: &SolverArgument, solution: &Solution) {
    let movie = solution_to_movie(solver_arg, solution);

    for input in movie {
        println!("{}", input.display_neshawk());
    }
}

fn solution_to_movie(solver_arg: &SolverArgument, solution: &Solution) -> Vec<MovieInput> {
    let mut pos = solver_arg.position().clone();
    let mut moves = solver_arg.moves().clone();

    let mut movie = Vec::<MovieInput>::new();

    for &src in solution.srcs() {
        assert_ne!(pos.move_count_remain(), 0);

        // 自機を動かして待つ。
        let inputs_hero = inputs_hero_move(pos.hero_row(), src);
        movie.extend(&inputs_hero);

        let mv = moves.iter().copied().find(|mv| mv.src() == src).unwrap();
        assert!(pos.is_legal_move(mv));
        let (pos_nxt, _cost_hero_move, cost_throw, _erase_count) = pos.do_move(mv);

        // ブロックを投げて待つ。
        // ムービーを出力する分には最終手のコスト調整は特に不要。
        let wait_len = usize::from(cost_throw) - 1;
        movie.push(MovieInput::A);
        movie.extend(vec![MovieInput::None; wait_len]);

        pos = pos_nxt;
        moves = pos.update_moves(&moves);
    }

    movie
}

fn inputs_hero_move(from: GroundRow, to: GroundRow) -> Vec<MovieInput> {
    use std::cmp::Ordering;

    const WAIT_LEN: usize = COST_HERO_STEP as usize - 1;

    let mut inputs = Vec::<MovieInput>::new();

    match from.cmp(&to) {
        Ordering::Less => {
            for _ in 0..to.to_inner() - from.to_inner() {
                inputs.push(MovieInput::Down);
                inputs.extend([MovieInput::None; WAIT_LEN]);
            }
        }
        Ordering::Greater => {
            for _ in 0..from.to_inner() - to.to_inner() {
                inputs.push(MovieInput::Up);
                inputs.extend([MovieInput::None; WAIT_LEN]);
            }
        }
        _ => {}
    }

    inputs
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MovieInput {
    None,
    A,
    Up,
    Down,
}

impl MovieInput {
    fn display_neshawk(self) -> &'static str {
        match self {
            Self::None => "|..|........|........|",
            Self::A => "|..|.......A|........|",
            Self::Up => "|..|U.......|........|",
            Self::Down => "|..|.D......|........|",
        }
    }
}
