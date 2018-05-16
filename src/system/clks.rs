use stm32f429::{RCC, PWR, FLASH};

use spl_rs::rcc;

/*
This configuration follows the one present in system_stm32f4xx.c for
the stm32f429i-disco board template project in sw4stm32 using SPL.
*/

pub fn init() -> Result<(), ()>{
    let pwr = unsafe{ &*PWR::ptr() };
    let flash = unsafe{ &*FLASH::ptr() };

    rcc::clock_ctrl(rcc::Clock::HSE_ON, true);
    while rcc::check_flag(rcc::ClkFlag::HSE_RDY) != true {}

    match rcc::configure_pll(0b0111, 0b101101000, 0b11, 0b001111) {
        Ok(()) => (),
        Err(()) => return Err(()),
    };

    rcc::set_apb1_periph_clk(rcc::Apb1Enable::PWR, true);

    pwr.cr.modify(|_, w| unsafe{w.vos().bits(0b11)});

    rcc::set_ahb_pre(rcc::AhbPre::Div1);
    rcc::set_apb2_pre(rcc::ApbPre::Div2);
    rcc::set_apb1_pre(rcc::ApbPre::Div4);

    rcc::clock_ctrl(rcc::Clock::PLL_ON, true);
    while rcc::check_flag(rcc::ClkFlag::PLL_RDY) != true {}

    pwr.cr.modify(|_, w| w.oden().bit(true));
    while pwr.csr.read().odrdy().bit() == false {}

    pwr.cr.modify(|_, w| w.odswen().bit(true));
    while pwr.csr.read().odswrdy().bit() == false {}

    flash.acr.modify(|_, w| unsafe {
        w.prften().bit(true)
         .icen().bit(true)
         .latency().bits(5)
    });

    rcc::set_sysclk_src(rcc::SysClkSrc::Pll);
    while rcc::get_sysclk_src() != rcc::SysClkSrc::Pll {}

    return Ok(())
}
