use stm32f429::{
    GPIOA, GPIOB, GPIOC, GPIOD, GPIOE, GPIOF, GPIOG, GPIOH, GPIOI, GPIOJ, GPIOK,
};

use stm32f429::{gpioa, gpiob, gpiok};
use core::ops::Deref;

pub enum Mode {
    Input   = 0b00,
    Output  = 0b01,
    AltFn   = 0b10,
    Analog  = 0b11,
}

pub enum OutType {
    PushPull = 0,
    OpenDrain = 1,
}

pub enum OutSpeed {
    Low = 0b00,
    Medium = 0b01,
    High = 0b10,
    VeryHigh = 0b11,
}

pub enum PullType {
    NoPull = 0b00,
    PullUp = 0b01,
    PullDown = 0b10,
}

pub enum AltFn {
    Sys                      = 0,
    Tim_1_2                  = 1,
    Tim_3_4_5                = 2,
    Tim_8_9_10               = 3,
    I2c_1_2_3                = 4,
    Spi_1_2_3_4_5_6          = 5,
    Spi_2_3_Sai_1            = 6,
    Spi_3_Usart_1_2_3        = 7,
    Usart_6_Uart_4_5_7_8     = 8,
    Can_1_2_Tim_12_13_14_Lcd = 9,
    Otg_2_Hs_Otg_1_Fs        = 10,
    Eth                      = 11,
    Fmc_Sdio_Otg_2_Fs        = 12,
    Dcmi                     = 13,
    Lcd                      = 14,
    SysEvent                 = 15,
}

pub trait IsPortA : Deref<Target = gpioa::RegisterBlock> {

}

pub trait IsPortB : Deref<Target = gpiob::RegisterBlock> {

}

pub trait IsPortK : Deref<Target = gpiok::RegisterBlock> {

}

impl IsPortA for GPIOA {}
impl IsPortB for GPIOB {}
impl IsPortK for GPIOC {}
impl IsPortK for GPIOD {}
impl IsPortK for GPIOE {}
impl IsPortK for GPIOF {}
impl IsPortK for GPIOG {}
impl IsPortK for GPIOH {}
impl IsPortK for GPIOI {}
impl IsPortK for GPIOJ {}
impl IsPortK for GPIOK {}

/////////////////////////////////////////////////////////////////////////////////
// GPIOA Implementation
/////////////////////////////////////////////////////////////////////////////////
pub mod gpio_a {
    use spl_rs::gpio::*;
    pub fn configure<T : IsPortA>(gpio_port : &T,
        pin : u8,
        m : Mode,
        ot : OutType,
        os : OutSpeed,
        pt : PullType
    ) -> Result<(), ()> {
        if pin > 15 {return Err(());}
        let port  = gpio_port;

        // configure mode
        port.moder.modify(|r, w| unsafe {
            // clear targeted bits
            let o = r.bits() & !(0b11 << (pin * 2));
            // print pattern on new bitfield
            let n = o | ((m as u32) << (pin * 2));
            w.bits(n)
        });

        // configure output type. if input or analog, don't care
        port.otyper.modify(|r, w| unsafe {
            let o = r.bits() & !(1 << pin);
            let n = o | ((ot as u32) << pin);
            w.bits(n)
        });

        // configure output speed
        port.ospeedr.modify(|r, w| unsafe {
            // clear targeted bits
            let o = r.bits() & !(0b11 << (pin * 2));
            // print pattern on new bitfield
            let n = o | ((os as u32) << (pin * 2));
            w.bits(n)
        });

        // configure pull type
        port.pupdr.modify(|r, w| unsafe {
            // clear targeted bits
            let o = r.bits() & !(0b11 << (pin * 2));
            // print pattern on new bitfield
            let n = o | ((pt as u32) << (pin * 2));
            w.bits(n)
        });

        Ok(())
    }

    pub fn set_alt_fn<T : IsPortA>(gpio_port : &T, pin : u8, af : AltFn) -> Result<(), ()> {
        if pin > 15 {return Err(());}
        let port = gpio_port;
        if pin > 7 {
            port.afrh.modify(|r, w| unsafe {
                // clear targeted bits
                let o = r.bits() & !(0b1111 << ((pin - 8) * 4));
                // print pattern on new bitfield
                let n = o | ((af as u32) << ((pin - 8) * 4));
                w.bits(n)
            });
        } else {
            port.afrl.modify(|r, w| unsafe {
                // clear targeted bits
                let o = r.bits() & !(0b1111 << (pin * 4));
                // print pattern on new bitfield
                let n = o | ((af as u32) << (pin * 4));
                w.bits(n)
            });
        }
        Ok(())
    }

    pub fn read<T : IsPortA>(gpio_port : &T, pin : u8) -> Result<bool, ()> {
        if pin > 15 {return Err(());}
        let port = gpio_port;
        Ok((port.idr.read().bits() | (1 << pin)) != 0)
    }

    pub fn write<T : IsPortA>(gpio_port : &T, pin : u8, state : bool) -> Result<(), ()> {
        if pin > 15 {return Err(());}
        let port = gpio_port;
        port.bsrr.write(|w| unsafe {
            if state {
                w.bits(1 << pin)
            } else {
                w.bits(1 << (pin + 16))
            }
        });
        Ok(())
    }
}


/////////////////////////////////////////////////////////////////////////////////
// GPIOB Implementation
/////////////////////////////////////////////////////////////////////////////////
pub mod gpio_b {
    use spl_rs::gpio::*;
    pub fn configure<T : IsPortB>(gpio_port : &T,
        pin : u8,
        m : Mode,
        ot : OutType,
        os : OutSpeed,
        pt : PullType
    ) -> Result<(), ()> {
        if pin > 15 {return Err(());}
        let port  = gpio_port;

        // configure mode
        port.moder.modify(|r, w| unsafe {
            // clear targeted bits
            let o = r.bits() & !(0b11 << (pin * 2));
            // print pattern on new bitfield
            let n = o | ((m as u32) << (pin * 2));
            w.bits(n)
        });

        // configure output type. if input or analog, don't care
        port.otyper.modify(|r, w| unsafe {
            let o = r.bits() & !(1 << pin);
            let n = o | ((ot as u32) << pin);
            w.bits(n)
        });

        // configure output speed
        port.ospeedr.modify(|r, w| unsafe {
            // clear targeted bits
            let o = r.bits() & !(0b11 << (pin * 2));
            // print pattern on new bitfield
            let n = o | ((os as u32) << (pin * 2));
            w.bits(n)
        });

        // configure pull type
        port.pupdr.modify(|r, w| unsafe {
            // clear targeted bits
            let o = r.bits() & !(0b11 << (pin * 2));
            // print pattern on new bitfield
            let n = o | ((pt as u32) << (pin * 2));
            w.bits(n)
        });

        Ok(())
    }

    pub fn set_alt_fn<T : IsPortB>(gpio_port : &T, pin : u8, af : AltFn) -> Result<(), ()> {
        if pin > 15 {return Err(());}
        let port = gpio_port;
        if pin > 7 {
            port.afrh.modify(|r, w| unsafe {
                // clear targeted bits
                let o = r.bits() & !(0b1111 << ((pin - 8) * 4));
                // print pattern on new bitfield
                let n = o | ((af as u32) << ((pin - 8) * 4));
                w.bits(n)
            });
        } else {
            port.afrl.modify(|r, w| unsafe {
                // clear targeted bits
                let o = r.bits() & !(0b1111 << (pin * 4));
                // print pattern on new bitfield
                let n = o | ((af as u32) << (pin * 4));
                w.bits(n)
            });
        }
        Ok(())
    }

    pub fn read<T : IsPortB>(gpio_port : &T, pin : u8) -> Result<bool, ()> {
        if pin > 15 {return Err(());}
        let port = gpio_port;
        Ok((port.idr.read().bits() | (1 << pin)) != 0)
    }

    pub fn write<T : IsPortB>(gpio_port : &T, pin : u8, state : bool) -> Result<(), ()> {
        if pin > 15 {return Err(());}
        let port = gpio_port;
        port.bsrr.write(|w| unsafe {
            if state {
                w.bits(1 << pin)
            } else {
                w.bits(1 << (pin + 16))
            }
        });
        Ok(())
    }
}

/////////////////////////////////////////////////////////////////////////////////
// GPIOK Implementation
/////////////////////////////////////////////////////////////////////////////////
pub mod port_others {
    use spl_rs::gpio::*;
    pub fn configure<T : IsPortK>(gpio_port : &T,
        pin : u8,
        m : Mode,
        ot : OutType,
        os : OutSpeed,
        pt : PullType
    ) -> Result<(), ()> {
        if pin > 15 {return Err(());}
        let port  = gpio_port;

        // configure mode
        port.moder.modify(|r, w| unsafe {
            // clear targeted bits
            let o = r.bits() & !(0b11 << (pin * 2));
            // print pattern on new bitfield
            let n = o | ((m as u32) << (pin * 2));
            w.bits(n)
        });

        // configure output type. if input or analog, don't care
        port.otyper.modify(|r, w| unsafe {
            let o = r.bits() & !(1 << pin);
            let n = o | ((ot as u32) << pin);
            w.bits(n)
        });

        // configure output speed
        port.ospeedr.modify(|r, w| unsafe {
            // clear targeted bits
            let o = r.bits() & !(0b11 << (pin * 2));
            // print pattern on new bitfield
            let n = o | ((os as u32) << (pin * 2));
            w.bits(n)
        });

        // configure pull type
        port.pupdr.modify(|r, w| unsafe {
            // clear targeted bits
            let o = r.bits() & !(0b11 << (pin * 2));
            // print pattern on new bitfield
            let n = o | ((pt as u32) << (pin * 2));
            w.bits(n)
        });

        Ok(())
    }

    pub fn set_alt_fn<T : IsPortK>(gpio_port : &T, pin : u8, af : AltFn) -> Result<(), ()> {
        if pin > 15 {return Err(());}
        let port = gpio_port;
        if pin > 7 {
            port.afrh.modify(|r, w| unsafe {
                // clear targeted bits
                let o = r.bits() & !(0b1111 << ((pin - 8) * 4));
                // print pattern on new bitfield
                let n = o | ((af as u32) << ((pin - 8) * 4));
                w.bits(n)
            });
        } else {
            port.afrl.modify(|r, w| unsafe {
                // clear targeted bits
                let o = r.bits() & !(0b1111 << (pin * 4));
                // print pattern on new bitfield
                let n = o | ((af as u32) << (pin * 4));
                w.bits(n)
            });
        }
        Ok(())
    }

    pub fn read<T : IsPortK>(gpio_port : &T, pin : u8) -> Result<bool, ()> {
        if pin > 15 {return Err(());}
        let port = gpio_port;
        Ok((port.idr.read().bits() | (1 << pin)) != 0)
    }

    pub fn write<T : IsPortK>(gpio_port : &T, pin : u8, state : bool) -> Result<(), ()> {
        if pin > 15 {return Err(());}
        let port = gpio_port;
        port.bsrr.write(|w| unsafe {
            if state {
                w.bits(1 << pin)
            } else {
                w.bits(1 << (pin + 16))
            }
        });
        Ok(())
    }
}
