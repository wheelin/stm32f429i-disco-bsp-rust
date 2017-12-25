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

extern crate volatile;

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

use stm32f429::GPIOG;

fn main() {
    clks::init();

    let pg = unsafe {&*GPIOG.get()};
    gpio::port_others::configure(pg, 13, gpio::Mode::Output, gpio::OutType::PushPull, gpio::OutSpeed::Low, gpio::PullType::NoPull);

    loop {
        gpio::port_others::write(pg, 13, true);
        delay(0xFFFF);
        gpio::port_others::write(pg, 13, false);
        delay(0xFFFF);
    }
}

/*fn main() {
    let fmc = unsafe{&*stm32f429::FMC.get()};
    let mut stdout = hio::hstdout().unwrap();
    let fmc_sdcr_addr = &(fmc.sdcr2) as *const _;
    writeln!(stdout, "Address of fmc sdcr1 = {:p}", fmc_sdcr_addr);
}*/
