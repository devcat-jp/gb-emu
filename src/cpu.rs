//use std::io::{self};

// CPU
use crate::{
    registers::Registers,
    peripherals::Peripherals,
    cpu::operand::{IO8, Reg8, Direct8, Reg16, Imm8, Imm16, Indirect, Cond},
};

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
        if self.ctx.opecode == 0xFF {
            println!("-------------------------");
            println!("op: {:x}", self.ctx.opecode);
            println!("pc: {:x}", self.regs.pc);
            println!("sp: {:x}", self.regs.sp);
            //println!("hl: {:x}", self.regs.hl());
            //println!("bc: {:x}", self.regs.bc());
            //println!("a: {:x}", self.regs.a);
            println!("c: {:x}", self.regs.c);
        }
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


            
            0x06 => self.ld(bus, Reg8::B, Imm8),
            0x0E => self.ld(bus, Reg8::C, Imm8),
            0x1A => self.ld(bus, Reg8::A, Indirect::DE),
            0x22 => self.ld(bus, Indirect::HLI, Reg8::A),
            0x32 => self.ld(bus, Indirect::HLD, Reg8::A),
            0x2E => self.ld(bus, Reg8::L, Imm8),
            0x3E => self.ld(bus, Reg8::A, Imm8),
            0x47 => self.ld(bus, Reg8::B, Reg8::A),
            0x57 => self.ld(bus, Reg8::D, Reg8::A),
            0x79 => self.ld(bus, Reg8::A, Reg8::C),
            0x7A => self.ld(bus, Reg8::A, Reg8::D),
            0x7B => self.ld(bus, Reg8::A, Reg8::E),
            0xE0 => self.ld(bus, Direct8::DFF, Reg8::A),
            0xEA => self.ld(bus, Direct8::D, Reg8::A),
            
            
            0x01 => self.ld16(bus, Reg16::BC, Imm16),
            0x11 => self.ld16(bus, Reg16::DE, Imm16),
            0x21 => self.ld16(bus, Reg16::HL, Imm16),
            0x31 => self.ld16(bus, Reg16::SP, Imm16),

            0x13 => self.inc16(bus, Reg16::DE),
            0x23 => self.inc16(bus, Reg16::HL),

            0x05 => self.dec(bus, Reg8::B),
            0x15 => self.dec(bus, Reg8::D),
            0x0D => self.dec(bus, Reg8::C),
            0x3D => self.dec(bus, Reg8::A),

            0x18 => self.jr(bus),
            0x20 => self.jr_c(bus, Cond::NZ),
            0x28 => self.jr_c(bus, Cond::Z),

            //0xC5 => self.push(bus, Reg16::BC),
            //0xD5 => self.push(bus, Reg16::DE),
            //0xE5 => self.push(bus, Reg16::HL),
            //0xF5 => self.push(bus, Reg16::AF),

            0xC3 => self.jp(bus),
            
            0xC9 => self.ret(bus),
            0xCB => self.cb_prefixed(bus),
            
            0xCD => self.call(bus),

            0xC1 => self.pop(bus, Reg16::BC),
            0xF1 => self.pop(bus, Reg16::AF),

            0xC5 => self.push(bus, Reg16::BC),
            0xE5 => self.push(bus, Reg16::HL),
            0xF5 => self.push(bus, Reg16::AF),
            
            
            0xFE => self.cp(bus, Imm8),

            _    => panic!("Not implemented: {:02x}", self.ctx.opecode),
        }
    }

    // 2つ目の表のデコード
    pub fn cb_decode (&mut self, bus: &mut Peripherals) {
        // オペコードで分類
        match self.ctx.opecode {
            0x10 => self.rl(bus, Reg8::B),
            0x11 => self.rl(bus, Reg8::C),
            0x6C => self.bit(bus, 5, Reg8::H),
            _    => panic!("Not implemented: {:02x}", self.ctx.opecode),
        }
    }

    // サイクル
    pub fn emulate_cycle (&mut self, bus: &mut Peripherals) {
        self.cycle = self.cycle.wrapping_add(1);
        //if self.ctx.opecode == 0x18  {println!("M-cycle {}", self.cycle);}
        self.decode(bus);

        // debug
        // 命令終了時の状態で停止させる
        /*
        if self.regs.a == 0x91 {
            println!("[[STOP]]");
            println!("op: {:x}", self.ctx.opecode);
            println!("pc: {:x}", self.regs.pc);
            println!("sp: {:x}", self.regs.sp);

            let mut line = String::new(); // 入力用のバッファ
            io::stdin()
                .read_line(&mut line) // キーボードからの入力（標準入力）を 1 行読み込む
                .expect("Failed to read line"); // 戻り値の Result が Err の場合は終了
        }
        */
    }



}

