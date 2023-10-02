use std::ops::{AddAssign, Range};

#[derive(Debug, Copy, Clone)]
pub enum GameboyAddressRegion {
    Bootstrap(u16),
    CartridgeBankZero(u16),
    CartridgeBankSwitchable(u16),
    Vram(u16),
    CartridgeRam(u16),
    WorkRam(u16),
    WorkRam2(u16),
    ProhibitedEchoRam(u16),
    ObjectAttributeMemory(u16),
    Prohibited(u16),
    IORegisters(u16),
    HiRam(u16),
    InterruptEnableRegister,
}

impl GameboyAddressRegion {
    pub fn new(address: u16, bootstrap_mapped: bool) -> GameboyAddressRegion {
        let bootstrap: Range<u16> = 0x000..0x0100;
        let cartridge_bank_zero: Range<u16> = 0x0000..0x4000;
        let cartridge_bank_switchable: Range<u16> = 0x4000..0x8000;
        let vram: Range<u16> = 0x8000..0xA000;
        let cartridge_ram: Range<u16> = 0xA000..0xC000;
        let work_ram: Range<u16> = 0xC000..0xD000;
        let work_ram_2: Range<u16> = 0xD000..0xE000;
        let prohibited_echo_ram: Range<u16> = 0xE000..0xFE00;
        let object_attribute_memory: Range<u16> = 0xFE00..0xFEA0;
        let prohibited: Range<u16> = 0xFEA0..0xFF00;
        let io_registers: Range<u16> = 0xFF00..0xFF80;
        let hi_ram: Range<u16> = 0xFF80..0xFFFE;
        match address {
            // Cartridge bank zero
            0x0000..=0x3FFF => {
                if bootstrap_mapped {
                    if !bootstrap.contains(&address) {
                        panic!("Accessing outside of bootstrap region in bank zero while mapped!");
                    }
                    let offset = address - bootstrap.start;
                    GameboyAddressRegion::Bootstrap(offset)
                } else {
                    let offset = address - cartridge_bank_zero.start;
                    GameboyAddressRegion::CartridgeBankZero(offset)
                }
            }
            // Switchable cartridge bank
            0x4000..=0x7FFF => {
                let offset = address - cartridge_bank_switchable.start;
                GameboyAddressRegion::CartridgeBankSwitchable(offset)
            }
            // VRAM
            0x8000..=0x9FFF => {
                let offset = address - vram.start;
                GameboyAddressRegion::Vram(offset)
            }
            // Cartridge Ram
            0xA000..=0xBFFF => {
                let offset = address - cartridge_ram.start;
                GameboyAddressRegion::CartridgeRam(offset)
            }
            0xC000..=0xCFFF => {
                let offset = address - work_ram.start;
                GameboyAddressRegion::WorkRam(offset)
            }
            0xD000..=0xDFFF => {
                let offset = address - work_ram_2.start;
                GameboyAddressRegion::WorkRam2(offset)
            }
            0xE000..=0xFDFF => {
                let offset = address - prohibited_echo_ram.start;
                GameboyAddressRegion::ProhibitedEchoRam(offset)
            }
            0xFE00..=0xFE9F => {
                let offset = address - object_attribute_memory.start;
                GameboyAddressRegion::ObjectAttributeMemory(offset)
            }
            0xFEA0..=0xFEFF => {
                let offset = address - prohibited.start;
                GameboyAddressRegion::Prohibited(offset)
            }
            0xFF00..=0xFF7F => {
                let offset = address - io_registers.start;
                GameboyAddressRegion::IORegisters(offset)
            }
            0xFF80..=0xFFFE => {
                let offset = address - hi_ram.start;
                GameboyAddressRegion::HiRam(offset)
            }
            0xFFFF => GameboyAddressRegion::InterruptEnableRegister,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GameboyAddress {
    pub region: GameboyAddressRegion,
    pub address: u16,
}

impl AddAssign<u16> for GameboyAddress {
    fn add_assign(&mut self, other: u16) {
        let new_address = self.address + other;
        *self = Self {
            region: GameboyAddressRegion::new(new_address, true),
            address: new_address,
        };
    }
}

impl GameboyAddress {
    pub fn new(address: u16, bootstrap_mapped: bool) -> GameboyAddress {
        GameboyAddress {
            region: GameboyAddressRegion::new(address, bootstrap_mapped),
            address,
        }
    }
}
