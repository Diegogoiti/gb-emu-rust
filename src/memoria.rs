static BIOS: &[u8; 256] = include_bytes!("bios.bin");
static ROM : &[u8] = include_bytes!("rom.gb");


pub struct Bus {
    // Memoria interna de la consola
    boot_rom: &'static [u8; 256],
    vram: [u8; 8192],      // 0x8000 - 0x9FFF
    wram: [u8; 8192],      // 0xC000 - 0xDFFF (incluye el echo ram)
    oam: [u8; 160],        // 0xFE00 - 0xFE9F
    hram: [u8; 127],       // 0xFF80 - 0xFFFE
    
    // El juego (Cartucho)
    // Para juegos básicos sin MBC, son 32KB fijos.
    rom: &'static [u8],
    
    // Control
    boot_enabled: bool,
}

impl Bus {
    pub fn new() -> Self {
        Bus { 
            boot_rom: BIOS,
            vram: [0; 8192],
            wram: [0; 8192],
            oam: [0; 160],
            hram: [0; 127],
            rom: ROM,
            boot_enabled: true,
        }
    }

    pub fn read(&self, pos: u16) -> u8 {
        match pos {
            0x0000..=0x00FF if self.boot_enabled => self.boot_rom[pos as usize],

            0x0000..=0x7FFF => if (pos as usize) < self.rom.len() {
            self.rom[pos as usize]
            } else {
                0xFF // Si el PC se sale del juego, devolvemos "vacío"
            },

            0x8000..=0x9FFF => self.vram[(pos - 0x8000) as usize],

            0xC000..=0xDFFF => self.wram[(pos - 0xC000) as usize],

            0xFE00..=0xFE9F => self.oam[(pos - 0xFE00) as usize],

            0xFF80..=0xFFFE => self.hram[(pos - 0xFF80) as usize],
            _ => 0xff, // Direcciones no implementadas
        }

    }

    pub fn write(&mut self, pos: u16, valor: u8) {
        match pos {
            0x8000..=0x9FFF => self.vram[(pos - 0x8000) as usize] = valor,

            0xC000..=0xDFFF => self.wram[(pos - 0xC000) as usize] = valor,

            0xFE00..=0xFE9F => self.oam[(pos - 0xFE00) as usize] = valor,

            0xFF80..=0xFFFE => self.hram[(pos - 0xFF80) as usize] = valor,

            0xFF50 => {
                if valor != 0 {
                    self.boot_enabled = false;
                }
}
            _ => {}, // Direcciones no implementadas o de solo lectura
        }
    }


}