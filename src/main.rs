
mod bootrom;
mod hram;

use bootrom::Bootrom;
use hram::HRam;

// 表示用ライブラリ
use embedded_graphics::{prelude::*, pixelcolor::BinaryColor};
use embedded_graphics_simulator::{SimulatorDisplay,Window, OutputSettingsBuilder};


fn main() {
    // 疑似ディスプレイ
    let display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(320,240));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Debug", &output_settings);

    let bootrom = Bootrom::new();
    println!("boot {:x}!", bootrom.read(0x00));

    let mut hram = HRam::new();
    hram.write(0x00, 0x01);
    println!("hram {:x}!", hram.read(0x00));



    // 表示
    window.show_static(&display);
}