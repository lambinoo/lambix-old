use lib::*;

use crate::kernel::table::idt::*;
use core::convert::TryFrom;

#[inline(never)]
/// Panic for interrupt shortcode, never inline so the footprint of this section is minimal
fn _p(frame: &InterruptStackFrame, errcode: u64, vector: u8) {
    if let Ok(vec) = Vector::try_from(usize::from(vector)) {
        panic!("uncaught {:?}, aborting!\nstack frame: {:?}", vec, frame);
    }

    if let Ok(vec) = VectorWithError::try_from(usize::from(vector)) {
        let cr2: usize;
        unsafe {
            core::arch::asm!("mov {}, cr2", out(reg) cr2);
        }

        panic!(
            "uncaught {:?}, aborting!\nerror code: 0x{:x}\nstack frame: {:?}\ncr2: 0x{:x}\n",
            vec, errcode, frame, cr2
        );
    }

    panic!(
        "interrupt {} raised but not handled, aborting\nstack frame: {:?}",
        vector, frame
    );
}

isr! {
    pub fn panic_on_0(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 0); }
    pub fn panic_on_1(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 1); }
    pub fn panic_on_2(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 2); }
    pub fn panic_on_3(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 3); }
    pub fn panic_on_4(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 4); }
    pub fn panic_on_5(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 5); }
    pub fn panic_on_6(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 6); }
    pub fn panic_on_7(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 7); }
    pub fn panic_on_8(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 8); }
    pub fn panic_on_9(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 9); }
    pub fn panic_on_10(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 10); }
    pub fn panic_on_11(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 11); }
    pub fn panic_on_12(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 12); }
    pub fn panic_on_13(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 13); }
    pub fn panic_on_14(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 14); }
    pub fn panic_on_15(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 15); }
    pub fn panic_on_16(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 16); }
    pub fn panic_on_17(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 17); }
    pub fn panic_on_18(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 18); }
    pub fn panic_on_19(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 19); }
    pub fn panic_on_20(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 20); }
    pub fn panic_on_21(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 21); }
    pub fn panic_on_22(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 22); }
    pub fn panic_on_23(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 23); }
    pub fn panic_on_24(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 24); }
    pub fn panic_on_25(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 25); }
    pub fn panic_on_26(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 26); }
    pub fn panic_on_27(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 27); }
    pub fn panic_on_28(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 28); }
    pub fn panic_on_29(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 29); }
    pub fn panic_on_30(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 30); }
    pub fn panic_on_31(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 31); }
    pub fn panic_on_32(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 32); }
    pub fn panic_on_33(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 33); }
    pub fn panic_on_34(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 34); }
    pub fn panic_on_35(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 35); }
    pub fn panic_on_36(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 36); }
    pub fn panic_on_37(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 37); }
    pub fn panic_on_38(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 38); }
    pub fn panic_on_39(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 39); }
    pub fn panic_on_40(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 40); }
    pub fn panic_on_41(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 41); }
    pub fn panic_on_42(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 42); }
    pub fn panic_on_43(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 43); }
    pub fn panic_on_44(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 44); }
    pub fn panic_on_45(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 45); }
    pub fn panic_on_46(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 46); }
    pub fn panic_on_47(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 47); }
    pub fn panic_on_48(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 48); }
    pub fn panic_on_49(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 49); }
    pub fn panic_on_50(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 50); }
    pub fn panic_on_51(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 51); }
    pub fn panic_on_52(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 52); }
    pub fn panic_on_53(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 53); }
    pub fn panic_on_54(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 54); }
    pub fn panic_on_55(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 55); }
    pub fn panic_on_56(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 56); }
    pub fn panic_on_57(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 57); }
    pub fn panic_on_58(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 58); }
    pub fn panic_on_59(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 59); }
    pub fn panic_on_60(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 60); }
    pub fn panic_on_61(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 61); }
    pub fn panic_on_62(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 62); }
    pub fn panic_on_63(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 63); }
    pub fn panic_on_64(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 64); }
    pub fn panic_on_65(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 65); }
    pub fn panic_on_66(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 66); }
    pub fn panic_on_67(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 67); }
    pub fn panic_on_68(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 68); }
    pub fn panic_on_69(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 69); }
    pub fn panic_on_70(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 70); }
    pub fn panic_on_71(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 71); }
    pub fn panic_on_72(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 72); }
    pub fn panic_on_73(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 73); }
    pub fn panic_on_74(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 74); }
    pub fn panic_on_75(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 75); }
    pub fn panic_on_76(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 76); }
    pub fn panic_on_77(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 77); }
    pub fn panic_on_78(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 78); }
    pub fn panic_on_79(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 79); }
    pub fn panic_on_80(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 80); }
    pub fn panic_on_81(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 81); }
    pub fn panic_on_82(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 82); }
    pub fn panic_on_83(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 83); }
    pub fn panic_on_84(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 84); }
    pub fn panic_on_85(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 85); }
    pub fn panic_on_86(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 86); }
    pub fn panic_on_87(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 87); }
    pub fn panic_on_88(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 88); }
    pub fn panic_on_89(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 89); }
    pub fn panic_on_90(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 90); }
    pub fn panic_on_91(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 91); }
    pub fn panic_on_92(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 92); }
    pub fn panic_on_93(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 93); }
    pub fn panic_on_94(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 94); }
    pub fn panic_on_95(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 95); }
    pub fn panic_on_96(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 96); }
    pub fn panic_on_97(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 97); }
    pub fn panic_on_98(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 98); }
    pub fn panic_on_99(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 99); }
    pub fn panic_on_100(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 100); }
    pub fn panic_on_101(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 101); }
    pub fn panic_on_102(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 102); }
    pub fn panic_on_103(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 103); }
    pub fn panic_on_104(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 104); }
    pub fn panic_on_105(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 105); }
    pub fn panic_on_106(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 106); }
    pub fn panic_on_107(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 107); }
    pub fn panic_on_108(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 108); }
    pub fn panic_on_109(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 109); }
    pub fn panic_on_110(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 110); }
    pub fn panic_on_111(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 111); }
    pub fn panic_on_112(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 112); }
    pub fn panic_on_113(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 113); }
    pub fn panic_on_114(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 114); }
    pub fn panic_on_115(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 115); }
    pub fn panic_on_116(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 116); }
    pub fn panic_on_117(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 117); }
    pub fn panic_on_118(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 118); }
    pub fn panic_on_119(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 119); }
    pub fn panic_on_120(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 120); }
    pub fn panic_on_121(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 121); }
    pub fn panic_on_122(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 122); }
    pub fn panic_on_123(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 123); }
    pub fn panic_on_124(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 124); }
    pub fn panic_on_125(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 125); }
    pub fn panic_on_126(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 126); }
    pub fn panic_on_127(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 127); }
    pub fn panic_on_128(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 128); }
    pub fn panic_on_129(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 129); }
    pub fn panic_on_130(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 130); }
    pub fn panic_on_131(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 131); }
    pub fn panic_on_132(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 132); }
    pub fn panic_on_133(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 133); }
    pub fn panic_on_134(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 134); }
    pub fn panic_on_135(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 135); }
    pub fn panic_on_136(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 136); }
    pub fn panic_on_137(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 137); }
    pub fn panic_on_138(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 138); }
    pub fn panic_on_139(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 139); }
    pub fn panic_on_140(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 140); }
    pub fn panic_on_141(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 141); }
    pub fn panic_on_142(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 142); }
    pub fn panic_on_143(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 143); }
    pub fn panic_on_144(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 144); }
    pub fn panic_on_145(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 145); }
    pub fn panic_on_146(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 146); }
    pub fn panic_on_147(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 147); }
    pub fn panic_on_148(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 148); }
    pub fn panic_on_149(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 149); }
    pub fn panic_on_150(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 150); }
    pub fn panic_on_151(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 151); }
    pub fn panic_on_152(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 152); }
    pub fn panic_on_153(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 153); }
    pub fn panic_on_154(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 154); }
    pub fn panic_on_155(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 155); }
    pub fn panic_on_156(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 156); }
    pub fn panic_on_157(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 157); }
    pub fn panic_on_158(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 158); }
    pub fn panic_on_159(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 159); }
    pub fn panic_on_160(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 160); }
    pub fn panic_on_161(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 161); }
    pub fn panic_on_162(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 162); }
    pub fn panic_on_163(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 163); }
    pub fn panic_on_164(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 164); }
    pub fn panic_on_165(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 165); }
    pub fn panic_on_166(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 166); }
    pub fn panic_on_167(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 167); }
    pub fn panic_on_168(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 168); }
    pub fn panic_on_169(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 169); }
    pub fn panic_on_170(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 170); }
    pub fn panic_on_171(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 171); }
    pub fn panic_on_172(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 172); }
    pub fn panic_on_173(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 173); }
    pub fn panic_on_174(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 174); }
    pub fn panic_on_175(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 175); }
    pub fn panic_on_176(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 176); }
    pub fn panic_on_177(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 177); }
    pub fn panic_on_178(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 178); }
    pub fn panic_on_179(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 179); }
    pub fn panic_on_180(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 180); }
    pub fn panic_on_181(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 181); }
    pub fn panic_on_182(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 182); }
    pub fn panic_on_183(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 183); }
    pub fn panic_on_184(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 184); }
    pub fn panic_on_185(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 185); }
    pub fn panic_on_186(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 186); }
    pub fn panic_on_187(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 187); }
    pub fn panic_on_188(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 188); }
    pub fn panic_on_189(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 189); }
    pub fn panic_on_190(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 190); }
    pub fn panic_on_191(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 191); }
    pub fn panic_on_192(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 192); }
    pub fn panic_on_193(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 193); }
    pub fn panic_on_194(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 194); }
    pub fn panic_on_195(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 195); }
    pub fn panic_on_196(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 196); }
    pub fn panic_on_197(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 197); }
    pub fn panic_on_198(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 198); }
    pub fn panic_on_199(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 199); }
    pub fn panic_on_200(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 200); }
    pub fn panic_on_201(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 201); }
    pub fn panic_on_202(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 202); }
    pub fn panic_on_203(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 203); }
    pub fn panic_on_204(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 204); }
    pub fn panic_on_205(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 205); }
    pub fn panic_on_206(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 206); }
    pub fn panic_on_207(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 207); }
    pub fn panic_on_208(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 208); }
    pub fn panic_on_209(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 209); }
    pub fn panic_on_210(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 210); }
    pub fn panic_on_211(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 211); }
    pub fn panic_on_212(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 212); }
    pub fn panic_on_213(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 213); }
    pub fn panic_on_214(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 214); }
    pub fn panic_on_215(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 215); }
    pub fn panic_on_216(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 216); }
    pub fn panic_on_217(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 217); }
    pub fn panic_on_218(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 218); }
    pub fn panic_on_219(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 219); }
    pub fn panic_on_220(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 220); }
    pub fn panic_on_221(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 221); }
    pub fn panic_on_222(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 222); }
    pub fn panic_on_223(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 223); }
    pub fn panic_on_224(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 224); }
    pub fn panic_on_225(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 225); }
    pub fn panic_on_226(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 226); }
    pub fn panic_on_227(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 227); }
    pub fn panic_on_228(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 228); }
    pub fn panic_on_229(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 229); }
    pub fn panic_on_230(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 230); }
    pub fn panic_on_231(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 231); }
    pub fn panic_on_232(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 232); }
    pub fn panic_on_233(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 233); }
    pub fn panic_on_234(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 234); }
    pub fn panic_on_235(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 235); }
    pub fn panic_on_236(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 236); }
    pub fn panic_on_237(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 237); }
    pub fn panic_on_238(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 238); }
    pub fn panic_on_239(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 239); }
    pub fn panic_on_240(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 240); }
    pub fn panic_on_241(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 241); }
    pub fn panic_on_242(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 242); }
    pub fn panic_on_243(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 243); }
    pub fn panic_on_244(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 244); }
    pub fn panic_on_245(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 245); }
    pub fn panic_on_246(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 246); }
    pub fn panic_on_247(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 247); }
    pub fn panic_on_248(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 248); }
    pub fn panic_on_249(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 249); }
    pub fn panic_on_250(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 250); }
    pub fn panic_on_251(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 251); }
    pub fn panic_on_252(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 252); }
    pub fn panic_on_253(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 253); }
    pub fn panic_on_254(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 254); }
    pub fn panic_on_255(frame: &InterruptStackFrame, errcode: u64) { _p(frame, errcode, 255); }
}
