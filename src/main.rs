
#![allow(dead_code)]

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

    // 疑似ディスプレイ
    let display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(320,240));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Debug", &output_settings);

    let bootrom = Bootrom::new();
    let mut peripherals = Peripherals::new(bootrom);


    // cpu
    let mut cpu = Cpu::new();

    for _ in 0..100000 {
        cpu.emulate_cycle(&mut peripherals); 
    }

   

    // 表示
    window.show_static(&display);
}