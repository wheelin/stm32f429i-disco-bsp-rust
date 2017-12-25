use stm32f429::{RCC, PWR, FLASH};

/*
This configuration follows the one present in system_stm32f4xx.c for
the stm32f429i-disco board template project in sw4stm32 using SPL.
*/

pub fn init() {
    let rcc = unsafe{ &*RCC.get() };
    let pwr = unsafe{ &*PWR.get() };
    let flash = unsafe{ &*FLASH.get() };

    rcc.cr.modify(|_, w| w.hseon().bit(true));
    while rcc.cr.read().hserdy().bit() == false {}

    rcc.pllcfgr.modify(|_, w| unsafe {
        w.pllq().bits(0b0111)
         .pllm().bits(0b001111)
         .plln().bits(0b101101000)
         .pllp().bits(0b11)
         .pllsrc().bit(true)
    });

    rcc.apb1enr.modify(|_, w| w.pwren().bit(true));

    pwr.cr.modify(|_, w| unsafe{w.vos().bits(0b11)});

    rcc.cfgr.modify(|_, w| unsafe{w.hpre().bits(0b0000)});
    rcc.cfgr.modify(|_, w| unsafe{w.ppre2().bits(0b100)});
    rcc.cfgr.modify(|_, w| unsafe{w.ppre1().bits(0b101)});

    rcc.cr.modify(|_, w| w.pllon().bit(true));
    while rcc.cr.read().pllrdy().bit() == false {}

    pwr.cr.modify(|_, w| w.oden().bit(true));
    while pwr.csr.read().odrdy().bit() == false {}

    pwr.cr.modify(|_, w| w.odswen().bit(true));
    while pwr.csr.read().odswrdy().bit() == false {}

    flash.acr.modify(|_, w| unsafe {
        w.prften().bit(true)
         .icen().bit(true)
         .latency().bits(5)
    });

    rcc.cfgr.modify(|_, w| unsafe {
        w.sw().bits(0b10)
    });
    while rcc.cfgr.read().sws().bits() != 0b10 {}
}
