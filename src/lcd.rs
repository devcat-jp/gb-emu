#![allow(dead_code)]

// 表示用ライブラリ

use embedded_graphics::{prelude::*, pixelcolor::Rgb888};//pixelcolor::BinaryColor};
use embedded_graphics_simulator::{SimulatorDisplay,Window, OutputSettingsBuilder};
//use tinybmp::Bmp;

use crate::{LCD_HEIGHT, LCD_WIDTH};


pub struct Lcd {
    display: SimulatorDisplay<Rgb888>,
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
        let mut cnt = 0;
        for t in 0..LCD_HEIGHT {
            for s in 0..LCD_WIDTH {
                match pixcles[cnt] {
                    0xFF => {
                        let _= Pixel(Point::new(s as i32, t as i32), Rgb888::WHITE).draw(&mut self.display);
                    },
                    0xAA => {
                        let _= Pixel(Point::new(s as i32, t as i32), Rgb888::BLUE).draw(&mut self.display);
                    },
                    0x55 => {
                        let _= Pixel(Point::new(s as i32, t as i32), Rgb888::CSS_VIOLET).draw(&mut self.display);
                    },
                    0x00 => {
                        let _= Pixel(Point::new(s as i32, t as i32), Rgb888::BLACK).draw(&mut self.display);
                    },
                    _ => {
                        
                    }, 
                }
                cnt += 1;
            }
        }
        
    }

    // 画面更新
    pub fn updata(&mut self) {
        self.window.update(&self.display);
    }
}