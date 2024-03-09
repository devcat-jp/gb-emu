// CPU
use crate::{
    registers::Registers,
    peripherals::Peripherals,
    cpu::operand::{Reg8, Reg16, Imm8, Imm16, Indirect},
};

use self::operand::IO8;

mod operand;
mod instructions;


// 1サイクルで完了しない命令用
#[derive(Default, Clone)]
struct Ctx {
    opecode: u8,
    cb: bool,
}

#[derive(Default, Clone)]
pub struct Cpu {
    cycle: u8,          // debug
    regs: Registers,
    ctx: Ctx,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            cycle: 0,
            regs: Registers::default(),
            ctx: Ctx::default(),
        }
    }

    // フェッチ
    pub fn fetch (&mut self, bus: &Peripherals) {
        self.ctx.opecode = bus.read(self.regs.pc);  // プログラムカウンタを格納
        self.regs.pc = self.regs.pc.wrapping_add(1);      // プログラムカウンタをインクリメント、wrapping_addは桁溢れを無視
        self.ctx.cb = false;
        self.cycle = 0;
        // dbg
        println!("-------------------------");
        println!("op: {:x}", self.ctx.opecode);
        println!("pc: {:x}", self.regs.pc);
        println!("sp: {:x}", self.regs.sp);
        println!("hl: {:x}", self.regs.hl());
        println!("bc: {:x}", self.regs.bc());
        println!("a: {:x}", self.regs.a);
    }

    // 0xCBの場合は16bit命令
    pub fn cb_prefixed(&mut self, bus: &mut Peripherals) {
        // プログラムカウンタの値を読む
        if let Some(v) = self.read8(bus, Imm8) {
            self.ctx.opecode = v;
            self.ctx.cb = true;
            self.cb_decode(bus);
        }
    }

    // デコード
    pub fn decode (&mut self, bus: &mut Peripherals) {
        // 2つ目の表か？
        if self.ctx.cb {
            self.cb_decode(bus);
            return;
        }

        // オペコードで分類
        match self.ctx.opecode {
            0x00 => self.nop(bus),
            
            0x21 => self.ld16(bus, Reg16::HL, Imm16),
            0x31 => self.ld16(bus, Reg16::SP, Imm16),
            
            0x3E => self.ld(bus, Reg8::A, Imm8),
            0x22 => self.ld(bus, Indirect::HLI, Reg8::A),

            0xC5 => self.push(bus, Reg16::BC),
            0xD5 => self.push(bus, Reg16::DE),
            0xE5 => self.push(bus, Reg16::HL),
            0xF5 => self.push(bus, Reg16::AF),
            0xCB => self.cb_prefixed(bus),
            0xCD => self.call(bus),
            _    => panic!("Not implemented: {:02x}", self.ctx.opecode),
        }
    }

    // 2つ目の表のデコード
    pub fn cb_decode (&mut self, bus: &mut Peripherals) {
        // オペコードで分類
        match self.ctx.opecode {
            0x6C => self.bit(bus, 5, Reg8::H),
            _    => panic!("Not implemented: {:02x}", self.ctx.opecode),
        }
    }

    // サイクル
    pub fn emulate_cycle (&mut self, bus: &mut Peripherals) {
        self.cycle = self.cycle.wrapping_add(1);
        println!("M-cycle {}", self.cycle);
        self.decode(bus);
    }



}

