use stm32f429;
use stm32f429::interrupt;
use cortex_m::peripheral::NVIC;

pub enum L3GD20Error {
    SpiTimeout,
    SpiBusError,
    IdCannotBeRead,
}

#[allow(non_snake_case)]
pub enum Register {
    WHO_AM_I = 0x0F,
    CTRL_REG1 = 0x20,
    CTRL_REG2 = 0x21,
    CTRL_REG3 = 0x22,
    CTRL_REG4 = 0x23,
    CTRL_REG5 = 0x24,
    REFERENCE = 0x25,
    OUT_TEMP,
    STATUS_REG,
    OUT_X_L,
    OUT_X_H,
    OUT_Y_L,
    OUT_Y_H,
    OUT_Z_L,
    OUT_Z_H,
    FIFO_CTRL_REG,
    FIFO_SRC_REG,
    INT1_CFG,
    INT1_SRC,
    INT1_THS_XH,
    INT1_THS_XL,
    INT1_THS_YH,
    INT1_THS_ZL,
    INT1_THS_ZH,
    INT1_DURATION,
}

pub enum OpMode {
    PowerDown,
    Sleep,
    Normal,
}

pub enum Scale {
    _250Dps,
    _500Dps,
    _2000Dps,
}

static mut INSTANCE : L3GD20 = L3GD20{};

pub struct L3GD20;

impl L3GD20 {
    pub fn get_instance() -> &'static L3GD20 {
        unsafe {
            return &INSTANCE;
        }
    }

    pub fn init(&self) -> Result<(), ()> {
        // enable peripherals clocks
        let rcc = unsafe{&*stm32f429::RCC.get()};
        rcc.ahb1enr.modify(|_, w| w.gpiofen().bit(true)
                                    .gpioaen().bit(true)
                                    .gpiocen().bit(true));
        rcc.apb2enr.modify(|_, w| w.spi5enr().bit(true));

        // configure cs pin, output pull-up, default high state
        let pc = unsafe{&*stm32f429::GPIOC.get()};
        pc.moder.modify(|_, w| unsafe{w.moder1().bits(0b01)});
        pc.pupdr.modify(|_, w| unsafe{w.pupdr1().bits(0b01)});
        pc.bsrr.write(|w| w.bs1().bit(true));

        // configure spi pins
        // set as alternative function pin
        let pf = unsafe{ &*stm32f429::GPIOF.get() };
        pf.moder.modify(|_, w| unsafe{
            w.moder7().bits(0b10)
             .moder9().bits(0b10)
             .moder8().bits(0b10)
        });

        pf.otyper.modify(|_, w|
            w.ot7().bit(false)
             .ot8().bit(false)
             .ot9().bit(false)
        );

        // set spi pins speed
        pf.ospeedr.modify(|_, w| unsafe {
            w.ospeedr7().bits(0b11)
             .ospeedr9().bits(0b11)
             .ospeedr8().bits(0b11)
        });

        // set pin alternative function
        pf.afrl.modify(|_, w| unsafe{w.afrl7().bits(0b0101)});
        pf.afrh.modify(|_, w| unsafe{
            w.afrh8().bits(0b0101)
             .afrh9().bits(0b0101)
        });

        // configure spi5 for sensor interface
        let spi = unsafe{&*stm32f429::SPI5.get()};
        spi.cr1.modify(|_, w| unsafe {
            w.bidimode().bit(false)
             .crcen().bit(false)
             .dff().bit(false)
             .br().bits(0b010)
             .mstr().bit(true)
             .cpol().bit(true)
             .cpha().bit(true)
        });

        spi.cr2.modify(|_, w| w.ssoe().bit(true));

        // configure interrupts when pins changing
        // exti conf
        let sc = unsafe{&*stm32f429::SYSCFG.get()};
        sc.exticr1.modify(|_, w| unsafe{
            w.exti1().bits(0b0000)
        });

        let nvic = unsafe{&*NVIC.get()};
        nvic.enable(interrupt::Interrupt::EXTI1);

        spi.cr1.modify(|_, w| w.spe().bit(true));

        Ok(())
    }

    pub fn write_reg(&self, reg : Register, dat : u8) {
        let reg = 0x3F & reg as u8;

        let pc = unsafe{&*stm32f429::GPIOC.get()};
        pc.bsrr.write(|w| w.br1().bit(true));

        let spi = unsafe{&*stm32f429::SPI5.get()};

        spi.dr.write(|w| unsafe{w.bits(reg as u32)});
        while spi.sr.read().txe().bit() == false {}

        spi.dr.write(|w| unsafe{w.bits(dat as u32)});
        while spi.sr.read().txe().bit() == false {}

        pc.bsrr.write(|w| w.bs1().bit(true));
    }

    pub fn read_reg(&self, reg : Register) -> u8 {
        let reg = (0x80 | reg as u8) & 0b1011_1111;

        let pc = unsafe{&*stm32f429::GPIOC.get()};
        let spi = unsafe{&*stm32f429::SPI5.get()};

        pc.bsrr.write(|w| w.br1().bit(true));

        while spi.sr.read().txe().bit() == false { }
        spi.dr.write(|w| unsafe{w.bits(reg as u32)});
        while spi.sr.read().rxne().bit() == false { }
        let _ = spi.dr.read().bits() as u8;

        while spi.sr.read().txe().bit() == false {}
        spi.dr.write(|w| unsafe{w.bits(0x00)});
        while spi.sr.read().rxne().bit() == false { }

        let data = spi.dr.read().bits() as u8;
        pc.bsrr.write(|w| w.bs1().bit(true));
        data
    }

    pub fn check_connection(&self) -> Result<(),L3GD20Error> {
        match self.read_reg(Register::WHO_AM_I) {
            0b1101_0100 => Ok(()),
            _ => Err(L3GD20Error::IdCannotBeRead),
        }
    }

    pub fn sensor_interrupt() {

    }
}
