
#![allow(dead_code)]

mod hram;
mod bootrom;
mod peripherals;
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

    // bootrom
    println!("boot_chk1 {:x}", peripherals.read(0x0000));
    peripherals.write(0xFF50, 1);

    // hram
    peripherals.write(0xFF80, 0xA2);
    println!("hram_chk1 {:x}", peripherals.read(0xFF80));


    

    //println!("boot_chk1 {:x}", bootrom.read(0x00));
    //println!("boot_chk2 {}", bootrom.is_active());
    //bootrom.set_active(false);
    //println!("boot_chk3 {}", bootrom.is_active());


    //let mut hram = HRam::new();
    //hram.write(0x00, 0x01);
    //println!("hram {:x}", hram.read(0x00));





    // 表示
    window.show_static(&display);
}