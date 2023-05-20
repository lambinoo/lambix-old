use alloc::boxed::Box;

use core::convert::TryInto;
use core::mem::transmute;
use core::ops::{Index, IndexMut};

use super::*;

#[repr(C)]
pub struct IDT {
    entries: [EntryInner; 256],
}

impl IDT {
    pub fn empty() -> Box<IDT> {
        unsafe { Box::new_zeroed().assume_init() }
    }

    pub fn new() -> Box<IDT> {
        use crate::kernel::table::idt::default::*;

        let mut idt = IDT::empty();
        idt.set_entry_new(0, panic_on_0);
        idt.set_entry_new(1, panic_on_1);
        idt.set_entry_new(2, panic_on_2);
        idt.set_entry_new(3, panic_on_3);
        idt.set_entry_new(4, panic_on_4);
        idt.set_entry_new(5, panic_on_5);
        idt.set_entry_new(6, panic_on_6);
        idt.set_entry_new(7, panic_on_7);
        idt.set_entry_new(8, panic_on_8);
        idt.set_entry_new(9, panic_on_9);
        idt.set_entry_new(10, panic_on_10);
        idt.set_entry_new(11, panic_on_11);
        idt.set_entry_new(12, panic_on_12);
        idt.set_entry_new(13, panic_on_13);
        idt.set_entry_new(14, panic_on_14);
        idt.set_entry_new(15, panic_on_15);
        idt.set_entry_new(16, panic_on_16);
        idt.set_entry_new(17, panic_on_17);
        idt.set_entry_new(18, panic_on_18);
        idt.set_entry_new(19, panic_on_19);
        idt.set_entry_new(20, panic_on_20);
        idt.set_entry_new(21, panic_on_21);
        idt.set_entry_new(22, panic_on_22);
        idt.set_entry_new(23, panic_on_23);
        idt.set_entry_new(24, panic_on_24);
        idt.set_entry_new(25, panic_on_25);
        idt.set_entry_new(26, panic_on_26);
        idt.set_entry_new(27, panic_on_27);
        idt.set_entry_new(28, panic_on_28);
        idt.set_entry_new(29, panic_on_29);
        idt.set_entry_new(30, panic_on_30);
        idt.set_entry_new(31, panic_on_31);
        idt.set_entry_new(32, panic_on_32);
        idt.set_entry_new(33, panic_on_33);
        idt.set_entry_new(34, panic_on_34);
        idt.set_entry_new(35, panic_on_35);
        idt.set_entry_new(36, panic_on_36);
        idt.set_entry_new(37, panic_on_37);
        idt.set_entry_new(38, panic_on_38);
        idt.set_entry_new(39, panic_on_39);
        idt.set_entry_new(40, panic_on_40);
        idt.set_entry_new(41, panic_on_41);
        idt.set_entry_new(42, panic_on_42);
        idt.set_entry_new(43, panic_on_43);
        idt.set_entry_new(44, panic_on_44);
        idt.set_entry_new(45, panic_on_45);
        idt.set_entry_new(46, panic_on_46);
        idt.set_entry_new(47, panic_on_47);
        idt.set_entry_new(48, panic_on_48);
        idt.set_entry_new(49, panic_on_49);
        idt.set_entry_new(50, panic_on_50);
        idt.set_entry_new(51, panic_on_51);
        idt.set_entry_new(52, panic_on_52);
        idt.set_entry_new(53, panic_on_53);
        idt.set_entry_new(54, panic_on_54);
        idt.set_entry_new(55, panic_on_55);
        idt.set_entry_new(56, panic_on_56);
        idt.set_entry_new(57, panic_on_57);
        idt.set_entry_new(58, panic_on_58);
        idt.set_entry_new(59, panic_on_59);
        idt.set_entry_new(60, panic_on_60);
        idt.set_entry_new(61, panic_on_61);
        idt.set_entry_new(62, panic_on_62);
        idt.set_entry_new(63, panic_on_63);
        idt.set_entry_new(64, panic_on_64);
        idt.set_entry_new(65, panic_on_65);
        idt.set_entry_new(66, panic_on_66);
        idt.set_entry_new(67, panic_on_67);
        idt.set_entry_new(68, panic_on_68);
        idt.set_entry_new(69, panic_on_69);
        idt.set_entry_new(70, panic_on_70);
        idt.set_entry_new(71, panic_on_71);
        idt.set_entry_new(72, panic_on_72);
        idt.set_entry_new(73, panic_on_73);
        idt.set_entry_new(74, panic_on_74);
        idt.set_entry_new(75, panic_on_75);
        idt.set_entry_new(76, panic_on_76);
        idt.set_entry_new(77, panic_on_77);
        idt.set_entry_new(78, panic_on_78);
        idt.set_entry_new(79, panic_on_79);
        idt.set_entry_new(80, panic_on_80);
        idt.set_entry_new(81, panic_on_81);
        idt.set_entry_new(82, panic_on_82);
        idt.set_entry_new(83, panic_on_83);
        idt.set_entry_new(84, panic_on_84);
        idt.set_entry_new(85, panic_on_85);
        idt.set_entry_new(86, panic_on_86);
        idt.set_entry_new(87, panic_on_87);
        idt.set_entry_new(88, panic_on_88);
        idt.set_entry_new(89, panic_on_89);
        idt.set_entry_new(90, panic_on_90);
        idt.set_entry_new(91, panic_on_91);
        idt.set_entry_new(92, panic_on_92);
        idt.set_entry_new(93, panic_on_93);
        idt.set_entry_new(94, panic_on_94);
        idt.set_entry_new(95, panic_on_95);
        idt.set_entry_new(96, panic_on_96);
        idt.set_entry_new(97, panic_on_97);
        idt.set_entry_new(98, panic_on_98);
        idt.set_entry_new(99, panic_on_99);
        idt.set_entry_new(100, panic_on_100);
        idt.set_entry_new(101, panic_on_101);
        idt.set_entry_new(102, panic_on_102);
        idt.set_entry_new(103, panic_on_103);
        idt.set_entry_new(104, panic_on_104);
        idt.set_entry_new(105, panic_on_105);
        idt.set_entry_new(106, panic_on_106);
        idt.set_entry_new(107, panic_on_107);
        idt.set_entry_new(108, panic_on_108);
        idt.set_entry_new(109, panic_on_109);
        idt.set_entry_new(110, panic_on_110);
        idt.set_entry_new(111, panic_on_111);
        idt.set_entry_new(112, panic_on_112);
        idt.set_entry_new(113, panic_on_113);
        idt.set_entry_new(114, panic_on_114);
        idt.set_entry_new(115, panic_on_115);
        idt.set_entry_new(116, panic_on_116);
        idt.set_entry_new(117, panic_on_117);
        idt.set_entry_new(118, panic_on_118);
        idt.set_entry_new(119, panic_on_119);
        idt.set_entry_new(120, panic_on_120);
        idt.set_entry_new(121, panic_on_121);
        idt.set_entry_new(122, panic_on_122);
        idt.set_entry_new(123, panic_on_123);
        idt.set_entry_new(124, panic_on_124);
        idt.set_entry_new(125, panic_on_125);
        idt.set_entry_new(126, panic_on_126);
        idt.set_entry_new(127, panic_on_127);
        idt.set_entry_new(128, panic_on_128);
        idt.set_entry_new(129, panic_on_129);
        idt.set_entry_new(130, panic_on_130);
        idt.set_entry_new(131, panic_on_131);
        idt.set_entry_new(132, panic_on_132);
        idt.set_entry_new(133, panic_on_133);
        idt.set_entry_new(134, panic_on_134);
        idt.set_entry_new(135, panic_on_135);
        idt.set_entry_new(136, panic_on_136);
        idt.set_entry_new(137, panic_on_137);
        idt.set_entry_new(138, panic_on_138);
        idt.set_entry_new(139, panic_on_139);
        idt.set_entry_new(140, panic_on_140);
        idt.set_entry_new(141, panic_on_141);
        idt.set_entry_new(142, panic_on_142);
        idt.set_entry_new(143, panic_on_143);
        idt.set_entry_new(144, panic_on_144);
        idt.set_entry_new(145, panic_on_145);
        idt.set_entry_new(146, panic_on_146);
        idt.set_entry_new(147, panic_on_147);
        idt.set_entry_new(148, panic_on_148);
        idt.set_entry_new(149, panic_on_149);
        idt.set_entry_new(150, panic_on_150);
        idt.set_entry_new(151, panic_on_151);
        idt.set_entry_new(152, panic_on_152);
        idt.set_entry_new(153, panic_on_153);
        idt.set_entry_new(154, panic_on_154);
        idt.set_entry_new(155, panic_on_155);
        idt.set_entry_new(156, panic_on_156);
        idt.set_entry_new(157, panic_on_157);
        idt.set_entry_new(158, panic_on_158);
        idt.set_entry_new(159, panic_on_159);
        idt.set_entry_new(160, panic_on_160);
        idt.set_entry_new(161, panic_on_161);
        idt.set_entry_new(162, panic_on_162);
        idt.set_entry_new(163, panic_on_163);
        idt.set_entry_new(164, panic_on_164);
        idt.set_entry_new(165, panic_on_165);
        idt.set_entry_new(166, panic_on_166);
        idt.set_entry_new(167, panic_on_167);
        idt.set_entry_new(168, panic_on_168);
        idt.set_entry_new(169, panic_on_169);
        idt.set_entry_new(170, panic_on_170);
        idt.set_entry_new(171, panic_on_171);
        idt.set_entry_new(172, panic_on_172);
        idt.set_entry_new(173, panic_on_173);
        idt.set_entry_new(174, panic_on_174);
        idt.set_entry_new(175, panic_on_175);
        idt.set_entry_new(176, panic_on_176);
        idt.set_entry_new(177, panic_on_177);
        idt.set_entry_new(178, panic_on_178);
        idt.set_entry_new(179, panic_on_179);
        idt.set_entry_new(180, panic_on_180);
        idt.set_entry_new(181, panic_on_181);
        idt.set_entry_new(182, panic_on_182);
        idt.set_entry_new(183, panic_on_183);
        idt.set_entry_new(184, panic_on_184);
        idt.set_entry_new(185, panic_on_185);
        idt.set_entry_new(186, panic_on_186);
        idt.set_entry_new(187, panic_on_187);
        idt.set_entry_new(188, panic_on_188);
        idt.set_entry_new(189, panic_on_189);
        idt.set_entry_new(190, panic_on_190);
        idt.set_entry_new(191, panic_on_191);
        idt.set_entry_new(192, panic_on_192);
        idt.set_entry_new(193, panic_on_193);
        idt.set_entry_new(194, panic_on_194);
        idt.set_entry_new(195, panic_on_195);
        idt.set_entry_new(196, panic_on_196);
        idt.set_entry_new(197, panic_on_197);
        idt.set_entry_new(198, panic_on_198);
        idt.set_entry_new(199, panic_on_199);
        idt.set_entry_new(200, panic_on_200);
        idt.set_entry_new(201, panic_on_201);
        idt.set_entry_new(202, panic_on_202);
        idt.set_entry_new(203, panic_on_203);
        idt.set_entry_new(204, panic_on_204);
        idt.set_entry_new(205, panic_on_205);
        idt.set_entry_new(206, panic_on_206);
        idt.set_entry_new(207, panic_on_207);
        idt.set_entry_new(208, panic_on_208);
        idt.set_entry_new(209, panic_on_209);
        idt.set_entry_new(210, panic_on_210);
        idt.set_entry_new(211, panic_on_211);
        idt.set_entry_new(212, panic_on_212);
        idt.set_entry_new(213, panic_on_213);
        idt.set_entry_new(214, panic_on_214);
        idt.set_entry_new(215, panic_on_215);
        idt.set_entry_new(216, panic_on_216);
        idt.set_entry_new(217, panic_on_217);
        idt.set_entry_new(218, panic_on_218);
        idt.set_entry_new(219, panic_on_219);
        idt.set_entry_new(220, panic_on_220);
        idt.set_entry_new(221, panic_on_221);
        idt.set_entry_new(222, panic_on_222);
        idt.set_entry_new(223, panic_on_223);
        idt.set_entry_new(224, panic_on_224);
        idt.set_entry_new(225, panic_on_225);
        idt.set_entry_new(226, panic_on_226);
        idt.set_entry_new(227, panic_on_227);
        idt.set_entry_new(228, panic_on_228);
        idt.set_entry_new(229, panic_on_229);
        idt.set_entry_new(230, panic_on_230);
        idt.set_entry_new(231, panic_on_231);
        idt.set_entry_new(232, panic_on_232);
        idt.set_entry_new(233, panic_on_233);
        idt.set_entry_new(234, panic_on_234);
        idt.set_entry_new(235, panic_on_235);
        idt.set_entry_new(236, panic_on_236);
        idt.set_entry_new(237, panic_on_237);
        idt.set_entry_new(238, panic_on_238);
        idt.set_entry_new(239, panic_on_239);
        idt.set_entry_new(240, panic_on_240);
        idt.set_entry_new(241, panic_on_241);
        idt.set_entry_new(242, panic_on_242);
        idt.set_entry_new(243, panic_on_243);
        idt.set_entry_new(244, panic_on_244);
        idt.set_entry_new(245, panic_on_245);
        idt.set_entry_new(246, panic_on_246);
        idt.set_entry_new(247, panic_on_247);
        idt.set_entry_new(248, panic_on_248);
        idt.set_entry_new(249, panic_on_249);
        idt.set_entry_new(250, panic_on_250);
        idt.set_entry_new(251, panic_on_251);
        idt.set_entry_new(252, panic_on_252);
        idt.set_entry_new(253, panic_on_253);
        idt.set_entry_new(254, panic_on_254);
        idt.set_entry_new(255, panic_on_255);
        idt
    }

    fn set_entry_new(&mut self, idx: usize, handler: HandlerWithError) {
        self.get_entry_mut::<HandlerWithError>(idx).set(
            handler,
            GateType::INTERRUPT,
            DPL::PRIVILEGE0,
            0x20,
        );
    }
}

impl IDT {
    pub unsafe fn set_for_this_cpu(idt: Box<IDT>) -> *mut IDT {
        let mut old_reg: IDTRegister = IDTRegister {
            size: 0,
            addr: 0 as _,
        };
        core::arch::asm!("sidt ($0)" : "=r"(&mut old_reg));

        let new_idt = IDTRegister {
            size: core::mem::size_of::<IDT>().try_into().unwrap(),
            addr: Box::into_raw(idt),
        };

        core::arch::asm!("lidt ($0)" :: "r"(&new_idt) :: "volatile");
        old_reg.addr
    }

    fn get_entry<T>(&self, idx: usize) -> &Entry<T> {
        unsafe { transmute(&self.entries[idx]) }
    }

    fn get_entry_mut<T>(&mut self, idx: usize) -> &mut Entry<T> {
        unsafe { transmute(&mut self.entries[idx]) }
    }
}

impl Index<Vector> for IDT {
    type Output = Entry<Handler>;
    fn index(&self, int: Vector) -> &Self::Output {
        self.get_entry(usize::from(int))
    }
}

impl IndexMut<Vector> for IDT {
    fn index_mut(&mut self, int: Vector) -> &mut Self::Output {
        self.get_entry_mut(usize::from(int))
    }
}

impl Index<VectorWithError> for IDT {
    type Output = Entry<HandlerWithError>;
    fn index(&self, int: VectorWithError) -> &Self::Output {
        self.get_entry(usize::from(int))
    }
}

impl IndexMut<VectorWithError> for IDT {
    fn index_mut(&mut self, int: VectorWithError) -> &mut Self::Output {
        self.get_entry_mut(usize::from(int))
    }
}

impl Index<usize> for IDT {
    type Output = Entry<Handler>;
    fn index(&self, int: usize) -> &Self::Output {
        if int > 31 {
            self.get_entry(usize::from(int))
        } else {
            panic!(
                "All interrupts below 32 are reserved and can't be assigned manually. Panicking."
            );
        }
    }
}

impl IndexMut<usize> for IDT {
    fn index_mut(&mut self, int: usize) -> &mut Self::Output {
        if int > 31 {
            self.get_entry_mut(usize::from(int))
        } else {
            panic!(
                "All interrupts below 32 are reserved and can't be assigned manually. Panicking."
            );
        }
    }
}

#[repr(packed)]
struct IDTRegister {
    size: u16,
    addr: *mut IDT,
}
