
#![allow(dead_code)]

mod cpu;
mod ppu;
mod lcd;
mod hram;
mod bootrom;
mod registers;
mod peripherals;

use crate::{
    cpu::Cpu,
    lcd::Lcd,
    bootrom::Bootrom,
    peripherals::Peripherals,
};


pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;
pub const LCD_PIXELS: usize = LCD_WIDTH * LCD_HEIGHT;

const CPU_CLOCK_HZ: u128 = 4_194_304;
const M_CYCLE_CLOCK: u128 = 4;
const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;


fn main() {

    let mut lcd = Lcd::new();
    let mut cpu = Cpu::new();
    let bootrom = Bootrom::new();
    let mut peripherals = Peripherals::new(bootrom);

    //let time = time::Instant::now;
    //let mut elapsed = 0;
    loop {
        cpu.emulate_cycle(&mut peripherals); 

        if peripherals.ppu.emulate_cycle() {
            // 画面表示
            lcd.draw(&peripherals.ppu.buffer);
            lcd.updata();
            //println!("{:x}", peripherals.ppu.buffer());
        }
    }


   
}