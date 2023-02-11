use core::arch::asm;
use core::marker::PhantomData;

pub trait IO {
    fn write_port(port: u16, data: Self);
    fn read_port(port: u16) -> Self;

    fn write_byte(port: u16, data: u8) {
        unsafe {
            asm!(
                "outb %al, %dx",
                in("dx") port,
                in("al") data,
                options(att_syntax)
            );
        };
    }

    fn write_word(port: u16, data: u16) {
        unsafe {
            asm!(
                "outw %ax, %dx",
                in("dx") port,
                in("ax") data,
                options(att_syntax)
            );
        };
    }

    fn read_byte(port: u16) -> u8 {
        let mut readed: u8 = 0;
        unsafe {
            asm!(
                "inb %al, %dx",
                in("dx") port,
                out("al") readed,
                options(att_syntax)
            )
        };

        readed
    }

    fn read_word(port: u16) -> u16 {
        let mut readed: u16 = 0;
        unsafe {
            asm!(
                "inw %ax, %dx",
                in("dx") port,
                out("ax") readed,
                options(att_syntax)
            )
        };

        readed
    }
}

pub struct IOPort<T: IO> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: IO> IOPort<T> {
    pub fn new(port: u16) -> Self {
        IOPort {
            port,
            phantom: PhantomData,
        }
    }

    pub fn write(&self, data: T) {
        T::write_port(self.port, data);
    }

    pub fn read(&self) -> T {
        T::read_port(self.port)
    }
}

impl IO for u8 {
    fn write_port(port: u16, data: Self) {
        Self::write_byte(port, data)
    }

    fn read_port(port: u16) -> Self {
        Self::read_byte(port)
    }
}

impl IO for u16 {
    fn write_port(port: u16, data: Self) {
        Self::write_word(port, data)
    }

    fn read_port(port: u16) -> Self {
        Self::read_word(port)
    }
}

impl IO for char {
    fn write_port(port: u16, data: Self) {
        Self::write_byte(port, data as u8)
    }

    fn read_port(port: u16) -> Self {
        Self::read_byte(port).into()
    }
}
