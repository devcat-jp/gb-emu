

use std::sync::atomic::{
    AtomicU8,
    AtomicU16,
    Ordering::Relaxed,      // 同期は行わない
};

use crate::{
    cpu::{
        Cpu,
        operand::{IO8, IO16, Reg16, Imm16},
    },
    peripherals::Peripherals,
};


impl Cpu {
    // NOP命令
    pub fn nop (&mut self, bus: &Peripherals) {
        println!("nop");
        self.fetch(bus);
    }

    // ld d s ： s の値を d  に格納する
    pub fn ld<D: Copy, S: Copy> (&mut self, bus: &mut Peripherals, dst: D, src: S) 
    where Self: IO8<D> + IO8<S> {
        println!("ld8");
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(bus, src) {
                    VAL8.store(v, Relaxed);
                    STEP.store(1, Relaxed);
                    //
                    self.ld(bus, dst, src);
                }
            },
            1 => {
                if self.write8(bus, dst, VAL8.load(Relaxed)).is_some() {
                    STEP.store(0, Relaxed);
                    self.fetch(bus);
                }
            },
            _ => panic!("Not implemented: id16"),
        }
    }
    pub fn ld16<D: Copy, S: Copy> (&mut self, bus: &mut Peripherals, dst: D, src: S) 
    where Self: IO16<D> + IO16<S> {
        println!("[ld16]");
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read16(bus, src) {
                    VAL16.store(v, Relaxed);
                    STEP.store(1, Relaxed);
                    // 
                    self.ld16(bus, dst, src);
                }
            },
            1 => {
                if self.write16(bus, dst, VAL16.load(Relaxed)).is_some() {
                    STEP.store(0, Relaxed);
                    self.fetch(bus);
                }
            },
            _ => panic!("Not implemented: id16"),
        }
    }

    // bit num s : s の num bit目が0か1かを確認する
    pub fn bit <S: Copy>(&mut self, bus: &Peripherals, bit: usize, src: S) 
    where Self: IO8<S> {
        println!("[bit]");
        if let Some(mut v) = self.read8(bus, src) {
            v &= 1 << bit;
            self.regs.set_zf(v == 0);
            self.regs.set_nf(false);
            self.regs.set_hf(true);
            self.fetch(bus);
        }

    }


    // push ：　16bit値をデクリメントした後にスタックポインタが指すアドレスに値を格納する
    pub fn push16 (&mut self, bus: &mut Peripherals, val: u16) -> Option<()> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        println!("push16");
        static VAL8: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                // pushはメモリアクセス数+1のサイクル数
                STEP.store(1, Relaxed);
                None
            },
            1 => {
                // 値を取得
                let [lo, hi] = u16::to_le_bytes(val);
                // デクリメントしたアドレスに書き込み
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(self.regs.sp, hi);
                //
                VAL8.store(lo, Relaxed);
                STEP.store(2, Relaxed);
                None
            },
            2 => {
                // デクリメントしたアドレスに書き込み
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(self.regs.sp, VAL8.load(Relaxed));
                //
                STEP.store(0, Relaxed);
                Some(())
            },
            _ => panic!("Not implemented: push16"),
        }
    }
    pub fn push (&mut self, bus: &mut Peripherals, src: Reg16) {
        println!("push");
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                VAL16.store(self.read16(bus, src).unwrap(), Relaxed);
                STEP.store(1, Relaxed);
            },
            1 => {
                if self.push16(bus, VAL16.load(Relaxed)).is_some() {
                    STEP.store(1, Relaxed);
                    self.fetch(bus);
                }
            }
            _ => panic!("Not implemented: push"),
        }
    }

    // call ：　プログラムカウンタの値をスタックにpushし、その後元のプログラムカウンタに戻す、6サイクル
    pub fn call (&mut self, bus: &mut Peripherals) {
        println!("call");
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                // プログラムカウンタの値取り出し（2サイクル）
                if let Some(v) = self.read16(bus, Imm16) {
                    VAL16.store(v, Relaxed);
                    STEP.store(1, Relaxed);
                    println!("hoge {:x}", v);
                }
            },
            1 => {
                // プログラムカウンタの値をpush（3サイクル）
                if self.push16(bus, self.regs.pc).is_some() {
                    self.regs.pc = VAL16.load(Relaxed);
                    STEP.store(2, Relaxed);
                }
            },
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => panic!("Not implemented: call"),
        }
    }


}
