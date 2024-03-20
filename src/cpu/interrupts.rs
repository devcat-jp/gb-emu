
pub const VBLANK: u8 = 1 << 0;      // PPUがモード1に入るたびに呼ばれる
pub const STAT: u8 = 1 << 1;        // PPUに要求される
pub const TIMER: u8 = 1 << 2;       // TimerのTIMAレジスタがオーバーフローするたびに呼ばれる
pub const SERIAL: u8 = 1 << 3;      // シリアル通信により要求される
pub const JOYPAD: u8 = 1 << 4;      // ジョイパッドでボタン操作のたびに呼ばれる


#[derive(Default, Clone)]
pub struct Interrupts {
    pub ime: bool,          // 割り込みを許可・禁止するためのレジスタ
    pub int_flags: u8,      // 割り込みを有効化・無効化するためのレジスタ
    pub int_enables: u8,    // 割り込み要求を管理するレジスタ
}

impl Interrupts {
    // 割り込み要求
    pub fn irq(&mut self, val: u8) {
        self.int_flags |= val;
    }

    // 割り込み状況取得
    pub fn get_interrupt(&self) -> u8 {
        self.int_flags & self.int_enables & 0b11111
    }

    // 読み込み
    pub fn read(&self, addr: u16) -> u8 {
        match  addr {
            0xFF0F => self.int_flags,
            0xFFFF => self.int_enables,
            _      => panic!("Not Define {:x}", addr),    
        }
    }

    // 書き込み
    pub fn write(&mut self, addr: u16, val: u8) {
        match  addr {
            0xFF0F => self.int_flags = val,
            0xFFFF => self.int_enables = val,
            _      => panic!("Not Define {:x}", addr),    
        }
    }
}

