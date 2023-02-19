use crate::io::IOPort;

const MASTER_PORT: u16 = 0x20;
const SLAVE_PORT: u16 = 0xA0;

pub const MASTER_OFFSET: u8 = 0x30;
pub const SLAVE_OFFSET: u8 = 0x40;

#[derive(Copy, Clone)]
pub enum Irq {
    Pit = 0,
    Keyboard = 1,
}

#[derive(Copy, Clone)]
pub enum IrqState {
    Mask,
    Unmask,
}

pub fn init() {
    // port A
    let master_port: IOPort<u8> = IOPort::new(MASTER_PORT);
    let slave_port: IOPort<u8> = IOPort::new(SLAVE_PORT);

    master_port.write(0x11);
    slave_port.write(0x11);

    // Port B
    let master_port: IOPort<u8> = IOPort::new(MASTER_PORT + 1);
    let slave_port: IOPort<u8> = IOPort::new(SLAVE_PORT + 1);

    master_port.write(MASTER_OFFSET);
    slave_port.write(SLAVE_OFFSET);

    master_port.write(0x4);
    slave_port.write(0x2);

    master_port.write(0x1);
    slave_port.write(0x1);

    master_port.write(0xFF);
    slave_port.write(0xFF);
}

pub fn set_irq_state(irq: Irq, state: IrqState) {
    // For now, we only support IRQ on master port
    let port: IOPort<u8> = IOPort::new(MASTER_PORT + 1);

    let value = port.read();

    let to_write = match state {
        IrqState::Mask => value | (1 << (irq as u8)),
        IrqState::Unmask => value & !(1 << (irq as u8)),
    };

    port.write(to_write);
}

pub fn eoi_master() {
    let master_port: IOPort<u8> = IOPort::new(MASTER_PORT);
    master_port.write(0x20);
}

pub fn eoi_slave() {
    let slave_port: IOPort<u8> = IOPort::new(SLAVE_PORT);
    slave_port.write(0x20);
}
