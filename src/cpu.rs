// CPU

use crate::{
    registers::Registers,
    peripherals::Peripherals,
};


// 1サイクルで完了しない命令用
#[derive(Default)]
struct Ctx {
    opecode: u8,
    cb: bool,
}

#[derive(Default)]
pub struct Cpu {
    regs: Registers,
    ctx: Ctx,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::default(),
            ctx: Ctx::default(),
        }
    }

    // フェッチ
    pub fn fetch (&mut self, bus: &Peripherals) {
        self.ctx.opecode = bus.read(self.regs.pc);  // プログラムカウンタを格納
        self.regs.pc = self.regs.pc.wrapping_add(1);      // プログラムカウンタをインクリメント、wrapping_addは桁溢れを無視
        self.ctx.cb = false;
        // dbg
        println!("pc: {:x}", self.regs.pc);
        println!("op: {:x}", self.ctx.opecode);
    }

    // デコード
    pub fn decode (&mut self, bus: &mut Peripherals) {
        // オペコードで分類
        match self.ctx.opecode {
            0x00 => self.nop(bus),
            _    => panic!("Not implemented: {:02x}", self.ctx.opecode),
        }
    }

    // サイクル
    pub fn emulate_cycle (&mut self, bus: &mut Peripherals) {
        self.decode(bus);
    }

    // NOP命令
    pub fn nop (&mut self, bus: &Peripherals) {
        println!("nop");
        self.fetch(bus);
    }

}

