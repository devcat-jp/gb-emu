
#![allow(dead_code)]


use std::{
    env,
    fs::File,
    io::Read,
    process::exit,
};


mod cpu;
mod ppu;
mod lcd;
mod mbc;
mod hram;
mod bootrom;
mod cartridge;
mod registers;
mod peripherals;


use crate::{
    cpu::Cpu,
    lcd::Lcd,
    //mbc::Mbc,
    bootrom::Bootrom,
    cartridge::Cartridge,
    peripherals::Peripherals,
};


pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;
pub const LCD_PIXELS: usize = LCD_WIDTH * LCD_HEIGHT;

const CPU_CLOCK_HZ: u128 = 4_194_304;
const M_CYCLE_CLOCK: u128 = 4;
const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;


fn main() {
    // 起動パラメータ確認
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("The file name argument is required.");
        exit(1);
    }

    // カードリッジ読み込み
    fn file2vec(fname: &String) -> Vec<u8> {
        if let Ok(mut file) = File::open(fname) {
          let mut ret = vec![];
          file.read_to_end(&mut ret).unwrap();
          ret
        } else {
          panic!("Cannot open {}.", fname);
        }
    }
    let cartridge_raw = file2vec(&args[1]);
    let cartridge = Cartridge::new(cartridge_raw);

    // エミュレータ作成
    let mut lcd = Lcd::new();
    let mut cpu = Cpu::new();
    let bootrom = Bootrom::new();
    let mut peripherals = Peripherals::new(bootrom, cartridge);

    //let time = time::Instant::now;
    //let mut elapsed = 0;
    let is_lcd = false;
    loop {
        cpu.emulate_cycle(&mut peripherals); 

        if peripherals.ppu.emulate_cycle() {
            // 画面表示
            if is_lcd {
                lcd.draw(&peripherals.ppu.buffer);
                lcd.updata();
            }
        }
    }


   
}