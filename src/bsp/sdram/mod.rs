use stm32f429::*;
use misc;
use cortex_m;

use core::fmt;

pub const SDRAM_SIZE                            : u32 = 0x800000; // bytes, 0x200000 words
pub const SDRAM_BANK_ADDR                       : u32 = 0xD0000000;

const GPIO_ALT_FN                               : u8  = 0b10;
const ALT_FN_FMC                                : u8  = 0b1100;
const GPIO_SPEED_FMC                            : u8  = 0b10;
const GPIO_NPUPD_FMC                            : u8  = 0b00;

const SDRAM_STORAGE_ELEMENTS_SIZE               : u32 = 4; // bytes

pub enum SdRamError {
    OutOfBoundsAccess(u32), // contains size of overhead access
    RefreshError,
    UnalignedAccess(u32),   // contains next aligned address
}

impl fmt::Display for SdRamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SdRamError::OutOfBoundsAccess(x) => write!(f, "OutOfBoundsAccess ({})", x),
            SdRamError::RefreshError => write!(f, "RefreshError"),
            SdRamError::UnalignedAccess(x) => write!(f, "UnalignedAccess ({:X})", x),
        }
    }
}

pub fn init_gpios() {
    let rcc = unsafe{&*RCC.get()};
    let pb = unsafe{&*GPIOB.get()};
    let pc = unsafe{&*GPIOC.get()};
    let pd = unsafe{&*GPIOD.get()};
    let pe = unsafe{&*GPIOE.get()};
    let pf = unsafe{&*GPIOF.get()};
    let pg = unsafe{&*GPIOG.get()};

    rcc.ahb1enr.modify(|_, w|{
        w.gpioben().bit(true)
         .gpiocen().bit(true)
         .gpioden().bit(true)
         .gpioeen().bit(true)
         .gpiofen().bit(true)
         .gpiogen().bit(true)
    });

    // port B config, pins 5 and 6
    pb.moder.modify(|_, w| unsafe{
        w.moder5().bits(GPIO_ALT_FN)
         .moder6().bits(GPIO_ALT_FN)
    });
    pb.otyper.modify(|_, w|
        w.ot5().bit(false)
         .ot6().bit(false)
    );
    pb.ospeedr.modify(|_, w| unsafe{
        w.ospeedr5().bits(GPIO_SPEED_FMC)
         .ospeedr6().bits(GPIO_SPEED_FMC)
    });
    pb.pupdr.modify(|_, w| unsafe {
        w.pupdr5().bits(GPIO_NPUPD_FMC)
         .pupdr6().bits(GPIO_NPUPD_FMC)
    });
    pb.afrl.modify(|_, w| unsafe {
        w.afrl5().bits(ALT_FN_FMC)
         .afrl6().bits(ALT_FN_FMC)
    });

    // port C config
    pc.moder.modify(|_, w| unsafe{
        w.moder0().bits(GPIO_ALT_FN)
    });
    pc.otyper.modify(|_, w|
        w.ot0().bit(false)
    );
    pc.ospeedr.modify(|_, w| unsafe{
        w.ospeedr0().bits(GPIO_SPEED_FMC)
    });
    pc.pupdr.modify(|_, w| unsafe {
        w.pupdr0().bits(GPIO_NPUPD_FMC)
    });
    pc.afrl.modify(|_, w| unsafe {
        w.afrl0().bits(ALT_FN_FMC)
    });

    // port D config
    pd.moder.modify(|_, w| unsafe{
        w.moder0().bits(GPIO_ALT_FN)
         .moder1().bits(GPIO_ALT_FN)
         .moder8().bits(GPIO_ALT_FN)
         .moder9().bits(GPIO_ALT_FN)
         .moder10().bits(GPIO_ALT_FN)
         .moder14().bits(GPIO_ALT_FN)
         .moder15().bits(GPIO_ALT_FN)
    });
    pd.otyper.modify(|_, w|
        w.ot0().bit(false)
         .ot1().bit(false)
         .ot8().bit(false)
         .ot9().bit(false)
         .ot10().bit(false)
         .ot14().bit(false)
         .ot15().bit(false)
    );
    pd.ospeedr.modify(|_, w| unsafe{
        w.ospeedr0().bits(GPIO_SPEED_FMC)
         .ospeedr1().bits(GPIO_SPEED_FMC)
         .ospeedr8().bits(GPIO_SPEED_FMC)
         .ospeedr9().bits(GPIO_SPEED_FMC)
         .ospeedr10().bits(GPIO_SPEED_FMC)
         .ospeedr14().bits(GPIO_SPEED_FMC)
         .ospeedr15().bits(GPIO_SPEED_FMC)
    });
    pd.pupdr.modify(|_, w| unsafe {
        w.pupdr0().bits(GPIO_NPUPD_FMC)
         .pupdr1().bits(GPIO_NPUPD_FMC)
         .pupdr8().bits(GPIO_NPUPD_FMC)
         .pupdr9().bits(GPIO_NPUPD_FMC)
         .pupdr10().bits(GPIO_NPUPD_FMC)
         .pupdr14().bits(GPIO_NPUPD_FMC)
         .pupdr15().bits(GPIO_NPUPD_FMC)
    });
    pd.afrl.modify(|_, w| unsafe {
        w.afrl0().bits(ALT_FN_FMC)
         .afrl1().bits(ALT_FN_FMC)
    });
    pd.afrh.modify(|_, w| unsafe {
        w.afrh8().bits(ALT_FN_FMC)
         .afrh9().bits(ALT_FN_FMC)
         .afrh10().bits(ALT_FN_FMC)
         .afrh14().bits(ALT_FN_FMC)
         .afrh15().bits(ALT_FN_FMC)
    });

    // port E config
    pe.moder.modify(|_, w| unsafe{
        w.moder0().bits(GPIO_ALT_FN)
         .moder1().bits(GPIO_ALT_FN)
         .moder7().bits(GPIO_ALT_FN)
         .moder8().bits(GPIO_ALT_FN)
         .moder9().bits(GPIO_ALT_FN)
         .moder10().bits(GPIO_ALT_FN)
         .moder11().bits(GPIO_ALT_FN)
         .moder12().bits(GPIO_ALT_FN)
         .moder13().bits(GPIO_ALT_FN)
         .moder14().bits(GPIO_ALT_FN)
         .moder15().bits(GPIO_ALT_FN)
    });
    pe.otyper.modify(|_, w|
        w.ot0().bit(false)
         .ot1().bit(false)
         .ot7().bit(false)
         .ot8().bit(false)
         .ot9().bit(false)
         .ot10().bit(false)
         .ot11().bit(false)
         .ot12().bit(false)
         .ot13().bit(false)
         .ot14().bit(false)
         .ot15().bit(false)
    );
    pe.ospeedr.modify(|_, w| unsafe{
        w.ospeedr0().bits(GPIO_SPEED_FMC)
         .ospeedr1().bits(GPIO_SPEED_FMC)
         .ospeedr7().bits(GPIO_SPEED_FMC)
         .ospeedr8().bits(GPIO_SPEED_FMC)
         .ospeedr9().bits(GPIO_SPEED_FMC)
         .ospeedr10().bits(GPIO_SPEED_FMC)
         .ospeedr11().bits(GPIO_SPEED_FMC)
         .ospeedr12().bits(GPIO_SPEED_FMC)
         .ospeedr13().bits(GPIO_SPEED_FMC)
         .ospeedr14().bits(GPIO_SPEED_FMC)
         .ospeedr15().bits(GPIO_SPEED_FMC)
    });
    pe.pupdr.modify(|_, w| unsafe {
        w.pupdr0().bits(GPIO_NPUPD_FMC)
         .pupdr1().bits(GPIO_NPUPD_FMC)
         .pupdr7().bits(GPIO_NPUPD_FMC)
         .pupdr8().bits(GPIO_NPUPD_FMC)
         .pupdr9().bits(GPIO_NPUPD_FMC)
         .pupdr10().bits(GPIO_NPUPD_FMC)
         .pupdr11().bits(GPIO_NPUPD_FMC)
         .pupdr12().bits(GPIO_NPUPD_FMC)
         .pupdr13().bits(GPIO_NPUPD_FMC)
         .pupdr14().bits(GPIO_NPUPD_FMC)
         .pupdr15().bits(GPIO_NPUPD_FMC)
    });
    pe.afrl.modify(|_, w| unsafe {
        w.afrl0().bits(ALT_FN_FMC)
         .afrl1().bits(ALT_FN_FMC)
         .afrl7().bits(ALT_FN_FMC)
    });
    pe.afrh.modify(|_, w| unsafe {
        w.afrh8().bits(ALT_FN_FMC)
         .afrh9().bits(ALT_FN_FMC)
         .afrh10().bits(ALT_FN_FMC)
         .afrh11().bits(ALT_FN_FMC)
         .afrh12().bits(ALT_FN_FMC)
         .afrh13().bits(ALT_FN_FMC)
         .afrh14().bits(ALT_FN_FMC)
         .afrh15().bits(ALT_FN_FMC)
    });

    // port F config
    pf.moder.modify(|_, w| unsafe{
        w.moder0().bits(GPIO_ALT_FN)
         .moder1().bits(GPIO_ALT_FN)
         .moder2().bits(GPIO_ALT_FN)
         .moder3().bits(GPIO_ALT_FN)
         .moder4().bits(GPIO_ALT_FN)
         .moder5().bits(GPIO_ALT_FN)
         .moder11().bits(GPIO_ALT_FN)
         .moder12().bits(GPIO_ALT_FN)
         .moder13().bits(GPIO_ALT_FN)
         .moder14().bits(GPIO_ALT_FN)
         .moder15().bits(GPIO_ALT_FN)
    });
    pf.otyper.modify(|_, w|
        w.ot0().bit(false)
         .ot1().bit(false)
         .ot2().bit(false)
         .ot3().bit(false)
         .ot4().bit(false)
         .ot5().bit(false)
         .ot11().bit(false)
         .ot12().bit(false)
         .ot13().bit(false)
         .ot14().bit(false)
         .ot15().bit(false)
    );
    pf.ospeedr.modify(|_, w| unsafe{
        w.ospeedr0().bits(GPIO_SPEED_FMC)
         .ospeedr1().bits(GPIO_SPEED_FMC)
         .ospeedr2().bits(GPIO_SPEED_FMC)
         .ospeedr3().bits(GPIO_SPEED_FMC)
         .ospeedr4().bits(GPIO_SPEED_FMC)
         .ospeedr5().bits(GPIO_SPEED_FMC)
         .ospeedr11().bits(GPIO_SPEED_FMC)
         .ospeedr12().bits(GPIO_SPEED_FMC)
         .ospeedr13().bits(GPIO_SPEED_FMC)
         .ospeedr14().bits(GPIO_SPEED_FMC)
         .ospeedr15().bits(GPIO_SPEED_FMC)
    });
    pf.pupdr.modify(|_, w| unsafe {
        w.pupdr0().bits(GPIO_NPUPD_FMC)
         .pupdr1().bits(GPIO_NPUPD_FMC)
         .pupdr2().bits(GPIO_NPUPD_FMC)
         .pupdr3().bits(GPIO_NPUPD_FMC)
         .pupdr4().bits(GPIO_NPUPD_FMC)
         .pupdr5().bits(GPIO_NPUPD_FMC)
         .pupdr11().bits(GPIO_NPUPD_FMC)
         .pupdr12().bits(GPIO_NPUPD_FMC)
         .pupdr13().bits(GPIO_NPUPD_FMC)
         .pupdr14().bits(GPIO_NPUPD_FMC)
         .pupdr15().bits(GPIO_NPUPD_FMC)
    });
    pf.afrl.modify(|_, w| unsafe {
        w.afrl0().bits(ALT_FN_FMC)
         .afrl1().bits(ALT_FN_FMC)
         .afrl2().bits(ALT_FN_FMC)
         .afrl3().bits(ALT_FN_FMC)
         .afrl4().bits(ALT_FN_FMC)
         .afrl5().bits(ALT_FN_FMC)
    });
    pf.afrh.modify(|_, w| unsafe {
        w.afrh11().bits(ALT_FN_FMC)
         .afrh12().bits(ALT_FN_FMC)
         .afrh13().bits(ALT_FN_FMC)
         .afrh14().bits(ALT_FN_FMC)
         .afrh15().bits(ALT_FN_FMC)
    });

    // port G config
    pg.moder.modify(|_, w| unsafe{
        w.moder0().bits(GPIO_ALT_FN)
         .moder1().bits(GPIO_ALT_FN)
         .moder4().bits(GPIO_ALT_FN)
         .moder5().bits(GPIO_ALT_FN)
         .moder8().bits(GPIO_ALT_FN)
         .moder15().bits(GPIO_ALT_FN)
    });
    pg.otyper.modify(|_, w| 
        w.ot0().bit(false)
         .ot1().bit(false)
         .ot4().bit(false)
         .ot5().bit(false)
         .ot8().bit(false)
         .ot15().bit(false)
    );
    pg.ospeedr.modify(|_, w| unsafe{
        w.ospeedr0().bits(GPIO_SPEED_FMC)
         .ospeedr1().bits(GPIO_SPEED_FMC)
         .ospeedr4().bits(GPIO_SPEED_FMC)
         .ospeedr5().bits(GPIO_SPEED_FMC)
         .ospeedr8().bits(GPIO_SPEED_FMC)
         .ospeedr15().bits(GPIO_SPEED_FMC)
    });
    pg.pupdr.modify(|_, w| unsafe {
        w.pupdr0().bits(GPIO_NPUPD_FMC)
         .pupdr1().bits(GPIO_NPUPD_FMC)
         .pupdr4().bits(GPIO_NPUPD_FMC)
         .pupdr5().bits(GPIO_NPUPD_FMC)
         .pupdr8().bits(GPIO_NPUPD_FMC)
         .pupdr15().bits(GPIO_NPUPD_FMC)
    });
    pg.afrl.modify(|_, w| unsafe {
        w.afrl0().bits(ALT_FN_FMC)
         .afrl1().bits(ALT_FN_FMC)
         .afrl4().bits(ALT_FN_FMC)
         .afrl5().bits(ALT_FN_FMC)
    });
    pg.afrh.modify(|_, w| unsafe {
        w.afrh8().bits(ALT_FN_FMC)
         .afrh15().bits(ALT_FN_FMC)
    });
}

pub fn init() {
    init_gpios();

    let fmc = unsafe{&*FMC.get()};
    let rcc = unsafe{&*RCC.get()};

    rcc.ahb3enr.write(|w| w.fmcen().bit(true));

    fmc.sdcr1.modify(|_, w| unsafe {
        w.rpipe().bits(0b01)    // one hclk cycle delay
         .rburst().bit(false)   // burst disabled
         .sdclk().bits(0b10)    // sdclk period 2x hclk period
    });

    fmc.sdcr2.modify(|_, w| unsafe{
        w.cas().bits(0b11)      // cas latency 3 cycles
         .nb().bit(true)        // four internal banks number
         .mwid().bits(0b01)     // 16bits memory data bus width
         .nr().bits(0b01)       // row bits number : 12
         .nc().bits(0b00)       // col bits number : 8
    });

    // check stm32f429i_discovery_sdram.c
    // for more information on coming configurations bits
    fmc.sdtr1.modify(|_, w| unsafe {
        w.trp().bits(2)
         .trc().bits(7)
    });

    fmc.sdtr2.modify(|_, w| unsafe{
        w.trcd().bits(2)
         .twr().bits(2)
         .tras().bits(4)
         .txsr().bits(7)
         .tmrd().bits(2)
    });

    init_sequence();
}

pub fn init_sequence() {
    let fmc = unsafe{&*FMC.get()};

    while fmc.sdsr.read().busy().bit() == true {}
    // configure clock conf enable command
    fmc.sdcmr.modify(|_, w| unsafe{
        w.mrd().bits(0)
         .nrfs().bits(1)
         .ctb1().bit(false)
         .ctb2().bit(true)
         .mode().bits(0b001)
    });

    misc::delay(0xFFF);

    while fmc.sdsr.read().busy().bit() == true {}

    // configure pall precharge all command
    fmc.sdcmr.modify(|_, w| unsafe{
        w.mrd().bits(0)
         .nrfs().bits(1)
         .ctb1().bit(false)
         .ctb2().bit(true)
         .mode().bits(0b010)
    });

    while fmc.sdsr.read().busy().bit() == true {}

    // configure autorefresh, first command
    fmc.sdcmr.modify(|_, w| unsafe{
        w.mrd().bits(0)
         .nrfs().bits(4)
         .ctb1().bit(false)
         .ctb2().bit(true)
         .mode().bits(0b011)
    });

    while fmc.sdsr.read().busy().bit() == true {}

    // configure autorefresh, second command
    fmc.sdcmr.modify(|_, w| unsafe{
        w.mrd().bits(0)
         .nrfs().bits(4)
         .ctb1().bit(false)
         .ctb2().bit(true)
         .mode().bits(0b011)
    });

    while fmc.sdsr.read().busy().bit() == true {}

    // program the external memory mode register
    fmc.sdcmr.modify(|_, w| unsafe{
        w.mrd().bits(0x231)
         .nrfs().bits(1)
         .ctb1().bit(false)
         .ctb2().bit(true)
         .mode().bits(0b100)
    });

    while fmc.sdsr.read().busy().bit() == true {}

    fmc.sdrtr.modify(|_, w| unsafe{
        w.count().bits(1366)
    });

    while fmc.sdsr.read().busy().bit() == true {}

    fmc.sdcr2.modify(|_, w| w.wp().bit(false));

}

pub fn write_buffer(buf : &[u32], addr : u32) -> Result<(), SdRamError> {
    if (addr + (buf.len() as u32) * SDRAM_STORAGE_ELEMENTS_SIZE) > SDRAM_SIZE {
        let ohs = (addr + (buf.len() as u32) * SDRAM_STORAGE_ELEMENTS_SIZE) - SDRAM_SIZE;
        return Err(SdRamError::OutOfBoundsAccess(ohs));
    }

    let align = (SDRAM_BANK_ADDR + addr) % SDRAM_STORAGE_ELEMENTS_SIZE;
    if  align != 0 {
        let next_available_address = SDRAM_BANK_ADDR + addr + (SDRAM_STORAGE_ELEMENTS_SIZE - align);
        return Err(SdRamError::UnalignedAccess(next_available_address));
    }

    let fmc = unsafe{&*FMC.get()};
    while fmc.sdsr.read().busy().bit() == true {}

    // sd ram refresh error
    if fmc.sdsr.read().re().bit() {
        return Err(SdRamError::RefreshError);
    }

    for (i, item) in buf.iter().enumerate() {
        unsafe {
            let mut mem_loc = (SDRAM_BANK_ADDR + addr +
                              (i as u32 * SDRAM_STORAGE_ELEMENTS_SIZE)) as *mut u32;
            *mem_loc = *item;
            cortex_m::asm::nop();
        }
    }
    Ok(())
}

pub fn read_buffer(buf : &mut [u32], addr : u32) -> Result<(), SdRamError> {
    if (addr + (buf.len() as u32) * SDRAM_STORAGE_ELEMENTS_SIZE) > SDRAM_SIZE {
        let ohs = (addr + (buf.len() as u32) * SDRAM_STORAGE_ELEMENTS_SIZE) - SDRAM_SIZE;
        return Err(SdRamError::OutOfBoundsAccess(ohs));
    }

    let align = (SDRAM_BANK_ADDR + addr) % SDRAM_STORAGE_ELEMENTS_SIZE;
    if  align != 0 {
        let next_available_address = SDRAM_BANK_ADDR + addr + (SDRAM_STORAGE_ELEMENTS_SIZE - align);
        return Err(SdRamError::UnalignedAccess(next_available_address));
    }

    let fmc = unsafe{&*FMC.get()};
    while fmc.sdsr.read().busy().bit() == true {}

    let mut mem_ptr = SDRAM_BANK_ADDR + addr;
    let mut buf_ptr = 0;
    let last_mem_loc = SDRAM_BANK_ADDR + addr +
                       (buf.len() as u32) * SDRAM_STORAGE_ELEMENTS_SIZE;
    while mem_ptr < last_mem_loc {
        unsafe {
            buf[buf_ptr] = *(mem_ptr as *mut u32);
        }
        mem_ptr += SDRAM_STORAGE_ELEMENTS_SIZE;
        buf_ptr += 1;
    }

    Ok(())
}
