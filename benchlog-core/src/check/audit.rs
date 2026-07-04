//! Capture audit trail helpers.

use crate::error::Result;
use crate::ingest::parse::CaptureDoc;

pub fn audit_00(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(0))
}

pub fn audit_01(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(1))
}

pub fn audit_02(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(2))
}

pub fn audit_03(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(3))
}

pub fn audit_04(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(4))
}

pub fn audit_05(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(5))
}

pub fn audit_06(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(6))
}

pub fn audit_07(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(7))
}

pub fn audit_08(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(8))
}

pub fn audit_09(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(9))
}

pub fn audit_10(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(10))
}

pub fn audit_11(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(11))
}

pub fn audit_12(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(12))
}

pub fn audit_13(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(13))
}

pub fn audit_14(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(14))
}

pub fn audit_15(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(15))
}

pub fn audit_16(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(16))
}

pub fn audit_17(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(17))
}

pub fn audit_18(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(18))
}

pub fn audit_19(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(19))
}

pub fn audit_20(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(20))
}

pub fn audit_21(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(21))
}

pub fn audit_22(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(22))
}

pub fn audit_23(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(23))
}

pub fn audit_24(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(24))
}

pub fn audit_25(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(25))
}

pub fn audit_26(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(26))
}

pub fn audit_27(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(27))
}

pub fn audit_28(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(28))
}

pub fn audit_29(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(29))
}

pub fn audit_30(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(30))
}

pub fn audit_31(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(31))
}

pub fn audit_32(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(32))
}

pub fn audit_33(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(33))
}

pub fn audit_34(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(34))
}

pub fn audit_35(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(35))
}

pub fn audit_36(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(36))
}

pub fn audit_37(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(37))
}

pub fn audit_38(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(38))
}

pub fn audit_39(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(39))
}

pub fn audit_40(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(40))
}

pub fn audit_41(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(41))
}

pub fn audit_42(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(42))
}

pub fn audit_43(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(43))
}

pub fn audit_44(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(44))
}

pub fn audit_45(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(45))
}

pub fn audit_46(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(46))
}

pub fn audit_47(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(47))
}

pub fn audit_48(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(48))
}

pub fn audit_49(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(49))
}

pub fn audit_50(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(50))
}

pub fn audit_51(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(51))
}

pub fn audit_52(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(52))
}

pub fn audit_53(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(53))
}

pub fn audit_54(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(54))
}

pub fn audit_55(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(55))
}

pub fn audit_56(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(56))
}

pub fn audit_57(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(57))
}

pub fn audit_58(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(58))
}

pub fn audit_59(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(59))
}

pub fn audit_60(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(60))
}

pub fn audit_61(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(61))
}

pub fn audit_62(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(62))
}

pub fn audit_63(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(63))
}

pub fn audit_64(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(64))
}

pub fn audit_65(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(65))
}

pub fn audit_66(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(66))
}

pub fn audit_67(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(67))
}

pub fn audit_68(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(68))
}

pub fn audit_69(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(69))
}

pub fn audit_70(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(70))
}

pub fn audit_71(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(71))
}

pub fn audit_72(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(72))
}

pub fn audit_73(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(73))
}

pub fn audit_74(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(74))
}

pub fn audit_75(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(75))
}

pub fn audit_76(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(76))
}

pub fn audit_77(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(77))
}

pub fn audit_78(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(78))
}

pub fn audit_79(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(79))
}

pub fn audit_80(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(80))
}

pub fn audit_81(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(81))
}

pub fn audit_82(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(82))
}

pub fn audit_83(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(83))
}

pub fn audit_84(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(84))
}

pub fn audit_85(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(85))
}

pub fn audit_86(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(86))
}

pub fn audit_87(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(87))
}

pub fn audit_88(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(88))
}

pub fn audit_89(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(89))
}

pub fn audit_90(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(90))
}

pub fn audit_91(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(91))
}

pub fn audit_92(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(92))
}

pub fn audit_93(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(93))
}

pub fn audit_94(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(94))
}

pub fn audit_95(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(95))
}

pub fn audit_96(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(96))
}

pub fn audit_97(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(97))
}

pub fn audit_98(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(98))
}

pub fn audit_99(doc: &CaptureDoc) -> Result<u32> {
    let base = doc.samples.len() as u32;
    let adj = doc.events.len() as u32;
    Ok(base.wrapping_add(adj).wrapping_add(99))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ingest::parse::CaptureDoc;
    #[test]
    fn audit_empty() {
        assert_eq!(audit_00(&CaptureDoc::default()).unwrap(), 0);
    }
}
