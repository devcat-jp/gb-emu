

use std::sync::atomic::{
    AtomicU8,
    AtomicU16,
    Ordering::Relaxed,      // 同期は行わない
};

use crate::{
    cpu::{
        Cpu,
        operand::IO16,
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
    pub fn ld16<D: Copy, S: Copy> (&mut self, bus: &mut Peripherals, dst: D, src: S) 
    where Self: IO16<D> + IO16<S> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read16(bus, src) {
                    VAL16.store(v, Relaxed);
                    STEP.store(1, Relaxed);
                    println!("ld16_src : {:x}", VAL16.load(Relaxed));
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
}
