use crate::memory::PAGE_SIZE;

#[derive(Debug)]
pub enum AddressError {
    InvalidOffset,
}

pub type Result<T> = core::result::Result<T, AddressError>;

pub struct Address {
    address: u64,
    is_phys: bool,
}

pub const NULL: Address = Address {
    address: 0x0,
    is_phys: true,
};

impl Address {
    pub fn from_physical(address: u64) -> Self {
        Address {
            address,
            is_phys: true,
        }
    }

    pub fn from_virt(address: u64) -> Self {
        Address {
            address,
            is_phys: false,
        }
    }

    pub fn align_up(&mut self) {
        self.address = (self.address + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    }

    pub fn align_down(&mut self) {
        self.address = self.address & !(PAGE_SIZE - 1);
    }

    pub fn is_align(&self) -> bool {
        self.address & !(PAGE_SIZE - 1) == 0
    }

    pub fn get_offset(&self, address: &Address) -> Result<u64> {
        if self.address > address.address {
            return Err(AddressError::InvalidOffset);
        }

        Ok(address.address - self.address)
    }

    pub fn get(&self) -> u64 {
        self.address
    }
}
