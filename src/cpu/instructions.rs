

use std::sync::atomic::{
    AtomicU16, AtomicU8, Ordering::Relaxed      // 同期は行わない
};

use crate::{
    cpu::{
        operand::{Cond, Imm16, Imm8, Reg16, IO16, IO8}, Cpu
    },
    peripherals::Peripherals,
};


impl Cpu {
    // NOP命令
    pub fn nop (&mut self, bus: &Peripherals) {
        //println!("nop");
        self.fetch(bus);
    }

    // ld d s ： s の値を d  に格納する
    pub fn ld<D: Copy, S: Copy> (&mut self, bus: &mut Peripherals, dst: D, src: S) 
    where Self: IO8<D> + IO8<S> {
        //println!("ld8");
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(bus, src) {
                    VAL8.store(v, Relaxed);
                    STEP.store(1, Relaxed);
                    // 応答が得られたので再度処理を行う
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
        //println!("[ld16]");
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read16(bus, src) {
                    VAL16.store(v, Relaxed);
                    STEP.store(1, Relaxed);
                    // 応答が得られたので再度処理を行う
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

    // CP s : Aレジスタからsの値を引き、レジスタ設定を行う
    pub fn cp<S :Copy>(&mut self, bus: &Peripherals, src: S)
    where Self: IO8<S> {
        if let Some(v) = self.read8(bus, src) {
            let (result, carry) = self.regs.a.overflowing_sub(v);
            // フラグ設定
            self.regs.set_zf(result == 0);
            self.regs.set_nf(true);
            self.regs.set_hf((self.regs.a & 0xf) < (v & 0xf));
            self.regs.set_cf(carry);
            //
            self.fetch(bus);
        }
    }

    // bit num s : s の num bit目が0か1かを確認する
    pub fn bit <S: Copy>(&mut self, bus: &Peripherals, bit: usize, src: S) 
    where Self: IO8<S> {
        //println!("[bit]");
        if let Some(mut v) = self.read8(bus, src) {
            v &= 1 << bit;
            self.regs.set_zf(v == 0);
            self.regs.set_nf(false);
            self.regs.set_hf(true);
            self.fetch(bus);
        }

    }

    // DEC s : sをデクリメントする
    // 8bit操作の時はフラグレジスタ操作も必要
    pub fn dec<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> 
    {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(bus, src) {
                    let result = v.wrapping_sub(1);     // デクリメント
                    // Fレジスタ設定
                    self.regs.set_zf(result == 0);
                    self.regs.set_nf(true);
                    self.regs.set_hf(v & 0xf == 0);
                    // 記録
                    VAL8.store(result, Relaxed);
                    STEP.store(1, Relaxed);
                    // 応答が得られたので再度処理を行う
                    self.dec(bus, src);
                }
            },
            1 => {
                if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                    STEP.store(0, Relaxed);
                    self.fetch(bus);
                }
            }
            _ => panic!("Err dec :{}", STEP.load(Relaxed)),
        }
    }
    pub fn dec16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO16<S>
    {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read16(bus, src) {
                    VAL16.store(v.wrapping_sub(1), Relaxed);
                    STEP.store(1, Relaxed);
                    // 応答が得られたので再度処理を行う
                    self.dec16(bus, src);
                }
            },
            1 => {
                if self.write16(bus, src, VAL16.load(Relaxed)).is_some() {
                    STEP.store(2, Relaxed);
                }
            },
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => panic!(""),
        }
    }

    // INC s : sをインクリメントする
    // 8bit操作の時はフラグレジスタ操作も必要
    pub fn inc<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> 
    {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(bus, src) {
                    let result = v.wrapping_add(1);     // インクリメント
                    // Fレジスタ設定
                    self.regs.set_zf(result == 0);
                    self.regs.set_nf(false);
                    self.regs.set_hf(v & 0xf == 0xf);
                    // 記録
                    VAL8.store(result, Relaxed);
                    STEP.store(1, Relaxed);
                    // 応答が得られたので再度処理を行う
                    self.inc(bus, src);
                }
            },
            1 => {
                if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                    STEP.store(0, Relaxed);
                    self.fetch(bus);
                }
            }
            _ => panic!("Err dec :{}", STEP.load(Relaxed)),
        }
    }
    pub fn inc16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO16<S> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read16(bus, src) {
                    VAL16.store(v.wrapping_add(1), Relaxed);
                    STEP.store(1, Relaxed);
                    // 応答が得られたので再度処理を行う
                    self.inc16(bus, src);
                }
            },
            1 => {
                if self.write16(bus, src, VAL16.load(Relaxed)).is_some() {
                    STEP.store(2, Relaxed);
                }
            },
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => panic!(""),
        }
    }

    // JR : プログラムカウンタに値を加算する
    pub fn jr(&mut self, bus: &Peripherals) {
        static STEP: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(bus, Imm8) {
                    self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16);
                    STEP.store(1, Relaxed);
                    // 応答が得られたので再度処理を行う
                    self.jr(bus);
                }
            },
            1 => {
                // サイクル数+1
                STEP.store(2, Relaxed);
            }
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => panic!(""),
        }
    }


    // JR c : フラグがcを満たしていればJR命令（プログラムカウンタに加算）を行う
    fn cond (&mut self, cond: Cond) -> bool {
        match cond {
            Cond::NZ => !self.regs.zf(),
            Cond::Z  => self.regs.zf(),
            Cond::NC => !self.regs.cf(),
            Cond::C  => self.regs.cf(),
        }
    }
    pub fn jr_c (&mut self, bus: &Peripherals, c: Cond) {
        //println!("[jr_c]");
        static STEP: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(bus, Imm8) {
                    STEP.store(2, Relaxed);
                    // 条件を満たしている場合はジャンプ、加えてサイクル+1
                    if self.cond(c) {
                        //println!("exec jr");
                        self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16);
                        STEP.store(1, Relaxed);
                    }
                    // 応答が得られたので再度処理を行う
                    self.jr_c(bus, c);
                }
            },
            1 => {
                STEP.store(2, Relaxed);
            },
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            },
            _ => panic!(""),
        }
    }


    // RL s : sの値とCフラグを合わせた9bitの値を左に回転
    // 1bit左シフト、Cフラグを最下位bitにセットする
    pub fn rl<S: Copy> (&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        //println!("[rl]");
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(bus, src) {
                    let result = (v << 1) | self.regs.cf() as u8;
                    self.regs.set_zf(result == 0);
                    self.regs.set_nf(false);
                    self.regs.set_hf(false);
                    self.regs.set_cf(v & 0x80 > 0);
                    VAL8.store(result, Relaxed);
                    STEP.store(1, Relaxed);
                    // 応答が得られたので再度処理を行う
                    self.rl(bus, src);
                }
            },
            1 => {
                if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                    STEP.store(0, Relaxed);
                    self.fetch(bus);
                }
            }
            _ => panic!(""),
        }
    }


    // push ：　16bit値をデクリメントした後にスタックポインタが指すアドレスに値を格納する
    pub fn push16 (&mut self, bus: &mut Peripherals, val: u16) -> Option<()> {
        //println!("[push16]");
        static STEP: AtomicU8 = AtomicU8::new(0);
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
        //println!("push");
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                VAL16.store(self.read16(bus, src).unwrap(), Relaxed);
                STEP.store(1, Relaxed);
                // 応答が得られたので再度処理を行う
                self.push(bus, src);
            },
            1 => {
                if self.push16(bus, VAL16.load(Relaxed)).is_some() {
                    STEP.store(2, Relaxed);
                }
            },
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            },
            _ => panic!("Not implemented: push"),
        }
    }

    // pop d : 16bitの値をスタックからpopする
    pub fn pop16 (&mut self, bus: &Peripherals) -> Option<u16> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                VAL8.store(bus.read(self.regs.sp), Relaxed);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                STEP.store(1, Relaxed);
                None
            },
            1 => {
                let hi = bus.read(self.regs.sp);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                STEP.store(2, Relaxed);
                None
            },
            2 => {
                STEP.store(0, Relaxed);
                Some(VAL16.load(Relaxed))
            },
            _ => panic!(""),
        }
    }
    pub fn pop (&mut self, bus: &mut Peripherals, dst: Reg16) {
        if let Some(v) = self.pop16(bus) {
            self.write16(bus, dst, v);
            self.fetch(bus);
        }
    }

    // RET : return
    // 16bitの値をプログラムカウンタに代入する、4サイクル
    pub fn ret(&mut self, bus: &Peripherals) {
        static STEP: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.pop16(bus) {
                    self.regs.pc = v;
                    STEP.store(1, Relaxed);
                }
            },
            1 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            },
            _ => panic!(""),
        }

    }


    // call ：　プログラムカウンタの値をスタックにpushし、その後元のプログラムカウンタに戻す、6サイクル
    pub fn call (&mut self, bus: &mut Peripherals) {
        //println!("call");
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                // プログラムカウンタの値取り出し
                if let Some(v) = self.read16(bus, Imm16) {
                    VAL16.store(v, Relaxed);
                    STEP.store(1, Relaxed);
                    // 応答が得られたので再度処理を行う
                    self.call(bus);
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

    // JP
    pub fn jp(&mut self, bus: &Peripherals) {
        static STEP: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read16(bus, Imm16) {
                    self.regs.pc = v;
                    STEP.store(1, Relaxed);
                    // 応答が得られたので再度処理を行う
                    self.jp(bus);
                }
            },
            1 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => panic!(""),
        }
        
        /*
        step!(self.ctx.cache["inst"].step, (), {
          0: if let Some(v) = self.read16(bus, Imm16) {
            self.regs.pc = v;
            return go!(self.ctx.cache.get_mut("inst").unwrap().step, 1);
          },
          1: {
            go!(self.ctx.cache.get_mut("inst").unwrap().step, 0);
            self.fetch(bus);
          },
        });
        */
      }

    


}
