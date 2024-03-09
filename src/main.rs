
#![allow(dead_code)]

//use std::fs::File;
//use std::io::{Read, BufReader};

mod cpu;
mod hram;
mod bootrom;
mod registers;
mod peripherals;

use cpu::Cpu;
use bootrom::Bootrom;
use peripherals::Peripherals;

// 表示用ライブラリ
use embedded_graphics::{prelude::*, pixelcolor::BinaryColor};
use embedded_graphics_simulator::{SimulatorDisplay,Window, OutputSettingsBuilder};


fn main() {

    /*
    let mut _rom: Vec<u8> = Vec::with_capacity(256);
    let mut _cnt: u16 = 0;

    let f = File::open("dmg_bootrom.bin").unwrap();
    let mut bytes = BufReader::new(f).bytes();
    while let Some(Ok(b)) = bytes.next() {
        _rom.push(b);
    }

    for _ in 0..32 {
        for _ in 0..8 {
            print!("0x{:>02x}, ", _rom[_cnt as usize]);
            _cnt = _cnt + 1;
        }
        println!("");
    }
    */
    

    // 疑似ディスプレイ
    let display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(320,240));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Debug", &output_settings);

    let bootrom = Bootrom::new();
    let mut peripherals = Peripherals::new(bootrom);


    // cpu
    let mut cpu = Cpu::new();

    for _ in 0..14 {
        cpu.emulate_cycle(&mut peripherals); 
    }

   



    // 表示
    window.show_static(&display);
}