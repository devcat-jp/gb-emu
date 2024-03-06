
#![allow(dead_code)]

use std::sync::atomic::{
    AtomicU8,
    AtomicU16,
    Ordering::Relaxed,      // 同期は行わない
};


use crate::{
    cpu::Cpu,
    peripherals::Peripherals,
};

// 8bitの読み書きtrait
pub trait IO8<T: Copy> {
    fn read8(&mut self, bus: &Peripherals, src: T) -> Option<u8>;
    fn write8(&mut self, bus: &mut Peripherals, dst: T, val: u8) -> Option<()>;
}

// 16bitの読み書きtrai
pub trait IO16<T: Copy> {
    fn read16(&mut self, bus: &Peripherals, src: T) -> Option<u16>;
    fn write16(&mut self, bus: &mut Peripherals, dst: T, val: u16) -> Option<()>;
}

// enumと構造体
#[derive(Clone, Copy)]
pub enum Reg8 {A, B, C, D, E, H, L}
#[derive(Clone, Copy)]
pub enum Reg16 {AF, BC, DE, HL, SP}
#[derive(Clone, Copy)]
pub enum Indirect {BC, DE, HL, CFF, HLD, HLI}
#[derive(Clone, Copy)]
pub enum Direct8 {D, DFF}
#[derive(Clone, Copy)]
pub enum Cloud {NZ, Z, NC, C}
#[derive(Clone, Copy)]
pub struct Imm8;
#[derive(Clone, Copy)]
pub struct Imm16;

// 8bitレジスタ操作、サイクル消費なし
impl IO8<Reg8> for Cpu {
    fn read8(&mut self, _: &Peripherals, src: Reg8) -> Option<u8> {
        // 値の取り出し
        Some(match src {
            Reg8::A => self.regs.a,
            Reg8::B => self.regs.b,
            Reg8::C => self.regs.c,
            Reg8::D => self.regs.d,
            Reg8::E => self.regs.e,
            Reg8::H => self.regs.h,
            Reg8::L => self.regs.l,
        })
    }

    fn write8(&mut self, _: &mut Peripherals, dst: Reg8, val: u8) -> Option<()> {
        // 値の書き込み
        Some(match dst {
            Reg8::A => self.regs.a = val,
            Reg8::B => self.regs.b = val,
            Reg8::C => self.regs.c = val,
            Reg8::D => self.regs.d = val,
            Reg8::E => self.regs.e = val,
            Reg8::H => self.regs.h = val,
            Reg8::L => self.regs.l = val,
          })
    }
}

// 16bitレジスタ操作、サイクル消費なし
impl IO16<Reg16> for Cpu {
    fn read16(&mut self, _: &Peripherals, src: Reg16) -> Option<u16> {
        println!("Reg16 read");
        // 値の取り出し
        Some(match src {
            Reg16::AF => self.regs.af(),
            Reg16::BC => self.regs.bc(),
            Reg16::DE => self.regs.de(),
            Reg16::HL => self.regs.hl(),
            Reg16::SP => self.regs.sp,
        })
    }

    fn write16(&mut self, _: &mut Peripherals, dst: Reg16, val: u16) -> Option<()> {
        println!("Reg16 write");
        // 値の書き込み
        Some(match dst {
            Reg16::AF => self.regs.write_af(val),
            Reg16::BC => self.regs.write_bc(val),
            Reg16::DE => self.regs.write_de(val),
            Reg16::HL => self.regs.write_hl(val),
            Reg16::SP => self.regs.sp = val,
        })
    }
}


// プログラムカウンタが指す場所から読み取られる8bit、サイクル1消費
impl IO8<Imm8> for Cpu {
    fn read8(&mut self, bus: &Peripherals, _: Imm8) -> Option<u8> {
        static VAL8: AtomicU8 = AtomicU8::new(0);
        // プログラムカウンタの場所を読み取り
        VAL8.store(bus.read(self.regs.pc), Relaxed);
        // プログラムカウンタ増加
        self.regs.pc = self.regs.pc.wrapping_add(1);
        // 応答
        Some(VAL8.load(Relaxed))
    }

    fn write8(&mut self, _: &mut Peripherals, _: Imm8, _: u8) -> Option<()> {
        todo!()
    }
}


// プログラムカウンタが指す場所から読み取られる16bit、サイクル2消費
impl IO16<Imm16> for Cpu {
    fn read16(&mut self, bus: &Peripherals, _: Imm16) -> Option<u16> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                // 下位をImm8で読み込み
                if let Some(lo) = self.read8(bus, Imm8) {
                    VAL8.store(lo, Relaxed);
                    STEP.store(1, Relaxed);
                }
                None
            },
            1 => {
                // 上位を同様に読み出して結合
                if let Some(hi) = self.read8(bus, Imm8) {
                    VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                    STEP.store(2, Relaxed);
                }
                // 応答
                STEP.store(0, Relaxed);
                Some(VAL16.load(Relaxed))
            },
            _ => panic!("Not implemented: Imm8 read"),
        }
    }

    fn write16(&mut self, _: &mut Peripherals, _: Imm16, _: u16) -> Option<()> {
        todo!()
    }
}