// 128byteのRAM


pub struct HRam {
    hram: Vec<u8>,           // u8の配列
}

impl HRam {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self{
        hram: vec![0; 0x80]
    }
  }

  // RAM 1byte読み出し
  #[allow(dead_code)]
  pub fn read(&self, addr: u16) -> u8 {
    self.hram[(addr as usize) & 0x7f]
  }
  
  // RAM 1byte書き込み
  #[allow(dead_code)]
  pub fn write(&mut self, addr: u16, val: u8) {
    self.hram[(addr as usize) & 0x7f] = val;
  }
}