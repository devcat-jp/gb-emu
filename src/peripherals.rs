// 周辺機器管理
#![allow(dead_code)]

use crate::{
    bootrom::Bootrom,
    hram::HRam,
};

pub struct Peripherals {
    bootrom: Bootrom,
    hram: HRam,
}

impl Peripherals {
    pub fn new (bootrom: Bootrom) -> Self {
        Self {
            bootrom,
            hram: HRam::new(),
        }
    }

    // MMIO読み込み
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // ブートROMが無効の時はカートリッジ
            0x0000..=0x00FF => if self.bootrom.is_active() {
                self.bootrom.read(addr)
            } else {
                0xFF
            },
            0xFF80..=0xFFFE => self.hram.read(addr),
            _ => 0xFF
        }
    }

    // MMIO書き込み
    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF50          => self.bootrom.write(addr, val),
            0xFF80..=0xFFFE => self.hram.write(addr, val),
            _ => (),
        }
    }
}

