#![allow(dead_code)]

// 表示用ライブラリ
use embedded_graphics::{image::ImageRaw, pixelcolor::Gray8, prelude::*};
use embedded_graphics_simulator::{SimulatorDisplay,Window, OutputSettingsBuilder};

use crate::LCD_WIDTH;

pub struct Lcd {
    display: SimulatorDisplay<Gray8>,
    window: Window,
}

impl Lcd {
    pub fn new() -> Self{
        let output_settings = OutputSettingsBuilder::new().build();
        Self {
            display: SimulatorDisplay::new(Size::new(160,144)),
            window: Window::new("Debug", &output_settings),
        }
    }

    // 画面描画
    pub fn draw(&mut self, pixcles: &Vec<u8>) {
        let raw = ImageRaw::<Gray8>::new(pixcles, LCD_WIDTH as u32);
        //let image = Image::new(&data, Point::zero());
        let _ = raw.draw(&mut self.display);
        
    }

    // 画面更新
    pub fn updata(&mut self) {
        self.window.update(&self.display);
    }
}