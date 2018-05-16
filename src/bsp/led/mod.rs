use stm32f429::{RCC, GPIOG};

pub enum LedName {
    Led3,
    Led4,
}

pub struct Led {
    n : LedName,
}

impl Led {
    pub fn new(n : LedName) -> Led {
        Led {
            n,
        }
    }

    pub fn init(&self) {
        let rcc = unsafe {&*RCC::ptr()};
        let pg  = unsafe {&*GPIOG::ptr()};

        rcc.ahb1enr.modify(|_, w| w.gpiogen().bit(true));

        match self.n {
            LedName::Led3 => {
                pg.moder.modify(|_, w| unsafe {
                    w.moder13().bits(0b01)
                });
            },
            LedName::Led4 => {
                pg.moder.modify(|_, w| unsafe {
                    w.moder14().bits(0b01)
                });
            },
        };
    }

    pub fn on(&self) {
        let pg = unsafe {&*GPIOG::ptr()};

        match self.n {
            LedName::Led3 => pg.bsrr.write(|w| w.bs13().bit(true)),
            LedName::Led4 => pg.bsrr.write(|w| w.bs14().bit(true)),
        };
    }

    pub fn off(&self) {
        let pg = unsafe {&*GPIOG::ptr()};

        match self.n {
            LedName::Led3 => pg.bsrr.write(|w| w.br13().bit(true)),
            LedName::Led4 => pg.bsrr.write(|w| w.br14().bit(true)),
        };
    }

    pub fn toggle(&self) {
        let pg = unsafe {&*GPIOG::ptr()};

        match self.n {
            LedName::Led3 => {
                if pg.idr.read().idr13().bit() {
                    pg.bsrr.write(|w| w.br13().bit(true))
                } else {
                    pg.bsrr.write(|w| w.bs13().bit(true))
                }
            },
            LedName::Led4 => {
                if pg.idr.read().idr14().bit() {
                    pg.bsrr.write(|w| w.br14().bit(true))
                } else {
                    pg.bsrr.write(|w| w.bs14().bit(true))
                }
            },
        };
    }
}
