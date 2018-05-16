#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

#[macro_use(interrupt)]
extern crate stm32f429;

#[macro_use]
extern crate bitflags;
extern crate bare_metal;
extern crate volatile;

extern crate panic_abort;

use core::fmt::Write;

use cortex_m::asm;
use cortex_m_semihosting::hio;


mod bsp;
mod misc;
mod system;
mod spl_rs;

use system::clks;
use bsp::l3gd20::*;
use bsp::led::*;
use bsp::sdram;
use misc::*;
use bsp::lcd::{lcd, fonts};
use spl_rs::gpio;
use spl_rs::rcc;

use stm32f429::GPIOG;

fn main() {
    match clks::init() {
        Ok(()) => (),
        Err(()) => asm::bkpt(),
    };

    rcc::set_ahb1_periph_clk(rcc::Ahb1Enable::GPIOG, true);

    let pg = unsafe {&*GPIOG::ptr()};
    gpio::port_others::configure(
        pg,
        13,
        gpio::Mode::Output,
        gpio::OutType::PushPull,
        gpio::OutSpeed::Low,
        gpio::PullType::NoPull
    ).unwrap();

    L3GD20::get_instance().init().unwrap();

    loop {
        if L3GD20::get_instance().check_connection().is_ok() {
            gpio::port_others::write(pg, 13, true).unwrap();
            delay(0xFFFF);
        }
        gpio::port_others::write(pg, 13, false).unwrap();
        delay(0xFFFF);
    }
}

/*fn main() {
    let fmc = unsafe{&*stm32f429::FMC.get()};
    let mut stdout = hio::hstdout().unwrap();
    let fmc_sdcr_addr = &(fmc.sdcr2) as *const _;
    writeln!(stdout, "Address of fmc sdcr1 = {:p}", fmc_sdcr_addr);
}*/
