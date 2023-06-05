use byteorder::{ByteOrder as _, LE};

use crate::block::{Block, Blocks, BlocksCol, BlocksRow};
use crate::ground::{Ground, GroundCol, GroundElement, GroundRow};
use crate::problem::Problem;
use crate::rom::Rom;

/// ADVANCE モードの指定した面の問題を抽出する。
///
/// `stage` は `0..=49` でなければならない。
/// 何か変なことが起きたら panic する。
pub fn extract_advance_problem(rom: &Rom, stage: u8) -> Problem {
    assert!(stage < 50);

    let (bank, ptrs_offset) = if stage < 25 {
        (rom.chr_bank(0), 0x0A00 + 4 * usize::from(stage))
    } else {
        (rom.chr_bank(2), 0x1A00 + 4 * usize::from(stage - 25))
    };

    let (blocks, block_holding, move_count_remain) = {
        let ptr = usize::from(LE::read_u16(&bank[ptrs_offset..]) & 0x3FFF);
        let buf = &bank[ptr..][..48 + 2];

        let mut blocks = Blocks::new();
        for (row, buf) in itertools::zip_eq(BlocksRow::all(), buf.chunks_exact(8)) {
            assert!(
                buf[6..].iter().all(|&b| b == 0),
                "ブロック領域外の列にブロックがある"
            );
            for (col, &block) in itertools::zip_eq(BlocksCol::all(), &buf[..6]) {
                let block = match block {
                    0..=4 => Block::from_inner(block),
                    // ADVANCE モードでは盤面にワイルドカードが現れることはない。
                    _ => panic!("無効な盤面ブロック値: {block}"),
                };
                blocks[(col, row)] = block;
            }
        }

        let move_count_remain = buf[48];

        let block_holding = buf[49];
        let block_holding = Block::from_inner(block_holding)
            .unwrap_or_else(|| panic!("無効な保持ブロック値: {block_holding}"));

        (blocks, block_holding, move_count_remain)
    };

    let ground = {
        let ptr = usize::from(LE::read_u16(&bank[ptrs_offset + 2..]) & 0x3FFF);
        let buf = &bank[ptr..][..12 * 2];

        let mut ground = Ground::new();
        for (row, &value) in itertools::zip_eq(GroundRow::all(), &buf[..12]) {
            for col in GroundCol::all() {
                if (value & (1 << (7 - col.to_index()))) != 0 {
                    ground[(col, row)] = Some(GroundElement::Wall);
                }
            }
        }
        for (row, &value) in itertools::zip_eq(GroundRow::all(), &buf[12..]) {
            for col in GroundCol::all() {
                if (value & (1 << (7 - col.to_index()))) != 0 {
                    ground[(col, row)] = Some(GroundElement::Pipe);
                }
            }
        }

        ground
    };

    Problem::new_advance(ground, blocks, block_holding, move_count_remain)
        .expect("問題が ADVANCE モードの制約を満たしていない")
}
