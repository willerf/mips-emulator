
use crate::memory::Memory;

pub const PC: u32 = 32;

pub const PRINT_ADDR: u32 = 0b11111111111111110000000000001100;

pub struct RV32IState {
    mem: Memory,
    regs: Vec<u32>,
}

impl RV32IState {
    pub fn new(mem: Memory) -> RV32IState {
        RV32IState {
            mem: mem,
            regs: vec![0; 33],
        }
    }

    pub fn read_reg(&self, addr: u32) -> u32 {
        assert!(addr < 33);
        self.regs[addr as usize]
    }

    pub fn write_reg(&mut self, addr: u32, word: u32) {
        assert!(addr < 33);
        
        if addr != 0 {
            self.regs[addr as usize] = word;
        }
    }

    pub fn read_mem(&self, addr: u32) -> u32 {
        ((self.mem.read(addr) as u32) << 0) 
            | ((self.mem.read(addr+1) as u32) << 8)
            | ((self.mem.read(addr+2) as u32) << 16)
            | ((self.mem.read(addr+3) as u32) << 24)
    }

    pub fn write_mem(&mut self, addr: u32, word: u32) {
        if addr == PRINT_ADDR {
            print!("{}", char::from_u32(word).expect("Error"));
        }
        else {
            self.mem.write(addr, (word >> 0) as u8);
            self.mem.write(addr+1, (word >> 8) as u8);
            self.mem.write(addr+2, (word >> 16) as u8);
            self.mem.write(addr+3, (word >> 24) as u8);
        }
    }

    pub fn size(&self) -> u32 {
        self.mem.size()
    }

}