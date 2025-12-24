use crate::memoria::Bus;



pub struct CPU {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            a: 0, f: 0, b: 0, c: 0,
            d: 0, e: 0, h: 0, l: 0,
            pc: 0x0000, // Punto de entrada de la BIOS
            sp: 0x0000,
        }
    }

    fn fetch(&mut self, bus: &Bus) -> u8 {
        let byte = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1); 
        byte
    }

    pub fn execute(&mut self, opcode: u8, bus: &mut Bus) {
    match opcode {
        // LD SP, d16 (Cargar valor de 16 bits en SP)
        0x31 => {
            let bajo = self.fetch(bus) as u16;  // Paso 2: Parte baja
            let alto = self.fetch(bus) as u16;  // Paso 3: Parte alta
            
            self.sp = (alto << 8) | bajo;       // Armamos el 16-bit
        }


        
        _ => println!("Instrucción no implementada: {:#04X}", opcode),
    }
}

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn fetch_u16(&mut self, bus: &Bus) -> u16 {
        let bajo = self.fetch(bus) as u16;
        let alto = self.fetch(bus) as u16;
        (alto << 8) | bajo
    }
}
