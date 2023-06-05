use byteorder::{ByteOrder as _, LE};

use crate::block::{Block, Blocks, BlocksCol, BlocksRow, BlocksSquare};
use crate::ground::{Ground, GroundCol, GroundElement, GroundRow};
use crate::problem::Problem;
use crate::rom::Rom;

/// NORMAL モードの指定した (面, 乱数シード) の問題を抽出する。
///
/// 何か変なことが起きたら panic する。
pub fn extract_normal_problem(rom: &Rom, stage: u8, rng_state: [u8; 2], wild: bool) -> Problem {
    let ground = extract_normal_ground(rom, stage);
    let blocks = extract_normal_blocks(rom, stage, rng_state, wild);
    let block_count_target = extract_normal_block_count_target(rom, stage);

    Problem::new_normal(ground, blocks, block_count_target)
        .expect("問題が NORMAL モードの制約を満たしていない")
}

/// NORMAL モードの指定した (面, 乱数シード) のブロック配置を抽出する。
///
/// 何か変なことが起きたら panic する。
pub fn extract_normal_blocks(rom: &Rom, stage: u8, rng_state: [u8; 2], wild: bool) -> Blocks {
    let params_first = BlocksParams::new(stage, rng_state, false);
    let params_second = BlocksParams::new(stage, rng_state, true);

    let mut rng = {
        let carry = (stage & (1 << 1)) != 0;
        BlocksRng::new(rng_state, carry)
    };

    let mut buf = [0; 48];
    extract_blocks_part(rom, &params_first, &mut rng, &mut buf);
    extract_blocks_part(rom, &params_second, &mut rng, &mut buf);

    let mut blocks = Blocks::new();

    for (row, buf) in itertools::zip_eq(BlocksRow::all(), buf.chunks_exact(8)) {
        assert!(
            buf[6..].iter().all(|&b| b == 0),
            "ブロック領域外の列にブロックがある"
        );
        for (col, &block) in itertools::zip_eq(BlocksCol::all(), &buf[..6]) {
            let block = match block {
                0..=4 => Block::from_inner(block),
                // ここまでで生成されるブロックはワイルドカードではない。
                _ => panic!("無効な盤面ブロック値: {block}"),
            };
            blocks[(col, row)] = block;
        }
    }

    // ワイルドカードは決まった位置にしか配置されない。
    if wild {
        blocks[BlocksSquare::B5] = Some(Block::Wild);
    }

    blocks
}

fn extract_blocks_part(rom: &Rom, params: &BlocksParams, rng: &mut BlocksRng, buf: &mut [u8; 48]) {
    let mut remains = params.block_counts;

    for &idx in params.idxs {
        let block = loop {
            let block = rng.gen(rom);
            if remains[usize::from(block - 1)] > 0 {
                remains[usize::from(block - 1)] -= 1;
                break block;
            }
        };

        assert_eq!(buf[idx], 0);
        buf[idx] = block;
    }
}

#[derive(Debug)]
struct BlocksParams {
    block_counts: [u8; 4],
    idxs: &'static [usize],
}

impl BlocksParams {
    fn new(stage: u8, rng_state: [u8; 2], second: bool) -> Self {
        #[rustfmt::skip]
        const STAGE_KIND_TABLE: [usize; 0x20] = [
            0, 0, 0, 1, 1, 1, 1, 2,
            2, 2, 2, 1, 2, 2, 2, 1,
            1, 1, 1, 1, 1, 1, 1, 2,
            2, 2, 2, 2, 2, 0, 1, 1,
        ];

        const BLOCK_COUNTS_TABLE: [[[u8; 4]; 2]; 12] = [
            [[3, 2, 2, 2], [4, 4, 4, 4]],
            [[2, 3, 2, 2], [4, 4, 4, 4]],
            [[2, 2, 3, 2], [4, 4, 4, 4]],
            [[2, 2, 2, 3], [4, 4, 4, 4]],
            [[2, 2, 3, 3], [5, 5, 5, 5]],
            [[3, 3, 2, 2], [5, 5, 5, 5]],
            [[2, 3, 2, 3], [5, 5, 5, 5]],
            [[3, 2, 3, 2], [5, 5, 5, 5]],
            [[2, 3, 3, 3], [7, 6, 6, 6]],
            [[3, 2, 3, 3], [6, 7, 6, 6]],
            [[3, 3, 2, 3], [6, 6, 7, 6]],
            [[3, 3, 3, 2], [6, 6, 6, 7]],
        ];

        #[rustfmt::skip]
        const IDXS_TABLE: [[&[usize]; 2]; 3] = [
            [
                &[8, 9, 10, 11, 12, 20, 28, 36, 44],
                &[16, 17, 18, 19, 24, 25, 26, 27, 32, 33, 34, 35, 40, 41, 42, 43],
            ],
            [
                &[0, 1, 2, 3, 4, 12, 20, 28, 36, 44],
                &[8, 9, 10, 11, 16, 17, 18, 19, 24, 25, 26, 27, 32, 33, 34, 35, 40, 41, 42, 43],
            ],
            [
                &[0, 1, 2, 3, 4, 5, 13, 21, 29, 37, 45],
                &[8, 9, 10, 11, 12, 16, 17, 18, 19, 20, 24, 25, 26, 27, 28, 32, 33, 34, 35, 36, 40, 41, 42, 43, 44],
            ],
        ];

        let stage_kind = STAGE_KIND_TABLE[usize::from(stage % 32)];

        let block_counts = {
            let idx = (stage_kind << 2) | usize::from((rng_state[0] >> 2) & 3);
            BLOCK_COUNTS_TABLE[idx][usize::from(second)]
        };

        let idxs = IDXS_TABLE[stage_kind][usize::from(second)];

        Self { block_counts, idxs }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BlocksRng {
    state: [u8; 2],
    ptr: u16,
}

impl BlocksRng {
    fn new(state: [u8; 2], carry: bool) -> Self {
        let ptr = 0x8000 | (u16::from(state[0] & 0x1F) << 8) | u16::from(ror4(state[1], carry));

        Self { state, ptr }
    }

    fn gen(&mut self, rom: &Rom) -> u8 {
        self.state[0] = self.state[0].wrapping_add(1);
        self.state[1] = self.state[1].wrapping_sub(1);

        let x = rom.prg()[usize::from(self.ptr - 0x8000)];

        self.ptr = {
            let rhs = u16::from_le_bytes(self.state);
            0x8000 | (self.ptr.wrapping_add(rhs) & 0x1FFF)
        };

        let ptr = {
            let lhs = u16::from_be_bytes(self.state);
            0x8000 | (lhs.wrapping_sub(self.ptr) & 0x1FFF)
        };
        let y = rom.prg()[usize::from(ptr - 0x8000)];

        let shift = 2 * ((y >> 1) & 3);

        1 + ((x >> shift) & 3)
    }
}

fn ror4(x: u8, carry: bool) -> u8 {
    (x >> 4) | ((x & 7) << 5) | (u8::from(carry) << 4)
}

/// NORMAL モードの指定した面の壁/パイプ配置を抽出する。
///
/// 何か変なことが起きたら panic する。
pub fn extract_normal_ground(rom: &Rom, stage: u8) -> Ground {
    let (bank, ptr_offset) = if stage % 32 < 25 {
        (rom.chr_bank(2), 0x0C00 + 2 * usize::from(stage % 32))
    } else {
        (rom.chr_bank(0), 0x1C00 + 2 * usize::from(stage % 32 - 25))
    };

    let ptr = usize::from(LE::read_u16(&bank[ptr_offset..]) & 0x3FFF);
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
}

/// NORMAL モードの指定した面のブロック規定数を抽出する。
///
/// 何か変なことが起きたら panic する。
pub fn extract_normal_block_count_target(rom: &Rom, stage: u8) -> u8 {
    let buf = &rom.prg()[0x298D..][..32];

    buf[usize::from(stage % 32)]
}
