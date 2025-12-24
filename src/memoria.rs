static BOOT_ROM: &[u8; 256] = include_bytes!("bios.bin");


pub struct Bus {
    // Memoria interna de la consola
    boot_rom: &'static [u8; 256],
    vram: [u8; 8192],      // 0x8000 - 0x9FFF
    wram: [u8; 8192],      // 0xC000 - 0xDFFF (incluye el echo ram)
    oam: [u8; 160],        // 0xFE00 - 0xFE9F
    hram: [u8; 127],       // 0xFF80 - 0xFFFE
    
    // El juego (Cartucho)
    // Para juegos básicos sin MBC, son 32KB fijos.
    rom: [u8; 32768],      
    
    // Control
    boot_enabled: bool,
}

impl Bus {
    pub fn new() -> Self {
        Bus { 
            boot_rom: BOOT_ROM,
            vram: [0; 8192],
            wram: [0; 8192],
            oam: [0; 160],
            hram: [0; 127],
            rom: [0; 32768],
            boot_enabled: true,
        }
    }


}