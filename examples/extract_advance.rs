use std::path::PathBuf;

use clap::Parser;

use flipull_solver::*;

/// ADVANCE モードの問題を抽出する。
#[derive(Debug, Parser)]
struct Cli {
    /// 原作の ROM ファイル (iNES 形式)。
    path_ines: PathBuf,

    /// 面 (0..=49)。
    #[arg(value_parser = clap::value_parser!(u8).range(0..=49))]
    stage: u8,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let rom = Rom::from_ines_file(cli.path_ines)?;

    let problem = extract_advance_problem(&rom, cli.stage);

    print!("{problem}");

    Ok(())
}
