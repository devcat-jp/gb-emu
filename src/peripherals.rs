// 周辺機器管理
#![allow(dead_code)]

use crate::{
    bootrom::Bootrom, cartridge::Cartridge, cpu::interrupts::Interrupts, hram::HRam, ppu::Ppu
};

pub struct Peripherals {
    cartridge: Cartridge,
    bootrom: Bootrom,
    hram: HRam,
    pub ppu: Ppu,
}

impl Peripherals {
    pub fn new (bootrom: Bootrom, cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            bootrom,
            hram: HRam::new(),
            ppu: Ppu::new(),
        }
    }

    // MMIO読み込み
    pub fn read(&self, interrupts: &Interrupts, addr: u16) -> u8 {
        // dbg
        //println!("mmio read: {:x}", addr);
        
        match addr {
            // ブートROMが無効の時はカートリッジ
            0x0000..=0x00FF => if self.bootrom.is_active() {
                self.bootrom.read(addr)
            } else {
                self.cartridge.read(addr)
            },
            0x0100..=0x7FFF => self.cartridge.read(addr),
            0xA000..=0xBFFF => self.cartridge.read(addr), 
            0x8000..=0x9FFF => self.ppu.read(addr),
            0xFE00..=0xFE9F => self.ppu.read(addr),
            0xFF40..=0xFF4B => self.ppu.read(addr),
            0xFF80..=0xFFFE => self.hram.read(addr),
            0xFF0F          => interrupts.read(addr),
            0xFFFF          => interrupts.read(addr),
            _ => 0xFF
        }
    }

    // MMIO書き込み
    pub fn write(&mut self, interrupts: &mut Interrupts, addr: u16, val: u8) {
        match addr {
            0xFF50          => self.bootrom.write(addr, val),
            0x0100..=0x7FFF => self.cartridge.write(addr, val),
            0xA000..=0xBFFF => self.cartridge.write(addr, val), 
            0x8000..=0x9FFF => self.ppu.write(addr, val),
            0xFE00..=0xFE9F => self.ppu.write(addr, val),
            0xFF40..=0xFF4B => self.ppu.write(addr, val),
            0xFF80..=0xFFFE => self.hram.write(addr, val),
            0xFF0F          => interrupts.write(addr, val),
            0xFFFF          => interrupts.write(addr, val),
            _ => (),
        }
    }
}

