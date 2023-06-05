use std::path::PathBuf;

use clap::Parser;

use flipull_solver::*;

/// NORMAL モードの問題を抽出する。
#[derive(Debug, Parser)]
struct Cli {
    /// 盤面にワイルドカードを配置するかどうか。
    #[arg(long)]
    wild: bool,

    /// 原作の ROM ファイル (iNES 形式)。
    path_ines: PathBuf,

    /// 面 (0-based)。
    stage: u8,

    /// 乱数シード (u16, ビッグエンディアン)。
    #[arg(value_parser = parse_int::parse::<u16>)]
    rng_state: u16,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let rom = Rom::from_ines_file(cli.path_ines)?;

    let problem = extract_normal_problem(&rom, cli.stage, cli.rng_state.to_be_bytes(), cli.wild);

    print!("{problem}");

    Ok(())
}
