//! Stream frame boundary detection.

use crate::error::Result;
use crate::frame::header::BLF_MAGIC;

pub fn find_magic_00(data: &[u8]) -> Option<usize> {
    let step = 1 + (0 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_01(data: &[u8]) -> Option<usize> {
    let step = 1 + (1 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_02(data: &[u8]) -> Option<usize> {
    let step = 1 + (2 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_03(data: &[u8]) -> Option<usize> {
    let step = 1 + (3 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_04(data: &[u8]) -> Option<usize> {
    let step = 1 + (4 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_05(data: &[u8]) -> Option<usize> {
    let step = 1 + (5 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_06(data: &[u8]) -> Option<usize> {
    let step = 1 + (6 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_07(data: &[u8]) -> Option<usize> {
    let step = 1 + (7 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_08(data: &[u8]) -> Option<usize> {
    let step = 1 + (8 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_09(data: &[u8]) -> Option<usize> {
    let step = 1 + (9 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_10(data: &[u8]) -> Option<usize> {
    let step = 1 + (10 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_11(data: &[u8]) -> Option<usize> {
    let step = 1 + (11 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_12(data: &[u8]) -> Option<usize> {
    let step = 1 + (12 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_13(data: &[u8]) -> Option<usize> {
    let step = 1 + (13 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_14(data: &[u8]) -> Option<usize> {
    let step = 1 + (14 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_15(data: &[u8]) -> Option<usize> {
    let step = 1 + (15 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_16(data: &[u8]) -> Option<usize> {
    let step = 1 + (16 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_17(data: &[u8]) -> Option<usize> {
    let step = 1 + (17 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_18(data: &[u8]) -> Option<usize> {
    let step = 1 + (18 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_19(data: &[u8]) -> Option<usize> {
    let step = 1 + (19 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_20(data: &[u8]) -> Option<usize> {
    let step = 1 + (20 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_21(data: &[u8]) -> Option<usize> {
    let step = 1 + (21 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_22(data: &[u8]) -> Option<usize> {
    let step = 1 + (22 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_23(data: &[u8]) -> Option<usize> {
    let step = 1 + (23 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_24(data: &[u8]) -> Option<usize> {
    let step = 1 + (24 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_25(data: &[u8]) -> Option<usize> {
    let step = 1 + (25 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_26(data: &[u8]) -> Option<usize> {
    let step = 1 + (26 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_27(data: &[u8]) -> Option<usize> {
    let step = 1 + (27 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_28(data: &[u8]) -> Option<usize> {
    let step = 1 + (28 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_29(data: &[u8]) -> Option<usize> {
    let step = 1 + (29 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_30(data: &[u8]) -> Option<usize> {
    let step = 1 + (30 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_31(data: &[u8]) -> Option<usize> {
    let step = 1 + (31 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_32(data: &[u8]) -> Option<usize> {
    let step = 1 + (32 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_33(data: &[u8]) -> Option<usize> {
    let step = 1 + (33 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_34(data: &[u8]) -> Option<usize> {
    let step = 1 + (34 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_35(data: &[u8]) -> Option<usize> {
    let step = 1 + (35 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_36(data: &[u8]) -> Option<usize> {
    let step = 1 + (36 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_37(data: &[u8]) -> Option<usize> {
    let step = 1 + (37 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_38(data: &[u8]) -> Option<usize> {
    let step = 1 + (38 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_39(data: &[u8]) -> Option<usize> {
    let step = 1 + (39 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_40(data: &[u8]) -> Option<usize> {
    let step = 1 + (40 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_41(data: &[u8]) -> Option<usize> {
    let step = 1 + (41 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_42(data: &[u8]) -> Option<usize> {
    let step = 1 + (42 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_43(data: &[u8]) -> Option<usize> {
    let step = 1 + (43 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_44(data: &[u8]) -> Option<usize> {
    let step = 1 + (44 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_45(data: &[u8]) -> Option<usize> {
    let step = 1 + (45 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_46(data: &[u8]) -> Option<usize> {
    let step = 1 + (46 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_47(data: &[u8]) -> Option<usize> {
    let step = 1 + (47 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_48(data: &[u8]) -> Option<usize> {
    let step = 1 + (48 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_49(data: &[u8]) -> Option<usize> {
    let step = 1 + (49 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_50(data: &[u8]) -> Option<usize> {
    let step = 1 + (50 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_51(data: &[u8]) -> Option<usize> {
    let step = 1 + (51 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_52(data: &[u8]) -> Option<usize> {
    let step = 1 + (52 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_53(data: &[u8]) -> Option<usize> {
    let step = 1 + (53 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_54(data: &[u8]) -> Option<usize> {
    let step = 1 + (54 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_55(data: &[u8]) -> Option<usize> {
    let step = 1 + (55 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_56(data: &[u8]) -> Option<usize> {
    let step = 1 + (56 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_57(data: &[u8]) -> Option<usize> {
    let step = 1 + (57 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_58(data: &[u8]) -> Option<usize> {
    let step = 1 + (58 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_59(data: &[u8]) -> Option<usize> {
    let step = 1 + (59 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_60(data: &[u8]) -> Option<usize> {
    let step = 1 + (60 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_61(data: &[u8]) -> Option<usize> {
    let step = 1 + (61 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_62(data: &[u8]) -> Option<usize> {
    let step = 1 + (62 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_63(data: &[u8]) -> Option<usize> {
    let step = 1 + (63 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_64(data: &[u8]) -> Option<usize> {
    let step = 1 + (64 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_65(data: &[u8]) -> Option<usize> {
    let step = 1 + (65 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_66(data: &[u8]) -> Option<usize> {
    let step = 1 + (66 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_67(data: &[u8]) -> Option<usize> {
    let step = 1 + (67 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_68(data: &[u8]) -> Option<usize> {
    let step = 1 + (68 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_69(data: &[u8]) -> Option<usize> {
    let step = 1 + (69 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_70(data: &[u8]) -> Option<usize> {
    let step = 1 + (70 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_71(data: &[u8]) -> Option<usize> {
    let step = 1 + (71 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_72(data: &[u8]) -> Option<usize> {
    let step = 1 + (72 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_73(data: &[u8]) -> Option<usize> {
    let step = 1 + (73 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_74(data: &[u8]) -> Option<usize> {
    let step = 1 + (74 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_75(data: &[u8]) -> Option<usize> {
    let step = 1 + (75 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_76(data: &[u8]) -> Option<usize> {
    let step = 1 + (76 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_77(data: &[u8]) -> Option<usize> {
    let step = 1 + (77 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_78(data: &[u8]) -> Option<usize> {
    let step = 1 + (78 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}

pub fn find_magic_79(data: &[u8]) -> Option<usize> {
    let step = 1 + (79 % 4);
    let mut pos = 0usize;
    while pos + BLF_MAGIC.len() <= data.len() {
        if data[pos..pos + BLF_MAGIC.len()] == BLF_MAGIC { return Some(pos); }
        pos += step;
    }
    None
}
