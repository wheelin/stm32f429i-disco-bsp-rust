use stm32f429::RCC;

bitflags! {
    pub struct ClkFlag : u32 {
        const PLL_SAI_RDY = 1 << 29;
        const PLL_I2S_RDY = 1 << 27;
        const PLL_RDY     = 1 << 25;
        const HSE_RDY     = 1 << 17;
        const HSI_RDY     = 1 << 1 ;
    }
}

pub fn check_flag(f : ClkFlag) -> bool {
    let rcc = unsafe {&*RCC.get()};
    ((rcc.cr.read().bits() & f.bits()) != 0)
}

bitflags! {
    pub struct Clock : u32 {
        const PLL_SAI_RDY   = 1 << 28;
        const PLL_I2S_ON    = 1 << 26;
        const PLL_ON        = 1 << 24;
        const CSS_ON        = 1 << 19;
        const HSE_BYP       = 1 << 18;
        const HSE_ON        = 1 << 16;
        const HSI_ON        = 1 << 0 ;
    }
}

pub fn clock_ctrl(c : Clock, state : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.cr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if state {
            o | c.bits()
        } else {
            o & !c.bits()
        };
        w.bits(n)
    });
}

pub fn select_pll_src(hse : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.pllcfgr.modify(|_, w| {
        w.pllsrc().bit(hse)
    });
}

pub fn configure_pll(q : u8, n : u16, p : u8, m : u8) -> Result<(), ()> {
    if q > 0b1111 {
        return Err(())
    }

    if p > 0b11 {
        return Err(())
    }

    if n > 0x1FF {
        return Err(())
    }

    if m > 0x1F {
        return Err(())
    }

    let rcc = unsafe {&*RCC.get()};
    rcc.pllcfgr.modify(|_, w| unsafe {
        w.pllq().bits(q)
         .pllp().bits(p)
         .plln().bits(n)
         .pllm().bits(m)
    });
    Ok(())
}

pub enum Mco2ClockSrc {
    SysClk    = 0b00,
    PllI2s    = 0b01,
    Hse       = 0b10,
    Pll       = 0b11,
}

pub fn set_mco2_src(clk : Mco2ClockSrc) {
    let rcc = unsafe {&*RCC.get()};
    rcc.cfgr.modify(|_, w| unsafe {
        w.mco2().bits(clk as u8)
    });
}

pub enum Mco1ClockSrc {
    Hsi = 0b00,
    Lse = 0b01,
    Hse = 0b10,
    Pll = 0b11,
}

pub fn set_mco1_src(clk : Mco1ClockSrc) {
    let rcc = unsafe {&*RCC.get()};
    rcc.cfgr.modify(|_, w| unsafe {
        w.mco1().bits(clk as u8)
    });
}

pub enum McoPre {
    DivBy2 = 0b100,
    DivBy3 = 0b101,
    DivBy4 = 0b110,
    DivBy5 = 0b111,
}

pub fn set_mco2_pre(pre : McoPre) {
    let rcc = unsafe {&*RCC.get()};
    rcc.cfgr.modify(|_, w| unsafe {
        w.mco2pre().bits(pre as u8)
    });
}

pub fn set_mco1_pre(pre : McoPre) {
    let rcc = unsafe {&*RCC.get()};
    rcc.cfgr.modify(|_, w| unsafe {
        w.mco1pre().bits(pre as u8)
    });
}

pub enum I2sSrc {
    Hsi = 0,
    Hse = 1,
}

pub fn set_plli2s_src(is : I2sSrc) {
    let rcc = unsafe {&*RCC.get()};
    rcc.pllcfgr.modify(|_, w| unsafe {
        match is {
            I2sSrc::Hsi => w.pllsrc().bit(false),
            I2sSrc::Hse => w.pllsrc().bit(false),
        }
    });
}

pub fn set_rtc_div(d : u8) -> Result<(), ()> {
    if d > 31 {
        return Err(())
    }

    let rcc = unsafe {&*RCC.get()};
    rcc.cfgr.modify(|_, w| unsafe {
        w.rtcpre().bits(d)
    });

    Ok(())
}

pub enum ApbPre {
    Div1 = 0b000,
    Div2 = 0b100,
    Div4 = 0b101,
    Div8 = 0b110,
    Div16 = 0b111,
}

pub fn set_apb2_pre(ad : ApbPre) {
    let rcc = unsafe {&*RCC.get()};
    rcc.cfgr.modify(|_, w| unsafe {
        w.ppre2().bits(ad as u8)
    });
}

pub fn set_apb1_pre(ad : ApbPre) {
    let rcc = unsafe {&*RCC.get()};
    rcc.cfgr.modify(|_, w| unsafe {
        w.ppre1().bits(ad as u8)
    });
}

pub enum AhbPre {
    Div1 = 0b0000,
    Div2 = 0b1000,
    Div4 = 0b1001,
    Div8 = 0b1010,
    Div16 = 0b1011,
    Div64 = 0b1100,
    Div128 = 0b1101,
    Div256 = 0b1110,
    Div512 = 0b1111,
}

pub fn set_ahb_pre(ap : AhbPre) {
    let rcc = unsafe {&*RCC.get()};
    rcc.cfgr.modify(|_, w| unsafe {
        w.hpre().bits(ap as u8)
    });
}

#[derive(PartialEq)]
pub enum SysClkSrc {
    Hsi = 0b00,
    Hse = 0b01,
    Pll = 0b10,
}

impl SysClkSrc {
    pub fn from_bits(b : u8) -> SysClkSrc {
        match b {
            0 => SysClkSrc::Hsi,
            1 => SysClkSrc::Hse,
            2 => SysClkSrc::Pll,
            _ => SysClkSrc::Hsi,
        }
    }
}

pub fn get_sysclk_src() -> SysClkSrc {
    let rcc = unsafe {&*RCC.get()};
    SysClkSrc::from_bits(rcc.cfgr.read().sws().bits())
}

pub fn set_sysclk_src(sw : SysClkSrc) {
    let rcc = unsafe {&*RCC.get()};
    rcc.cfgr.modify(|_, w| unsafe {
        w.sw().bits(sw as u8)
    });
}

bitflags! {
    pub struct InterruptClear : u32 {
        const CSSC          = 1 << 23;
        const PLL_SAI_RDYC  = 1 << 22;
        const PLL_I2S_RDYC  = 1 << 21;
        const PLL_RDYC      = 1 << 20;
        const HSE_RDYC      = 1 << 19;
        const HSI_RDYC      = 1 << 18;
        const LSE_RDYC      = 1 << 17;
        const LSI_RDYC      = 1 << 16;
    }
}

bitflags! {
    pub struct InterruptEnable : u32 {
        const PLL_SAI_RDYIE = 1 << 14;
        const PLL_I2S_RDYIE = 1 << 13;
        const PLL_RDYIE     = 1 << 12;
        const HSE_RDYIE     = 1 << 11;
        const HSI_RDYIE     = 1 << 10;
        const LSE_RDYIE     = 1 << 9;
        const LSI_RDYIE     = 1 << 8;
    }
}

bitflags! {
    pub struct InterruptFlag : u32 {
        const CSSF          = 1 << 7;
        const PLL_SAI_RDYF  = 1 << 6;
        const PLL_I2S_RDYF  = 1 << 5;
        const PLL_RDYF      = 1 << 4;
        const HSE_RDYF      = 1 << 3;
        const HSI_RDYF      = 1 << 2;
        const LSE_RDYF      = 1 << 1;
        const LSI_RDYF      = 1 << 0;
    }
}

pub fn clear_interrupt_flag(ic : InterruptClear) {
    let rcc = unsafe {&*RCC.get()};
    rcc.cir.modify(|r, w| unsafe {
        let o = r.bits();
        let n = o | ic.bits();
        w.bits(n)
    });
}

pub fn set_interrupt(ie : InterruptEnable, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.cir.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ie.bits()
        } else {
            o & !ie.bits()
        };
        w.bits(n)
    });
}

pub fn check_interrupt_flag(intf : InterruptFlag) -> bool {
    let rcc = unsafe {&*RCC.get()};
    let rf = InterruptFlag::from_bits(rcc.cir.read().bits());
    let rf = if let Some(rf) = rf {
        rf
    } else {
        InterruptFlag::empty()
    };
    if rf.contains(intf) {
        true
    } else {
        false
    }
}

bitflags! {
    pub struct Ahb1Reset : u32 {
        const OTG_HS  = 1 << 29;
        const ETH_MAC = 1 << 25;
        const DMA_2D  = 1 << 23;
        const DMA2    = 1 << 22;
        const DMA1    = 1 << 21;
        const CRC     = 1 << 12;
        const GPIOK   = 1 << 10;
        const GPIOJ   = 1 << 9;
        const GPIOI   = 1 << 8;
        const GPIOH   = 1 << 7;
        const GPIOG   = 1 << 6;
        const GPIOF   = 1 << 5;
        const GPIOE   = 1 << 4;
        const GPIOD   = 1 << 3;
        const GPIOC   = 1 << 2;
        const GPIOB   = 1 << 1;
        const GPIOA   = 1 << 0;
    }
}

pub fn reset_ahb1_periph(ar : Ahb1Reset, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.ahb1rstr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ar.bits()
        } else {
            o & !ar.bits()
        };
        w.bits(n)
    });
}

bitflags! {
    pub struct Ahb2Reset : u32 {
        const OTG_FS    = 1 << 7;
        const RNG       = 1 << 6;
        const HASH      = 1 << 5;
        const CRYP      = 1 << 4;
        const DCMI      = 1 << 0;
    }
}

pub fn reset_ahb2_periph(ar : Ahb2Reset, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.ahb2rstr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ar.bits()
        } else {
            o & !ar.bits()
        };
        w.bits(n)
    });
}

bitflags! {
    pub struct Ahb3Reset : u32 {
        const FSMC    = 1 << 0;
    }
}

pub fn reset_ahb3_periph(ar : Ahb3Reset, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.ahb3rstr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ar.bits()
        } else {
            o & !ar.bits()
        };
        w.bits(n)
    });
}

bitflags! {
    pub struct Apb1Reset : u32 {
        const UART8  = 1 << 31;
        const UART7  = 1 << 30;
        const DAC    = 1 << 29;
        const PWR    = 1 << 28;
        const CAN2   = 1 << 26;
        const CAN1   = 1 << 25;
        const I2C3   = 1 << 23;
        const I2C2   = 1 << 22;
        const I2C1   = 1 << 21;
        const UART5  = 1 << 20;
        const UART4  = 1 << 19;
        const USART3 = 1 << 18;
        const USART2 = 1 << 17;
        const SPI3   = 1 << 15;
        const SPI2   = 1 << 14;
        const WWDG   = 1 << 11;
        const TIM14  = 1 << 8;
        const TIM13  = 1 << 7;
        const TIM12  = 1 << 6;
        const TIM7   = 1 << 5;
        const TIM6   = 1 << 4;
        const TIM5   = 1 << 3;
        const TIM4   = 1 << 2;
        const TIM3   = 1 << 1;
        const TIM2   = 1 << 0;
    }
}

pub fn reset_apb1_periph(ar : Apb1Reset, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.apb1rstr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ar.bits()
        } else {
            o & !ar.bits()
        };
        w.bits(n)
    });
}

bitflags! {
    pub struct Apb2Reset : u32 {
        const LTDC    = 1 << 26;
        const SAI1    = 1 << 22;
        const SPI6    = 1 << 21;
        const SPI5    = 1 << 20;
        const TIM11   = 1 << 18;
        const TIM10   = 1 << 17;
        const TIM9    = 1 << 16;
        const SYS_CFG = 1 << 14;
        const SPI4    = 1 << 13;
        const SPI1    = 1 << 12;
        const SDIO    = 1 << 11;
        const ADC     = 1 << 8;
        const USART6  = 1 << 5;
        const USART1  = 1 << 4;
        const TIM8    = 1 << 1;
        const TIM1    = 1 << 0;
    }
}

pub fn reset_apb2_periph(ar : Apb2Reset, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.apb2rstr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ar.bits()
        } else {
            o & !ar.bits()
        };
        w.bits(n)
    });
}

//////////////////////////////////////////////////////////////////////

bitflags! {
    pub struct Ahb1Enable : u32 {
        const OTG_HS_ULPI  = 1 << 30;
        const OTG_HS       = 1 << 29;
        const ETH_MAC_PTP  = 1 << 28;
        const ETH_MAC_RX   = 1 << 27;
        const ETH_MAC_TX   = 1 << 26;
        const ETH_MAC      = 1 << 25;
        const DMA_2D       = 1 << 23;
        const DMA2         = 1 << 22;
        const DMA1         = 1 << 21;
        const CCM_DATA_RAM = 1 << 20;
        const BKP_SRAM     = 1 << 18;
        const CRC          = 1 << 12;
        const GPIOK        = 1 << 10;
        const GPIOJ        = 1 << 9;
        const GPIOI        = 1 << 8;
        const GPIOH        = 1 << 7;
        const GPIOG        = 1 << 6;
        const GPIOF        = 1 << 5;
        const GPIOE        = 1 << 4;
        const GPIOD        = 1 << 3;
        const GPIOC        = 1 << 2;
        const GPIOB        = 1 << 1;
        const GPIOA        = 1 << 0;
    }
}

pub fn set_ahb1_periph_clk(ae : Ahb1Enable, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.ahb1enr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ae.bits()
        } else {
            o & !ae.bits()
        };
        w.bits(n)
    });
}

bitflags! {
    pub struct Ahb2Enable : u32 {
        const OTG_FS = 1 << 7;
        const RNG    = 1 << 6;
        const HASH   = 1 << 5;
        const CRYP   = 1 << 4;
        const DCMI   = 1 << 0;
    }
}

pub fn set_ahb2_periph_clk(ae : Ahb2Enable, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.ahb2enr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ae.bits()
        } else {
            o & !ae.bits()
        };
        w.bits(n)
    });
}

bitflags! {
    pub struct Ahb3Enable : u32 {
        const FSMC = 1 << 0;
    }
}

pub fn set_ahb3_periph_clk(ae : Ahb3Enable, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.ahb3enr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ae.bits()
        } else {
            o & !ae.bits()
        };
        w.bits(n)
    });
}

bitflags! {
    pub struct Apb1Enable : u32 {
        const UART8  = 1 << 31;
        const UART7  = 1 << 30;
        const DAC    = 1 << 29;
        const PWR    = 1 << 28;
        const CAN2   = 1 << 26;
        const CAN1   = 1 << 25;
        const I2C3   = 1 << 23;
        const I2C2   = 1 << 22;
        const I2C1   = 1 << 21;
        const UART5  = 1 << 20;
        const UART4  = 1 << 19;
        const USART3 = 1 << 18;
        const USART2 = 1 << 17;
        const SPI3   = 1 << 15;
        const SPI2   = 1 << 14;
        const WWDG   = 1 << 11;
        const TIM14  = 1 << 8;
        const TIM13  = 1 << 7;
        const TIM12  = 1 << 6;
        const TIM7   = 1 << 5;
        const TIM6   = 1 << 4;
        const TIM5   = 1 << 3;
        const TIM4   = 1 << 2;
        const TIM3   = 1 << 1;
        const TIM2   = 1 << 0;
    }
}

pub fn set_apb1_periph_clk(ae : Apb1Enable, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.apb1enr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ae.bits()
        } else {
            o & !ae.bits()
        };
        w.bits(n)
    });
}

bitflags! {
    pub struct Apb2Enable : u32 {
        const LTDC    = 1 << 26;
        const SAI1    = 1 << 22;
        const SPI6    = 1 << 21;
        const SPI5    = 1 << 20;
        const TIM11   = 1 << 18;
        const TIM10   = 1 << 17;
        const TIM9    = 1 << 16;
        const SYS_CFG = 1 << 14;
        const SPI1    = 1 << 12;
        const SDIO    = 1 << 11;
        const ADC3    = 1 << 10;
        const ADC2    = 1 << 9;
        const ADC1    = 1 << 8;
        const USART6  = 1 << 5;
        const USART1  = 1 << 4;
        const TIM8    = 1 << 1;
        const TIM1    = 1 << 0;
    }
}

pub fn set_apb2_periph_clk(ae : Apb2Enable, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.apb2enr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ae.bits()
        } else {
            o & !ae.bits()
        };
        w.bits(n)
    });
}

////////////////////////////////////////////////////////////////////////////////
pub fn set_lp_ahb1_periph_clk(ae : Ahb1Enable, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.ahb1lpenr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ae.bits()
        } else {
            o & !ae.bits()
        };
        w.bits(n)
    });
}

pub fn set_lp_ahb2_periph_clk(ae : Ahb2Enable, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.ahb2lpenr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ae.bits()
        } else {
            o & !ae.bits()
        };
        w.bits(n)
    });
}

pub fn set_lp_ahb3_periph_clk(ae : Ahb3Enable, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.ahb3lpenr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ae.bits()
        } else {
            o & !ae.bits()
        };
        w.bits(n)
    });
}

pub fn set_lp_apb1_periph_clk(ae : Apb1Enable, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.apb1lpenr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ae.bits()
        } else {
            o & !ae.bits()
        };
        w.bits(n)
    });
}

pub fn set_lp_apb2_periph_clk(ae : Apb2Enable, en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.apb2lpenr.modify(|r, w| unsafe {
        let o = r.bits();
        let n = if en {
            o | ae.bits()
        } else {
            o & !ae.bits()
        };
        w.bits(n)
    });
}

////////////////////////////////////////////////////////////////////////////////
pub fn reset_backup_domain(en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.bdcr.modify(|_, w| {
        w.bdrst().bit(en)
    });
}

pub fn set_rtc(en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.bdcr.modify(|_, w| {
        w.rtcen().bit(en)
    });
}

#[derive(Copy, Clone)]
pub enum RtcClkSrc {
    NoClk = 0b00,
    Lse   = 0b01,
    Lsi   = 0b10,
    Hse   = 0b11,
}

pub fn set_rtc_clk_src(rcs : RtcClkSrc) {
    let rcc = unsafe {&*RCC.get()};
    rcc.bdcr.modify(|_, w| {
        let sel0 = ((rcs as u8) & 1) != 0;
        let sel1 = ((rcs as u8) >> 1) & 1 != 0;
        w.rtcsel0().bit(sel0)
         .rtcsel1().bit(sel1)
    });
}

pub fn enable_lse_bypass(en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.bdcr.modify(|_, w| {
        w.lsebyp().bit(en)
    });
}

pub fn set_lse(en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.bdcr.modify(|_, w| {
        w.lseon().bit(en)
    });
}

pub fn get_lse_status() -> bool {
    let rcc = unsafe {&*RCC.get()};
    rcc.bdcr.read().lserdy().bit()
}

////////////////////////////////////////////////////////////////////////////////
bitflags! {
    pub struct ResetFlag : u32 {
        const LPWR_RST = 1 << 31;
        const WWDG_RST = 1 << 30;
        const IWDG_RST = 1 << 29;
        const SOFT_RST = 1 << 28;
        const POR_RST  = 1 << 27;
        const PIN_RST  = 1 << 26;
        const BOR_RST  = 1 << 25;
    }
}

pub fn get_reset_flag() -> ResetFlag {
    let rcc = unsafe {&*RCC.get()};
    let ret = ResetFlag::from_bits(rcc.csr.read().bits() & 0xFF000000);
    if let Some(ret) = ret {
        ret
    } else {
        ResetFlag::empty()
    }
}

pub fn clear_reset_flag() {
    let rcc = unsafe {&*RCC.get()};
    rcc.csr.modify(|_, w| w.rmvf().bit(true));
}

pub fn check_lsi_flag() -> bool {
    let rcc = unsafe {&*RCC.get()};
    rcc.csr.read().lsirdy().bit()
}

pub fn set_lsi(en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.csr.modify(|r, w| {
        w.lsion().bit(en)
    });
}

////////////////////////////////////////////////////////////////////////////////
pub fn set_sspm(en : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.sscgr.modify(|_, w| {
        w.sscgen().bit(en)
    });
}

pub fn select_spread(down : bool) {
    let rcc = unsafe {&*RCC.get()};
    rcc.sscgr.modify(|_, w| {
        w.spreadsel().bit(down)
    });
}

pub fn set_sscg_inc_step(is : u16) -> Result<(), ()> {
    if is > 32767 {
        return Err(())
    }
    let rcc = unsafe {&*RCC.get()};
    rcc.sscgr.modify(|_, w| unsafe {
        w.incstep().bits(is)
    });

    Ok(())
}

pub fn set_sscg_mod_period(mp : u16) -> Result<(), ()>{
    if mp > 4095 {
        return Err(())
    }
    let rcc = unsafe {&*RCC.get()};
    rcc.sscgr.modify(|_, w| unsafe {
        w.modper().bits(mp)
    });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

pub fn conf_i2s_pll(r : u8, q : u8, n : u16) -> Result<(), ()> {
    match r {
        0 | 1 => return Err(()),
        x if x > 7 => return Err(()),
        _ => (),
    };

    if n > 511 {
        return Err(())
    }

    match n {
        x if x < 50 => return Err(()),
        x if x > 432 => return Err(()),
        _ => (),
    };

    match q {
        0 | 1 => return Err(()),
        x if x > 15 => return Err(()),
        _ => (),
    };

    let rcc = unsafe {&*RCC.get()};
    rcc.plli2scfgr.modify(|_, w| unsafe {
        w.plli2sr().bits(r)
         .plli2sn().bits(n)
         .plli2sq().bits(q)
    });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

pub fn conf_sai_pll(r : u8, q : u8, n : u16) -> Result<(), ()> {
    match r {
        x if x < 2 => return Err(()),
        x if x > 7 => return Err(()),
        _ => (),
    };

    match q {
        x if x < 2 => return Err(()),
        _ => (),
    };

    match n {
        x if x < 50 => return Err(()),
        x if x > 432 => return Err(()),
        _ => (),
    };

    let rcc = unsafe {&*RCC.get()};
    rcc.pllsaicfgr.modify(|_, w| unsafe {
        w.pllsair().bits(r)
         .pllsain().bits(n)
         .pllsaiq().bits(q)
    });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
pub enum TimMul {
    Time2,
    Time4,
}

pub fn set_timers_freq(m : TimMul) {
    let rcc = unsafe {&*RCC.get()};
    rcc.dckcfgr.modify(|_, w| {
        match m {
            TimMul::Time2 => w.timpre().bit(false),
            TimMul::Time4 => w.timpre().bit(true),
        }
    });
}

pub enum Sai1BClkSrc {
    FSaiQDivQ = 0b00,
    FI2sQDivQ = 0b01,
    FAltInFreq = 0b10,
}

pub fn set_sai1b_clk_src(sbcs : Sai1BClkSrc) {
    let rcc = unsafe {&*RCC.get()};
    rcc.dckcfgr.modify(|_, w| unsafe {
        w.sai1bsrc().bits(sbcs as u8)
    });
}

pub enum Sai1AClkSrc {
    FSai1AQDivQ = 0b00,
    FI2sQDivQ = 0b01,
    FAltInFreq = 0b10,
}

pub fn set_sai1a_clk_src(sacs : Sai1AClkSrc) {
    let rcc = unsafe {&*RCC.get()};
    rcc.dckcfgr.modify(|_, w| unsafe {
        w.sai1asrc().bits(sacs as u8)
    });
}

pub enum LcdPllSaiDiv {
    Div2 = 0b00,
    Div4 = 0b01,
    Div8 = 0b10,
    Div16 = 0b11,
}

pub fn conf_lcd_pllsai_divr(d : LcdPllSaiDiv) {
    let rcc = unsafe {&*RCC.get()};
    rcc.dckcfgr.modify(|_, w| unsafe {
        w.pllsaidivr().bits(d as u8)
    });
}

pub fn conf_sai1_pllsai_divq(d : u8) -> Result<(), ()>{
    if d > 32 || d < 1 {
        return Err(())
    }
    let d = d - 1;
    let rcc = unsafe {&*RCC.get()};
    rcc.dckcfgr.modify(|_, w| unsafe {
        w.pllsaidivq().bits(d)
    });
    Ok(())
}

pub fn conf_i2s_pllsai_divq(d : u8) -> Result<(), ()> {
    if d > 32 || d < 1 {
        return Err(())
    }
    let d = d - 1;
    let rcc = unsafe {&*RCC.get()};
    rcc.dckcfgr.modify(|_, w| unsafe {
        w.pllis2divq().bits(d)
    });
    Ok(())
}
