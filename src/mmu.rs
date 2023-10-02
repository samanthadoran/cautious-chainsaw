use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

use crate::memory::{GameboyAddress, GameboyAddressRegion};

const BOOTSTRAP_SIZE: usize = 256;

struct Bootstrap {
    data: [u8; BOOTSTRAP_SIZE],
    mapped: bool,
}

impl Bootstrap {
    pub fn new(path: &Path) -> Result<Bootstrap, Error> {
        let mut file = File::open(path)?;
        let mut data: [u8; BOOTSTRAP_SIZE] = [0; BOOTSTRAP_SIZE];
        let num_read_bytes = file.read(&mut data)?;
        if num_read_bytes == BOOTSTRAP_SIZE {
            Ok(Bootstrap { data, mapped: true })
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid Bootstrap Size, expected 256 bytes.",
            ))
        }
    }

    pub fn read8(&self, offset: u16) -> u8 {
        if !self.mapped {
            panic!("Trying to read from bootstrap when it isn't mapped?");
        }
        self.data[offset as usize]
    }

    pub fn read16(&self, offset: u16) -> u16 {
        self.read8(offset) as u16 | ((self.read8(offset + 1) as u16) << 8)
    }
}

pub struct Mmu {
    bootstrap: Bootstrap,
}

impl Mmu {
    pub fn new(path: &Path) -> Mmu {
        Mmu {
            bootstrap: match Bootstrap::new(path) {
                Ok(bootstrap) => bootstrap,
                Err(e) => panic!("{:?}", e),
            },
        }
    }
    pub fn read(&self, address: GameboyAddress) -> u8 {
        match address.region {
            GameboyAddressRegion::Bootstrap(offset) => self.bootstrap.read8(offset),
            _ => todo!("Implement more places!"),
        }
    }
}
