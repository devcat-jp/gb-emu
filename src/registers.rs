// CPUレジスタ
#![allow(dead_code)]


#[derive(Default, Clone)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    // AFの組み合わせのレジスタ、Fが下位ビット
    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f as u16)
    }

    // BCの組み合わせのレジスタ、Cが下位ビット
    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    // DEの組み合わせのレジスタ、Eが下位ビット
    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    // HLの組み合わせのレジスタ、Lが下位ビット
    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    // AFへの書き込み
    pub fn write_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xF0) as u8;    // Fの下位4bitは未使用で常に0らしい
    }
    
    // BCへの書き込み
    pub fn write_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    // DEへの書き込み
    pub fn write_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    // HLへの書き込み
    pub fn write_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    // F（フラグレジスタ）の各ビットが立っているかを取得する
    // Z（7bit目）は演算結果が0の場合に1になる
    pub fn zf(&self) -> bool {
        (self.f & 0b_1000_0000) > 0
    }
    // N（6bit目）は減算命令の場合に1になる
    pub fn nf(&self) -> bool {
        (self.f & 0b_0100_0000) > 0
    }
    // H（5bit目）は3bit目で繰り上がり（下がり）が発生すると1になる
    pub fn hf(&self) -> bool {
        (self.f & 0b_0010_0000) > 0
    }
    // C（4bit目）は7bit目で繰り上がり（下がり）が発生すると1になる
    pub fn cf(&self) -> bool {
        (self.f & 0b_0001_0000) > 0
    }

    // F（フラグレジスタ）をセットする
    // Z（7bit目）
    pub fn set_zf(&mut self, zf: bool){
        if zf {
            self.f |= 0b_1000_0000;     // 7bit目を立てる
        } else {
            self.f &= 0b_0111_1111;     // 7bit目を下げる
        }
    }
    // N（6bit目）
    pub fn set_nf(&mut self, nf: bool){
        if nf {
            self.f |= 0b_0100_0000;     // 6bit目を立てる
        } else {
            self.f &= 0b_1011_1111;     // 6bit目を下げる
        }
    }
    // H（5bit目）
    pub fn set_hf(&mut self, hf: bool){
        if hf {
            self.f |= 0b_0010_0000;     // 5bit目を立てる
        } else {
            self.f &= 0b_1101_1111;     // 5bit目を下げる
        }
    }
    // C（4bit目）
    pub fn set_cf(&mut self, cf: bool){
        if cf {
            self.f |= 0b_0001_0000;     // 4bit目を立てる
        } else {
            self.f &= 0b_1110_1111;     // 4bit目を下げる
        }
    }
    

}