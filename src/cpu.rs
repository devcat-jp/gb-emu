//use std::io::{self};

use std::sync::atomic::{
    AtomicU8, Ordering::Relaxed      // 同期は行わない
};

// CPU
use crate::{
    cpu::{interrupts::{Interrupts, JOYPAD, SERIAL, STAT, TIMER, VBLANK}, operand::{Cond, Direct8, Imm16, Imm8, Indirect, Reg16, Reg8, IO8}}, peripherals::Peripherals, registers::Registers
};

mod operand;
mod instructions;
pub mod interrupts;



// 1サイクルで完了しない命令用
#[derive(Default, Clone)]
struct Ctx {
    opecode: u8,
    cb: bool,
    int: bool,          // 割り込みフラグ
}

#[derive(Default, Clone)]
pub struct Cpu {
    cycle: u8,          // debug
    regs: Registers,
    pub interrupts: Interrupts,
    ctx: Ctx,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            cycle: 0,
            regs: Registers::default(),
            interrupts: Interrupts::default(),
            ctx: Ctx::default(),
        }
    }

    // フェッチ
    pub fn fetch (&mut self, bus: &Peripherals) {
        self.ctx.opecode = bus.read(&self.interrupts, self.regs.pc);  // プログラムカウンタを格納
        if self.interrupts.ime && self.interrupts.get_interrupt() > 0 {     // 割り込みは有効か？
            self.ctx.int = true;
        } else {
            self.ctx.int = false;
        }
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

    // サイクル
    pub fn emulate_cycle (&mut self, bus: &mut Peripherals) {
        self.cycle = self.cycle.wrapping_add(1);
        //if self.ctx.opecode == 0xC5  {println!("M-cycle {}", self.cycle);}

        if self.ctx.int {
            self.call_isr(bus);
        } else {
            self.decode(bus);

        }
        
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

    // ISR
    fn call_isr(&mut self, bus: &mut Peripherals) {
        static STEP: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(_) = self.push16(bus, self.regs.pc) {
                    // 割り込み優先順位が高いものを処理する、trailing_zerosは末尾の0の数を返す
                    let highest_int: u8 = 1 << self.interrupts.get_interrupt().trailing_zeros();
                    self.interrupts.int_flags &= !highest_int;      // 割り込み承認
                    // ISR呼び出し
                    self.regs.pc = match highest_int {
                        VBLANK => 0x0040,       // ISRのアドレス
                        STAT   => 0x0048,
                        TIMER  => 0x0050,
                        SERIAL => 0x0058,
                        JOYPAD => 0x0060,
                        _      => panic!("Not Define: {:x}", highest_int),
                    };
                    STEP.store(1, Relaxed);
                }
            },
            1 => {
                self.interrupts.ime = false;    // 割り込み無効
                STEP.store(0, Relaxed);
                self.fetch(bus);
            },
            _ => panic!("Not Define"),
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

            _    => panic!("Not implemented: 0x{:02x}, pc: 0x{:x}", self.ctx.opecode, self.regs.pc),
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





}

