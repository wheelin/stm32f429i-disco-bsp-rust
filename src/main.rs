#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

#[macro_use(interrupt)]
extern crate stm32f429;

extern crate volatile;

use core::fmt::Write;

use cortex_m::asm;
use cortex_m_semihosting::hio;


mod bsp;
mod misc;
mod system;

use system::clks;
use bsp::l3gd20::*;
use bsp::led::*;
use bsp::sdram;
use misc::*;

pub fn array_cmp(a : &[u32], b : &[u32]) -> bool {
    for (i, elem) in a.iter().enumerate() {
        if *elem != b[i] {
            return false;
        }
    }
    true
}

fn main() {
    clks::init();
    sdram::init();

    let mut stdout = hio::hstdout().unwrap();

    //let mut stdout = hio::hstdout().unwrap();
    let addr = 0xFFFFFFF;
    let a = [32, 345, 134512, 234, 2365, 652234];
    let mut b : [u32; 6] = [0; 6];

    loop {
        match sdram::write_buffer(&a, addr) {
            Ok(_) => (),
            Err(e) => writeln!(stdout, "{}", e).unwrap(),
        };
        match sdram::read_buffer(&mut b, addr) {
            Ok(_) => (),
            Err(e) => writeln!(stdout, "{}", e).unwrap(),
        };
        if array_cmp(&a, &b) {
            writeln!(stdout, "Written array is equal to read array.");
        } else {
            writeln!(stdout, "Written array is different of read array.");
        }
        for i in b.iter_mut() {
            *i = 0;
        }
        delay(0xFFFF);
    }
}

/*fn main() {
    let fmc = unsafe{&*stm32f429::FMC.get()};
    let mut stdout = hio::hstdout().unwrap();
    let fmc_sdcr_addr = &(fmc.sdcr2) as *const _;
    writeln!(stdout, "Address of fmc sdcr1 = {:p}", fmc_sdcr_addr);
}*/
