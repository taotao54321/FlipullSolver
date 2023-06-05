use std::path::PathBuf;

use clap::Parser;
use log::{info, warn};

use flipull_solver::*;

/// NORMAL モードの指定した面について乱数シードを全探索し、実時間が早い順に最大 k 個の解を求める。
#[derive(Debug, Parser)]
struct Cli {
    /// 盤面にワイルドカードを配置するかどうか。
    #[arg(long)]
    wild: bool,

    /// 最終面かどうか。
    #[arg(long)]
    last_stage: bool,

    /// 5 個以上の同時消しを禁止するかどうか。
    #[arg(long)]
    forbid_five: bool,

    /// just clear を禁止するかどうか。
    #[arg(long)]
    forbid_just: bool,

    /// 最大コスト (枝刈り用)。
    #[arg(long, default_value_t = COST_INF)]
    max_cost: Cost,

    /// 原作の ROM ファイル (iNES 形式)。
    path_ines: PathBuf,

    /// 面 (0-based)。
    stage: u8,

    /// 求める解の個数。
    k: usize,
}

fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cli = Cli::parse();

    let rom = Rom::from_ines_file(cli.path_ines)?;

    let mut config = SolverConfig {
        last_stage: cli.last_stage,
        forbid_five: cli.forbid_five,
        forbid_just: cli.forbid_just,
        max_cost: cli.max_cost,
    };

    let mut bests = BoundedHeap::<HeapElement>::new(cli.k);

    for (rng_state, problem) in generate_problems(&rom, cli.stage, cli.wild) {
        let solver_arg = SolverArgument::new(&problem, &config);
        if let Some((solution, cost)) = solve_problem(&solver_arg) {
            info!("solution: {rng_state:#06X} {cost} {solution}");

            let elt = HeapElement {
                rng_state,
                solution,
                cost,
            };
            bests.insert(elt);

            // 解が k 個出揃ったら、以降は max_cost を更新できる。
            if bests.is_full() {
                let elt = bests.peek().unwrap();
                if elt.cost < config.max_cost {
                    config.max_cost = elt.cost;
                    info!("max_cost: {}", config.max_cost);
                }
            }
        }
    }

    for elt in bests.into_sorted_vec() {
        let HeapElement {
            rng_state,
            solution,
            cost,
        } = elt;
        println!("{rng_state:#06X} {cost} {solution}");

        let problem = extract_normal_problem(&rom, cli.stage, rng_state.to_be_bytes(), cli.wild);
        let solver_arg = SolverArgument::new(&problem, &config);
        match solution.verify(&solver_arg) {
            Ok(cost_verify) => {
                if cost_verify != cost {
                    warn!("最適解の verify に失敗: コスト不一致 (solve: {cost}, verify: {cost_verify}");
                }
            }
            Err(e) => warn!("最適解の verify に失敗: {e}"),
        }
    }

    Ok(())
}

fn generate_problems(
    rom: &Rom,
    stage: u8,
    wild: bool,
) -> impl Iterator<Item = (u16, Problem)> + ExactSizeIterator + std::iter::FusedIterator + '_ {
    let ground = extract_normal_ground(rom, stage);
    let block_count_target = extract_normal_block_count_target(rom, stage);

    (0..=u16::MAX).map(move |rng_state| {
        let blocks = extract_normal_blocks(rom, stage, rng_state.to_be_bytes(), wild);
        let problem = Problem::new_normal(ground.clone(), blocks, block_count_target)
            .expect("問題が NORMAL モードの制約を満たしていない");
        (rng_state, problem)
    })
}

#[derive(Debug)]
struct HeapElement {
    rng_state: u16,
    solution: Solution,
    cost: Cost,
}

impl PartialEq for HeapElement {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for HeapElement {}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}
