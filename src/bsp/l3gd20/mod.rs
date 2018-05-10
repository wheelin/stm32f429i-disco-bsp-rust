use stm32f429;
use stm32f429::interrupt;
use cortex_m::peripheral::NVIC;

use spl_rs::{gpio, rcc};

pub enum L3GD20Error {
    SpiTimeout,
    SpiBusError,
    IdCannotBeRead,
}


pub enum Register {
    WhoAmI = 0x0F,
    CtrlReg1 = 0x20,
    CtrlReg2 = 0x21,
    CtrlReg3 = 0x22,
    CtrlReg4 = 0x23,
    CtrlReg5 = 0x24,
    Reference = 0x25,
    OutTemp,
    StatisReg,
    OutXL,
    OutXH,
    OutYL,
    OutYH,
    OutZL,
    OutZH,
    FifoCtrlReg,
    FifoSrcReg,
    Int1Reg,
    Int1Src,
    Int1ThsXH,
    Int1ThsXL,
    Int1ThsYH,
    Int1ThsZL,
    Int1ThsZH,
    Int1Duration,
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
        rcc::set_ahb1_periph_clk(
            rcc::Ahb1Enable::GPIOF |
            rcc::Ahb1Enable::GPIOA |
            rcc::Ahb1Enable::GPIOC,
            true
        );
        rcc::set_apb2_periph_clk(rcc::Apb2Enable::SPI5, true);

        // configure cs pin, output pull-up, default high state
        let pc = unsafe{&*stm32f429::GPIOC.get()};
        gpio::port_others::configure(
            pc,
            1,
            gpio::Mode::Output,
            gpio::OutType::PushPull,
            gpio::OutSpeed::Low,
            gpio::PullType::NoPull
        ).unwrap();

        // configure spi pins
        // set as alternative function pin
        let pf = unsafe{ &*stm32f429::GPIOF.get() };
        let pfpins = [7, 8, 9];
        for i in pfpins.iter() {
            gpio::port_others::configure(
                pf,
                *i,
                gpio::Mode::AltFn,
                gpio::OutType::PushPull,
                gpio::OutSpeed::High,
                gpio::PullType::NoPull
            ).unwrap();
            gpio::port_others::set_alt_fn(
                pf,
                *i,
                gpio::AltFn::Spi123456
            ).unwrap();
        }

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
        gpio::port_others::write(pc, 1, true).unwrap();

        let spi = unsafe{&*stm32f429::SPI5.get()};

        spi.dr.write(|w| unsafe{w.bits(reg as u32)});
        while spi.sr.read().txe().bit() == false {}

        spi.dr.write(|w| unsafe{w.bits(dat as u32)});
        while spi.sr.read().txe().bit() == false {}

        gpio::port_others::write(pc, 1, false).unwrap();
    }

    pub fn read_reg(&self, reg : Register) -> u8 {
        let reg = (0x80 | reg as u8) & 0b1011_1111;

        let pc = unsafe{&*stm32f429::GPIOC.get()};
        let spi = unsafe{&*stm32f429::SPI5.get()};

        gpio::port_others::write(pc, 1, true).unwrap();

        while spi.sr.read().txe().bit() == false { }
        spi.dr.write(|w| unsafe{w.bits(reg as u32)});
        while spi.sr.read().rxne().bit() == false { }
        let _ = spi.dr.read().bits() as u8;

        while spi.sr.read().txe().bit() == false {}
        spi.dr.write(|w| unsafe{w.bits(0x00)});
        while spi.sr.read().rxne().bit() == false { }

        let data = spi.dr.read().bits() as u8;
        gpio::port_others::write(pc, 1, false).unwrap();
        data
    }

    pub fn check_connection(&self) -> Result<(),L3GD20Error> {
        match self.read_reg(Register::WhoAmI) {
            0b1101_0100 => Ok(()),
            _ => Err(L3GD20Error::IdCannotBeRead),
        }
    }

    pub fn sensor_interrupt() {

    }
}
