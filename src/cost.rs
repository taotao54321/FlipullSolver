use std::num::NonZeroU16;

use crate::block::BlocksSquare;
use crate::game_mode::GameMode;
use crate::ground::GroundRow;
use crate::macros::unreachable_unchecked;
use crate::position::Position;

/// 所要フレーム数を表す型。
pub type Cost = u16;

type NonZeroCost = NonZeroU16;

/// コスト無限大を表す値。
pub const COST_INF: Cost = 30000; // 8 分以上かかるようなケースは扱わないので、これで十分だろう。

/// 自機が 1 歩移動するのにかかるコスト。
pub const COST_HERO_STEP: Cost = 16;

/// 面クリア時のブロック消去演出のコスト (1 個あたり)。
pub const COST_CLEAR_ERASE_BLOCK: Cost = 11;

/// NORMAL モードでパーフェクトで面クリアした際の花火演出のコスト。
pub const COST_CLEAR_FIREWORKS: Cost = 96;

// NOTE: NORMAL モードでの just clear には追加コストがかからない。

/// 自機の移動コストを求める。
pub const fn calc_hero_move_cost(from: GroundRow, to: GroundRow) -> Cost {
    let d = from.to_inner().abs_diff(to.to_inner()) as Cost;

    COST_HERO_STEP * d
}

/// ブロック投げコストを求める。
/// ブロックを投げた位置を `src`、置換前に最後にブロックが通った位置を `sq_last` とする。
pub const fn calc_throw_cost(src: GroundRow, sq_last: BlocksSquare) -> Cost {
    const LEN_SRC: usize = GroundRow::NUM;
    const LEN_SQ: usize = (BlocksSquare::MAX_VALUE + 1) as usize;

    const TABLE: [[Option<NonZeroCost>; LEN_SQ]; LEN_SRC] = {
        use crate::ground::*;

        let mut res = [[None; LEN_SQ]; LEN_SRC];

        macro_rules! set {
            ($src:expr, $sq:expr, $cost:expr) => {{
                let cost = unsafe { NonZeroCost::new_unchecked($cost) };
                res[$src.to_index()][$sq.to_index()] = Some(cost);
            }};
        }

        set!(GROUND_ROW_1, BlocksSquare::A1, 107);
        set!(GROUND_ROW_1, BlocksSquare::A2, 113);
        set!(GROUND_ROW_1, BlocksSquare::A3, 119);
        set!(GROUND_ROW_1, BlocksSquare::A4, 125);
        set!(GROUND_ROW_1, BlocksSquare::A5, 131);
        set!(GROUND_ROW_1, BlocksSquare::A6, 132);
        set!(GROUND_ROW_1, BlocksSquare::B1, 103);
        set!(GROUND_ROW_1, BlocksSquare::B2, 109);
        set!(GROUND_ROW_1, BlocksSquare::B3, 115);
        set!(GROUND_ROW_1, BlocksSquare::B4, 121);
        set!(GROUND_ROW_1, BlocksSquare::B5, 127);
        set!(GROUND_ROW_1, BlocksSquare::B6, 128);
        set!(GROUND_ROW_1, BlocksSquare::C1, 99);
        set!(GROUND_ROW_1, BlocksSquare::C2, 105);
        set!(GROUND_ROW_1, BlocksSquare::C3, 111);
        set!(GROUND_ROW_1, BlocksSquare::C4, 117);
        set!(GROUND_ROW_1, BlocksSquare::C5, 123);
        set!(GROUND_ROW_1, BlocksSquare::C6, 124);
        set!(GROUND_ROW_1, BlocksSquare::D1, 95);
        set!(GROUND_ROW_1, BlocksSquare::D2, 101);
        set!(GROUND_ROW_1, BlocksSquare::D3, 107);
        set!(GROUND_ROW_1, BlocksSquare::D4, 113);
        set!(GROUND_ROW_1, BlocksSquare::D5, 119);
        set!(GROUND_ROW_1, BlocksSquare::D6, 120);
        set!(GROUND_ROW_1, BlocksSquare::E1, 91);
        set!(GROUND_ROW_1, BlocksSquare::E2, 97);
        set!(GROUND_ROW_1, BlocksSquare::E3, 103);
        set!(GROUND_ROW_1, BlocksSquare::E4, 109);
        set!(GROUND_ROW_1, BlocksSquare::E5, 115);
        set!(GROUND_ROW_1, BlocksSquare::E6, 116);
        set!(GROUND_ROW_1, BlocksSquare::F1, 87);
        set!(GROUND_ROW_1, BlocksSquare::F2, 93);
        set!(GROUND_ROW_1, BlocksSquare::F3, 99);
        set!(GROUND_ROW_1, BlocksSquare::F4, 105);
        set!(GROUND_ROW_1, BlocksSquare::F5, 111);
        set!(GROUND_ROW_1, BlocksSquare::F6, 112);
        set!(GROUND_ROW_2, BlocksSquare::A1, 104);
        set!(GROUND_ROW_2, BlocksSquare::A2, 110);
        set!(GROUND_ROW_2, BlocksSquare::A3, 116);
        set!(GROUND_ROW_2, BlocksSquare::A4, 122);
        set!(GROUND_ROW_2, BlocksSquare::A5, 128);
        set!(GROUND_ROW_2, BlocksSquare::A6, 129);
        set!(GROUND_ROW_2, BlocksSquare::B1, 100);
        set!(GROUND_ROW_2, BlocksSquare::B2, 106);
        set!(GROUND_ROW_2, BlocksSquare::B3, 112);
        set!(GROUND_ROW_2, BlocksSquare::B4, 118);
        set!(GROUND_ROW_2, BlocksSquare::B5, 124);
        set!(GROUND_ROW_2, BlocksSquare::B6, 125);
        set!(GROUND_ROW_2, BlocksSquare::C1, 96);
        set!(GROUND_ROW_2, BlocksSquare::C2, 102);
        set!(GROUND_ROW_2, BlocksSquare::C3, 108);
        set!(GROUND_ROW_2, BlocksSquare::C4, 114);
        set!(GROUND_ROW_2, BlocksSquare::C5, 120);
        set!(GROUND_ROW_2, BlocksSquare::C6, 121);
        set!(GROUND_ROW_2, BlocksSquare::D1, 92);
        set!(GROUND_ROW_2, BlocksSquare::D2, 98);
        set!(GROUND_ROW_2, BlocksSquare::D3, 104);
        set!(GROUND_ROW_2, BlocksSquare::D4, 110);
        set!(GROUND_ROW_2, BlocksSquare::D5, 116);
        set!(GROUND_ROW_2, BlocksSquare::D6, 117);
        set!(GROUND_ROW_2, BlocksSquare::E1, 88);
        set!(GROUND_ROW_2, BlocksSquare::E2, 94);
        set!(GROUND_ROW_2, BlocksSquare::E3, 100);
        set!(GROUND_ROW_2, BlocksSquare::E4, 106);
        set!(GROUND_ROW_2, BlocksSquare::E5, 112);
        set!(GROUND_ROW_2, BlocksSquare::E6, 113);
        set!(GROUND_ROW_2, BlocksSquare::F1, 84);
        set!(GROUND_ROW_2, BlocksSquare::F2, 90);
        set!(GROUND_ROW_2, BlocksSquare::F3, 96);
        set!(GROUND_ROW_2, BlocksSquare::F4, 102);
        set!(GROUND_ROW_2, BlocksSquare::F5, 108);
        set!(GROUND_ROW_2, BlocksSquare::F6, 109);
        set!(GROUND_ROW_3, BlocksSquare::A1, 100);
        set!(GROUND_ROW_3, BlocksSquare::A2, 106);
        set!(GROUND_ROW_3, BlocksSquare::A3, 112);
        set!(GROUND_ROW_3, BlocksSquare::A4, 118);
        set!(GROUND_ROW_3, BlocksSquare::A5, 124);
        set!(GROUND_ROW_3, BlocksSquare::A6, 125);
        set!(GROUND_ROW_3, BlocksSquare::B1, 96);
        set!(GROUND_ROW_3, BlocksSquare::B2, 102);
        set!(GROUND_ROW_3, BlocksSquare::B3, 108);
        set!(GROUND_ROW_3, BlocksSquare::B4, 114);
        set!(GROUND_ROW_3, BlocksSquare::B5, 120);
        set!(GROUND_ROW_3, BlocksSquare::B6, 121);
        set!(GROUND_ROW_3, BlocksSquare::C1, 92);
        set!(GROUND_ROW_3, BlocksSquare::C2, 98);
        set!(GROUND_ROW_3, BlocksSquare::C3, 104);
        set!(GROUND_ROW_3, BlocksSquare::C4, 110);
        set!(GROUND_ROW_3, BlocksSquare::C5, 116);
        set!(GROUND_ROW_3, BlocksSquare::C6, 117);
        set!(GROUND_ROW_3, BlocksSquare::D1, 88);
        set!(GROUND_ROW_3, BlocksSquare::D2, 94);
        set!(GROUND_ROW_3, BlocksSquare::D3, 100);
        set!(GROUND_ROW_3, BlocksSquare::D4, 106);
        set!(GROUND_ROW_3, BlocksSquare::D5, 112);
        set!(GROUND_ROW_3, BlocksSquare::D6, 113);
        set!(GROUND_ROW_3, BlocksSquare::E1, 84);
        set!(GROUND_ROW_3, BlocksSquare::E2, 90);
        set!(GROUND_ROW_3, BlocksSquare::E3, 96);
        set!(GROUND_ROW_3, BlocksSquare::E4, 102);
        set!(GROUND_ROW_3, BlocksSquare::E5, 108);
        set!(GROUND_ROW_3, BlocksSquare::E6, 109);
        set!(GROUND_ROW_3, BlocksSquare::F1, 80);
        set!(GROUND_ROW_3, BlocksSquare::F2, 86);
        set!(GROUND_ROW_3, BlocksSquare::F3, 92);
        set!(GROUND_ROW_3, BlocksSquare::F4, 98);
        set!(GROUND_ROW_3, BlocksSquare::F5, 104);
        set!(GROUND_ROW_3, BlocksSquare::F6, 105);
        set!(GROUND_ROW_4, BlocksSquare::A1, 96);
        set!(GROUND_ROW_4, BlocksSquare::A2, 102);
        set!(GROUND_ROW_4, BlocksSquare::A3, 108);
        set!(GROUND_ROW_4, BlocksSquare::A4, 114);
        set!(GROUND_ROW_4, BlocksSquare::A5, 120);
        set!(GROUND_ROW_4, BlocksSquare::A6, 121);
        set!(GROUND_ROW_4, BlocksSquare::B1, 92);
        set!(GROUND_ROW_4, BlocksSquare::B2, 98);
        set!(GROUND_ROW_4, BlocksSquare::B3, 104);
        set!(GROUND_ROW_4, BlocksSquare::B4, 110);
        set!(GROUND_ROW_4, BlocksSquare::B5, 116);
        set!(GROUND_ROW_4, BlocksSquare::B6, 117);
        set!(GROUND_ROW_4, BlocksSquare::C1, 88);
        set!(GROUND_ROW_4, BlocksSquare::C2, 94);
        set!(GROUND_ROW_4, BlocksSquare::C3, 100);
        set!(GROUND_ROW_4, BlocksSquare::C4, 106);
        set!(GROUND_ROW_4, BlocksSquare::C5, 112);
        set!(GROUND_ROW_4, BlocksSquare::C6, 113);
        set!(GROUND_ROW_4, BlocksSquare::D1, 84);
        set!(GROUND_ROW_4, BlocksSquare::D2, 90);
        set!(GROUND_ROW_4, BlocksSquare::D3, 96);
        set!(GROUND_ROW_4, BlocksSquare::D4, 102);
        set!(GROUND_ROW_4, BlocksSquare::D5, 108);
        set!(GROUND_ROW_4, BlocksSquare::D6, 109);
        set!(GROUND_ROW_4, BlocksSquare::E1, 80);
        set!(GROUND_ROW_4, BlocksSquare::E2, 86);
        set!(GROUND_ROW_4, BlocksSquare::E3, 92);
        set!(GROUND_ROW_4, BlocksSquare::E4, 98);
        set!(GROUND_ROW_4, BlocksSquare::E5, 104);
        set!(GROUND_ROW_4, BlocksSquare::E6, 105);
        set!(GROUND_ROW_4, BlocksSquare::F1, 76);
        set!(GROUND_ROW_4, BlocksSquare::F2, 82);
        set!(GROUND_ROW_4, BlocksSquare::F3, 88);
        set!(GROUND_ROW_4, BlocksSquare::F4, 94);
        set!(GROUND_ROW_4, BlocksSquare::F5, 100);
        set!(GROUND_ROW_4, BlocksSquare::F6, 101);
        set!(GROUND_ROW_5, BlocksSquare::A1, 92);
        set!(GROUND_ROW_5, BlocksSquare::A2, 98);
        set!(GROUND_ROW_5, BlocksSquare::A3, 104);
        set!(GROUND_ROW_5, BlocksSquare::A4, 110);
        set!(GROUND_ROW_5, BlocksSquare::A5, 116);
        set!(GROUND_ROW_5, BlocksSquare::A6, 117);
        set!(GROUND_ROW_5, BlocksSquare::B1, 88);
        set!(GROUND_ROW_5, BlocksSquare::B2, 94);
        set!(GROUND_ROW_5, BlocksSquare::B3, 100);
        set!(GROUND_ROW_5, BlocksSquare::B4, 106);
        set!(GROUND_ROW_5, BlocksSquare::B5, 112);
        set!(GROUND_ROW_5, BlocksSquare::B6, 113);
        set!(GROUND_ROW_5, BlocksSquare::C1, 84);
        set!(GROUND_ROW_5, BlocksSquare::C2, 90);
        set!(GROUND_ROW_5, BlocksSquare::C3, 96);
        set!(GROUND_ROW_5, BlocksSquare::C4, 102);
        set!(GROUND_ROW_5, BlocksSquare::C5, 108);
        set!(GROUND_ROW_5, BlocksSquare::C6, 109);
        set!(GROUND_ROW_5, BlocksSquare::D1, 80);
        set!(GROUND_ROW_5, BlocksSquare::D2, 86);
        set!(GROUND_ROW_5, BlocksSquare::D3, 92);
        set!(GROUND_ROW_5, BlocksSquare::D4, 98);
        set!(GROUND_ROW_5, BlocksSquare::D5, 104);
        set!(GROUND_ROW_5, BlocksSquare::D6, 105);
        set!(GROUND_ROW_5, BlocksSquare::E1, 76);
        set!(GROUND_ROW_5, BlocksSquare::E2, 82);
        set!(GROUND_ROW_5, BlocksSquare::E3, 88);
        set!(GROUND_ROW_5, BlocksSquare::E4, 94);
        set!(GROUND_ROW_5, BlocksSquare::E5, 100);
        set!(GROUND_ROW_5, BlocksSquare::E6, 101);
        set!(GROUND_ROW_5, BlocksSquare::F1, 72);
        set!(GROUND_ROW_5, BlocksSquare::F2, 78);
        set!(GROUND_ROW_5, BlocksSquare::F3, 84);
        set!(GROUND_ROW_5, BlocksSquare::F4, 90);
        set!(GROUND_ROW_5, BlocksSquare::F5, 96);
        set!(GROUND_ROW_5, BlocksSquare::F6, 97);
        set!(GROUND_ROW_6, BlocksSquare::A1, 87);
        set!(GROUND_ROW_6, BlocksSquare::A2, 93);
        set!(GROUND_ROW_6, BlocksSquare::A3, 99);
        set!(GROUND_ROW_6, BlocksSquare::A4, 105);
        set!(GROUND_ROW_6, BlocksSquare::A5, 111);
        set!(GROUND_ROW_6, BlocksSquare::A6, 112);
        set!(GROUND_ROW_6, BlocksSquare::B1, 83);
        set!(GROUND_ROW_6, BlocksSquare::B2, 89);
        set!(GROUND_ROW_6, BlocksSquare::B3, 95);
        set!(GROUND_ROW_6, BlocksSquare::B4, 101);
        set!(GROUND_ROW_6, BlocksSquare::B5, 107);
        set!(GROUND_ROW_6, BlocksSquare::B6, 108);
        set!(GROUND_ROW_6, BlocksSquare::C1, 79);
        set!(GROUND_ROW_6, BlocksSquare::C2, 85);
        set!(GROUND_ROW_6, BlocksSquare::C3, 91);
        set!(GROUND_ROW_6, BlocksSquare::C4, 97);
        set!(GROUND_ROW_6, BlocksSquare::C5, 103);
        set!(GROUND_ROW_6, BlocksSquare::C6, 104);
        set!(GROUND_ROW_6, BlocksSquare::D1, 75);
        set!(GROUND_ROW_6, BlocksSquare::D2, 81);
        set!(GROUND_ROW_6, BlocksSquare::D3, 87);
        set!(GROUND_ROW_6, BlocksSquare::D4, 93);
        set!(GROUND_ROW_6, BlocksSquare::D5, 99);
        set!(GROUND_ROW_6, BlocksSquare::D6, 100);
        set!(GROUND_ROW_6, BlocksSquare::E1, 71);
        set!(GROUND_ROW_6, BlocksSquare::E2, 77);
        set!(GROUND_ROW_6, BlocksSquare::E3, 83);
        set!(GROUND_ROW_6, BlocksSquare::E4, 89);
        set!(GROUND_ROW_6, BlocksSquare::E5, 95);
        set!(GROUND_ROW_6, BlocksSquare::E6, 96);
        set!(GROUND_ROW_6, BlocksSquare::F1, 67);
        set!(GROUND_ROW_6, BlocksSquare::F2, 73);
        set!(GROUND_ROW_6, BlocksSquare::F3, 79);
        set!(GROUND_ROW_6, BlocksSquare::F4, 85);
        set!(GROUND_ROW_6, BlocksSquare::F5, 91);
        set!(GROUND_ROW_6, BlocksSquare::F6, 92);
        set!(GROUND_ROW_7, BlocksSquare::A1, 83);
        set!(GROUND_ROW_7, BlocksSquare::A2, 89);
        set!(GROUND_ROW_7, BlocksSquare::A3, 95);
        set!(GROUND_ROW_7, BlocksSquare::A4, 101);
        set!(GROUND_ROW_7, BlocksSquare::A5, 107);
        set!(GROUND_ROW_7, BlocksSquare::A6, 108);
        set!(GROUND_ROW_7, BlocksSquare::B1, 77);
        set!(GROUND_ROW_7, BlocksSquare::C1, 73);
        set!(GROUND_ROW_7, BlocksSquare::D1, 69);
        set!(GROUND_ROW_7, BlocksSquare::E1, 65);
        set!(GROUND_ROW_7, BlocksSquare::F1, 61);
        set!(GROUND_ROW_8, BlocksSquare::A2, 85);
        set!(GROUND_ROW_8, BlocksSquare::A3, 91);
        set!(GROUND_ROW_8, BlocksSquare::A4, 97);
        set!(GROUND_ROW_8, BlocksSquare::A5, 103);
        set!(GROUND_ROW_8, BlocksSquare::A6, 104);
        set!(GROUND_ROW_8, BlocksSquare::B2, 79);
        set!(GROUND_ROW_8, BlocksSquare::C2, 75);
        set!(GROUND_ROW_8, BlocksSquare::D2, 71);
        set!(GROUND_ROW_8, BlocksSquare::E2, 67);
        set!(GROUND_ROW_8, BlocksSquare::F2, 63);
        set!(GROUND_ROW_9, BlocksSquare::A3, 87);
        set!(GROUND_ROW_9, BlocksSquare::A4, 93);
        set!(GROUND_ROW_9, BlocksSquare::A5, 99);
        set!(GROUND_ROW_9, BlocksSquare::A6, 100);
        set!(GROUND_ROW_9, BlocksSquare::B3, 81);
        set!(GROUND_ROW_9, BlocksSquare::C3, 77);
        set!(GROUND_ROW_9, BlocksSquare::D3, 73);
        set!(GROUND_ROW_9, BlocksSquare::E3, 69);
        set!(GROUND_ROW_9, BlocksSquare::F3, 65);
        set!(GROUND_ROW_10, BlocksSquare::A4, 89);
        set!(GROUND_ROW_10, BlocksSquare::A5, 95);
        set!(GROUND_ROW_10, BlocksSquare::A6, 96);
        set!(GROUND_ROW_10, BlocksSquare::B4, 83);
        set!(GROUND_ROW_10, BlocksSquare::C4, 79);
        set!(GROUND_ROW_10, BlocksSquare::D4, 75);
        set!(GROUND_ROW_10, BlocksSquare::E4, 71);
        set!(GROUND_ROW_10, BlocksSquare::F4, 67);
        set!(GROUND_ROW_11, BlocksSquare::A5, 91);
        set!(GROUND_ROW_11, BlocksSquare::A6, 92);
        set!(GROUND_ROW_11, BlocksSquare::B5, 85);
        set!(GROUND_ROW_11, BlocksSquare::C5, 81);
        set!(GROUND_ROW_11, BlocksSquare::D5, 77);
        set!(GROUND_ROW_11, BlocksSquare::E5, 73);
        set!(GROUND_ROW_11, BlocksSquare::F5, 69);
        set!(GROUND_ROW_12, BlocksSquare::A6, 88);
        set!(GROUND_ROW_12, BlocksSquare::B6, 87);
        set!(GROUND_ROW_12, BlocksSquare::C6, 83);
        set!(GROUND_ROW_12, BlocksSquare::D6, 79);
        set!(GROUND_ROW_12, BlocksSquare::E6, 75);
        set!(GROUND_ROW_12, BlocksSquare::F6, 71);

        res
    };

    let cost = match TABLE[src.to_index()][sq_last.to_index()] {
        Some(cost) => cost,
        None => unsafe { unreachable_unchecked!() },
    };

    cost.get()
}

pub fn calc_clear_cost(game_mode: GameMode, pos: &Position, last_stage: bool) -> Cost {
    const BLOCK_COUNT_PERFECT: u8 = 3;

    if last_stage {
        0
    } else {
        let cost_erase = COST_CLEAR_ERASE_BLOCK * Cost::from(pos.block_count());
        let cost_fireworks = if game_mode.is_normal() && pos.block_count() <= BLOCK_COUNT_PERFECT {
            COST_CLEAR_FIREWORKS
        } else {
            0
        };
        cost_erase + cost_fireworks
    }
}
