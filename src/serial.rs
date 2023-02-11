use crate::io::IOPort;

pub const COM1: u16 = 0x3F8;
pub const COM2: u16 = 0x2F8;
pub const COM3: u16 = 0x3E8;
pub const COM4: u16 = 0x2E8;
pub const COM5: u16 = 0x5F8;
pub const COM6: u16 = 0x4F8;
pub const COM7: u16 = 0x5E8;
pub const COM8: u16 = 0x4E8;

const FREQ: u32 = 115200;

const DLAB_SET: u8 = 0x1 << 7;
const WORD_8_BITS: u8 = 0x3;
const FIFO_ENABLE: u8 = 0x1;
const FIFO_CLEAR_RECEIVE: u8 = 0x1 << 1;
const FIFO_CLEAR_TRANSMIT: u8 = 0x1 << 2;
const FIFO_TRIGER_14: u8 = 0xC0;
const LSR_READY_WRITE: u8 = 0x1 << 5;
const LSR_READY_READ: u8 = 0x1;

pub enum SerialError {
    InvalidBaudrate,
}

pub type Result<T> = core::result::Result<T, SerialError>;

pub struct Serial {
    dll: IOPort<u8>,
    dlh: IOPort<u8>,
    fcr: IOPort<u8>,
    lcr: IOPort<u8>,
    lsr: IOPort<u8>,
}

impl Serial {
    pub fn new(base_port: u16, baudrate: u32) -> Result<Self> {
        let serial = Serial {
            dll: IOPort::new(base_port),
            dlh: IOPort::new(base_port + 1),
            fcr: IOPort::new(base_port + 2),
            lcr: IOPort::new(base_port + 3),
            lsr: IOPort::new(base_port + 5),
        };

        let divisor_latch_value: [u8; 2] = match u16::try_from(FREQ / baudrate) {
            Ok(n) => n.to_be_bytes(),
            Err(_) => return Err(SerialError::InvalidBaudrate),
        };

        // Set dlab to 1
        serial.lcr.write(DLAB_SET);

        serial.dll.write(divisor_latch_value[0]);
        serial.dlh.write(divisor_latch_value[1]);

        // Restore dlab to 0, 8 bits word length and no parity
        serial.lcr.write(WORD_8_BITS);

        // Send interuptions
        serial.fcr.write(FIFO_TRIGER_14);

        Ok(serial)
    }

    /// Tells if the UART transmitter is ready to accept a new byte.
    pub fn ready_to_write(&self) -> bool {
        (self.lsr.read() & LSR_READY_WRITE) != 0
    }

    /// Tells if there a byte to read
    pub fn ready_to_read(&self) -> bool {
        (self.lsr.read()) & LSR_READY_READ != 0
    }

    /// Wait until the UART is ready to accept a new char and write it.
    pub fn write_byte(&self, data: u8) {
        while !self.ready_to_write() {}

        self.dll.write(data);
    }

    /// Wait until a byte is ready to be readed and read it.
    pub fn read_byte(&self) -> u8 {
        while !self.ready_to_read() {}

        self.dll.read()
    }
}
