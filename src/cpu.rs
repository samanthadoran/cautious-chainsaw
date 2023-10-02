use core::fmt;
use std::path::Path;

use crate::memory::GameboyAddress;
use crate::mmu::Mmu;

#[derive(Debug)]
pub struct Instruction {
    address: GameboyAddress,
    dissassembly: String,
    cycles: u8,
    length: usize,
    immediate16: Option<u16>,
    pub execute: fn(&mut Cpu, &Self),
}

impl Default for Instruction {
    fn default() -> Self {
        let address = GameboyAddress::new(0x0, true);
        Instruction {
            address,
            dissassembly: format!("NOP ;${:#04x}", address.address),
            cycles: 4,
            length: 1,
            immediate16: None,
            execute: Cpu::nop,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dissassembly)
    }
}
enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

pub struct Cpu {
    pub pc: GameboyAddress,
    sp: GameboyAddress,

    // Registers
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,

    //Interconnect style memory dispatch
    mmu: Mmu,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: GameboyAddress::new(0x0, true),
            sp: GameboyAddress::new(0x0, true),
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            mmu: Mmu::new(Path::new("roms/dmg/bootstrap/dmg_rom.bin")),
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let result = self.mmu.read(self.pc);
        self.pc += 1;
        result
    }

    pub fn decode(&mut self, instruction_byte: u8, address: GameboyAddress) -> Instruction {
        match instruction_byte {
            0x31 => {
                let immediate_lower = self.fetch() as u16;
                let immediate_upper = self.fetch() as u16;
                let immediate16 = immediate_lower | (immediate_upper << 8);
                Instruction {
                    address,
                    dissassembly: format!("LD SP, {:#04x} ;${:#04x}", immediate16, address.address)
                        .to_owned(),
                    cycles: 12,
                    length: 3,
                    immediate16: Some(immediate16),
                    execute: Cpu::ld_sp,
                }
            }
            0xAF => Instruction {
                address,
                dissassembly: format!("XOR A ;${:#04x}", address.address).to_owned(),
                cycles: 4,
                length: 1,
                immediate16: None,
                execute: Cpu::xor_a,
            },
            0x21 => {
                let immediate_lower = self.fetch() as u16;
                let immediate_upper = self.fetch() as u16;
                let immediate16 = immediate_lower | (immediate_upper << 8);
                Instruction {
                    address,
                    dissassembly: format!("LD HL, {:#04x} ;${:#04x}", immediate16, address.address)
                        .to_owned(),
                    cycles: 12,
                    length: 3,
                    immediate16: Some(immediate16),
                    execute: Cpu::ld_hl,
                }
            }
            instruction_byte => unimplemented!("{:#01x} is unimplemented.", instruction_byte),
        }
    }

    pub fn set_flags(&mut self, result: u8) {
        let mut flags: u8 = 0;

        // Z
        if result == 0 {
            flags |= 0x80;
        }

        // H
        // TODO(Samantha): Implement.

        // N
        // TODO(Samantha): Implement.

        // C
        // TODO(Samantha): Implement.

        self.write_register(Register::F, flags);
    }

    pub fn nop(&mut self, _: &Instruction) {}

    pub fn xor_a(&mut self, _: &Instruction) {
        let zero = self.read_register(Register::A) ^ self.read_register(Register::A);
        self.write_register(Register::A, zero);
        self.set_flags(zero);
    }

    pub fn ld_sp(&mut self, instruction: &Instruction) {
        match instruction.immediate16 {
            Some(immediate16) => self.write_register_16(Register16::SP, immediate16),
            None => panic!("LD SP should always have an immediate 16!"),
        }
    }

    pub fn ld_hl(&mut self, instruction: &Instruction) {
        match instruction.immediate16 {
            Some(immediate16) => self.write_register_16(Register16::HL, immediate16),
            None => panic!("LD HL should always have an immediate 16!"),
        }
    }

    fn read_register(&self, register: Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::F => self.f,
            Register::H => self.h,
            Register::L => self.l,
        }
    }

    fn write_register(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            // Lower 4 bits of F must always be zero.
            Register::F => self.f = value & 0xF0,
            Register::H => self.h = value,
            Register::L => self.l = value,
        }
    }

    fn write_register_16(&mut self, register: Register16, value: u16) {
        match register {
            Register16::AF => (),
            Register16::BC => (),
            Register16::DE => (),
            Register16::HL => {
                self.write_register(Register::H, (value >> 8) as u8);
                self.write_register(Register::L, (value & 0x00FF) as u8)
            }
            Register16::SP => self.sp = GameboyAddress::new(value, true),
        }
    }

    fn read_register_16(self, register: Register16) -> u16 {
        match register {
            Register16::AF => (self.a as u16) << 8 | (self.f as u16),
            Register16::BC => (self.b as u16) << 8 | (self.c as u16),
            Register16::DE => (self.d as u16) << 8 | (self.e as u16),
            Register16::HL => (self.h as u16) << 8 | (self.l as u16),
            Register16::SP => self.sp.address,
        }
    }
}
