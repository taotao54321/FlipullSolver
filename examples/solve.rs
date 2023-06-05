use std::path::PathBuf;

use anyhow::{ensure, Context as _};
use clap::Parser;
use log::info;

use flipull_solver::*;

/// 与えられた問題に対する実時間最速の解を求める。
#[derive(Debug, Parser)]
struct Cli {
    /// 最終面かどうか。
    #[arg(long)]
    last_stage: bool,

    /// 5 個以上の同時消しを禁止するかどうか。
    #[arg(long)]
    forbid_five: bool,

    /// just clear を禁止するかどうか。
    #[arg(long)]
    forbid_just: bool,

    /// 問題ファイル。
    path_problem: PathBuf,
}

fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let cli = Cli::parse();

    let problem = std::fs::read_to_string(&cli.path_problem).with_context(|| {
        format!(
            "問題ファイル '{}' を読み取れない",
            cli.path_problem.display()
        )
    })?;
    let problem: Problem = problem.parse()?;

    let config = SolverConfig {
        last_stage: cli.last_stage,
        forbid_five: cli.forbid_five,
        forbid_just: cli.forbid_just,
        ..Default::default()
    };

    let solver_arg = SolverArgument::new(&problem, &config);
    if let Some((solution, cost)) = solve_problem(&solver_arg) {
        println!("{solution}");

        let cost_verify = solution
            .verify(&solver_arg)
            .context("最適解の verify に失敗")?;
        ensure!(
            cost_verify == cost,
            "最適解の verify に失敗: コスト不一致 (solve: {cost}, verify: {cost_verify})"
        );
    } else {
        info!("NO SOLUTION FOUND");
    }

    Ok(())
}
