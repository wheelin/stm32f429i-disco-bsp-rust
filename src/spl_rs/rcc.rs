use stm32f429::RCC;

bitflags! {
    pub struct Flag : u32 {
        const PLL_I2S_RDY = 1 << 27;
        const PLL_RDY     = 1 << 25;
        const HSE_RDY     = 1 << 17;
        const HSI_RDY     = 1 << 1 ;
    }
}

pub fn check_flag(f : u32) -> Result<bool , ()> {
    if f & Flag::PLL_I2S_RDY == 0 &&
        f & Flag::PLL_RDY == 0 &&
        f & Flag::HSE_RDY == 0 &&
        f & Flag::HSI_RDY == 0
    {
        return Err(())
    }

    let rcc = unsafe {&*RCC.get()};
    ((rcc.cr.read().bits() & f) != 0)
}

bitflags! {
    pub struct Clock : u32 {
        const PLL_I2S_ON    = 1 << 26;
        const PLL_ON        = 1 << 24;
        const CSS_ON        = 1 << 19;
        const HSE_BYP       = 1 << 18;
        const HSE_ON        = 1 << 16;
        const HSI_ON        = 1 << 0 ;
    }
}

pub fn clock_ctrl(c : u32, state : bool) -> Result<(), ()> {
    
}

pub fn configure_pll(q : u8, n : u8, p : u8, m : u8) -> Result<(), ()> {

}

pub enum Mco2ClockSrc {
    SysClk    = 0b00,
    PllI2s    = 0b01,
    Hse       = 0b10,
    Pll       = 0b11,
}

pub fn set_mco2_src(clk : Clock) {

}

pub enum McoPre {
    DivBy2 = 0b100,
    DivBy3 = 0b101,
    DivBy4 = 0b110,
    DivBy5 = 0b111,
}

pub fn set_mco2_pre(pre : McoPre) {

}

pub fn set_mco1_pre(pre : McoPre) {

}

pub enum I2sSrc {
    PllI2s = 0,
    ExtClk = 1,
}

pub fn set_i2s_src(is : I2sSrc) {

}

pub enum Mco1OutSrc {
    Hsi = 0b00,
    Lse = 0b01,
    Hse = 0b10,
    Pll = 0b11,
}

pub fn set_mco1_clk_output(o : Mco1OutSrc) {

}

pub fn set_rtc_div(d : u8) {

}

pub enum ApbPre {
    Div1 = 0b000,
    Div2 = 0b100,
    Div4 = 0b101,
    Div8 = 0b110,
    Div16 = 0b111,
}

pub fn set_apb2_pre(ad : ApbPre) {

}

pub fn set_apb1_pre(ad : ApbPre) {

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

}

pub enum SysClkSrc {
    Hsi = 0b00,
    Hse = 0b01,
    Pll = 0b10,
}

pub fn get_sysclk_src() -> SysClkSrc {

}

pub fn set_sysclk_src(sw : SysClkStat) {

}

bitflags! {
    pub struct InterruptClear : u32 {
        const CSSC          = 1 << 23;
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
        const PLL_I2S_RDYF  = 1 << 5;
        const PLL_RDYF      = 1 << 4;
        const HSE_RDYF      = 1 << 3;
        const HSI_RDYF      = 1 << 2;
        const LSE_RDYF      = 1 << 1;
        const LSI_RDYF      = 1 << 0;
    }
}

pub fn clear_interrupt_flag(ic : InterruptClear) -> Result<(), ()> {

}

pub fn enable_interrupt(ie : InterruptEnable) -> Result<(), ()> {

}

pub fn check_interrupt_flag(if : InterruptFlag) -> Result<(), ()> {

}

bitflags! {
    pub struct Ahb1Reset : u32 {
        const OTG_HS_RST    = 1 << 29;
        const ETH_MAC_RST   = 1 << 25;
        const DMA2_RST      = 1 << 22;
        const DMA1_RST      = 1 << 21;
        const CRC_RST       = 1 << 12;
        const GPIOI_RST     = 1 << 8;
        const GPIOH_RST     = 1 << 7;
        const GPIOG_RST     = 1 << 6;
        const GPIOF_RST     = 1 << 5;
        const GPIOE_RST     = 1 << 4;
        const GPIOD_RST     = 1 << 3;
        const GPIOC_RST     = 1 << 2;
        const GPIOB_RST     = 1 << 1;
        const GPIOA_RST     = 1 << 0;
    }
}

pub fn reset_ahb1_periph(ar : u32) -> Result<(), ()> {

}

bitflags! {
    pub struct Ahb2Reset : u32 {
        const OTG_FS_RST    = 1 << 7;
        const RNG_RST       = 1 << 6;
        const HASH_RST      = 1 << 5;
        const CRYP_RST      = 1 << 4;
        const DCMI_RST      = 1 << 0;
    }
}

pub fn reset_ahb2_periph(ar : u32) -> Result<(), ()> {

}

bitflags! {
    pub struct Ahb3Reset : u32 {
        const FSMC_RST    = 1 << 0;
    }
}

pub fn reset_ahb3_periph(ar : u32) -> Result<(), ()> {

}

bitflags! {
    pub struct Apb1Reset : u32 {
        const DAC_RST       = 1 << 29;
        const PWR_RST       = 1 << 28;
        const CAN2_RST      = 1 << 26;
        const CAN1_RST      = 1 << 25;
        const I2C3_RST      = 1 << 23;
        const I2C2_RST      = 1 << 22;
        const I2C1_RST      = 1 << 21;
        const UART5_RST     = 1 << 20;
        const UART4_RST     = 1 << 19;
        const UART3_RST     = 1 << 18;
        const UART2_RST     = 1 << 17;
        const SPI3_RST      = 1 << 15;
        const SPI2_RST      = 1 << 14;
        const WWDG_RST      = 1 << 11;
        const TIM14_RST     = 1 << 8;
        const TIM13_RST     = 1 << 7;
        const TIM12_RST     = 1 << 6;
        const TIM7_RST     = 1 << 5;
        const TIM6_RST     = 1 << 4;
        const TIM5_RST     = 1 << 3;
        const TIM4_RST     = 1 << 2;
        const TIM3_RST     = 1 << 1;
        const TIM2_RST     = 1 << 0;
    }
}

pub fn reset_apb1_periph(ar : u32) -> Result<(), ()> {

}

bitflags! {
    pub struct Apb2Reset : u32 {
        const TIM11_RST   = 1 << 18;
        const TIM10_RST   = 1 << 17;
        const TIM9_RST    = 1 << 16;
        const SYS_CFG_RST = 1 << 14;
        const SPI1_RST    = 1 << 12;
        const SDIO_RST    = 1 << 11;
        const ADC_RST     = 1 << 8;
        const USART6_RST  = 1 << 5;
        const USART1_RST  = 1 << 4;
        const TIM8_RST    = 1 << 1;
        const TIM1_RST    = 1 << 0;
    }
}

pub fn reset_apb2_periph(ar : u32) -> Result<(), ()> {

}

//////////////////////////////////////////////////////////////////////

bitflags! {
    pub struct Ahb1Enable : u32 {
        const OTG_HS_ULPIEN   = 1 << 30;
        const OTG_HS_EN       = 1 << 29;
        const ETH_MAC_PTP_EN  = 1 << 28;
        const ETH_MAC_RX_EN   = 1 << 27;
        const ETH_MAC_TX_EN   = 1 << 26;
        const ETH_MAC_EN      = 1 << 25;
        const DMA2_EN         = 1 << 22;
        const DMA1_EN         = 1 << 21;
        const CCM_DATA_RAM_EN = 1 << 20;
        const BKP_SRAM_EN     = 1 << 18;
        const CRC_EN          = 1 << 12;
        const GPIOI_EN        = 1 << 8;
        const GPIOH_EN        = 1 << 7;
        const GPIOG_EN        = 1 << 6;
        const GPIOF_EN        = 1 << 5;
        const GPIOE_EN        = 1 << 4;
        const GPIOD_EN        = 1 << 3;
        const GPIOC_EN        = 1 << 2;
        const GPIOB_EN        = 1 << 1;
        const GPIOA_EN        = 1 << 0;
    }
}

pub fn set_ahb1_periph_clk(ae : u32, en : bool) -> Result<(), ()> {

}

bitflags! {
    pub struct Ahb2Enable : u32 {
        const OTG_FS_EN = 1 << 7;
        const RNG_EN    = 1 << 6;
        const HASH_EN   = 1 << 5;
        const CRYP_EN   = 1 << 4;
        const DCMI_EN   = 1 << 0;
    }
}

pub fn set_ahb2_periph_clk(ae : u32, en : bool) -> Result<(), ()> {

}

bitflags! {
    pub struct Ahb3Enable : u32 {
        const FSMC_EN = 1 << 0;
    }
}

pub fn set_ahb3_periph_clk(ae : u32, en : bool) -> Result<(), ()> {

}

bitflags! {
    pub struct Apb1Enable : u32 {
        const DAC_EN       = 1 << 29;
        const PWR_EN       = 1 << 28;
        const CAN2_EN      = 1 << 26;
        const CAN1_EN      = 1 << 25;
        const I2C3_EN      = 1 << 23;
        const I2C2_EN      = 1 << 22;
        const I2C1_EN      = 1 << 21;
        const UART5_EN     = 1 << 20;
        const UART4_EN     = 1 << 19;
        const UART3_EN     = 1 << 18;
        const UART2_EN     = 1 << 17;
        const SPI3_EN      = 1 << 15;
        const SPI2_EN      = 1 << 14;
        const WWDG_EN      = 1 << 11;
        const TIM14_EN     = 1 << 8;
        const TIM13_EN     = 1 << 7;
        const TIM12_EN     = 1 << 6;
        const TIM7_EN     = 1 << 5;
        const TIM6_EN     = 1 << 4;
        const TIM5_EN     = 1 << 3;
        const TIM4_EN     = 1 << 2;
        const TIM3_EN     = 1 << 1;
        const TIM2_EN     = 1 << 0;
    }
}

pub fn set_apb1_periph_clk(ae : u32, en : bool) -> Result<(), ()> {

}

bitflags! {
    pub struct Apb2Enable : u32 {
        const TIM11_EN   = 1 << 18;
        const TIM10_EN   = 1 << 17;
        const TIM9_EN    = 1 << 16;
        const SYS_CFG_EN = 1 << 14;
        const SPI1_EN    = 1 << 12;
        const SDIO_EN    = 1 << 11;
        const ADC3_EN    = 1 << 10;
        const ADC2_EN    = 1 << 9;
        const ADC1_EN    = 1 << 8;
        const USART6_EN  = 1 << 5;
        const USART1_EN  = 1 << 4;
        const TIM8_EN    = 1 << 1;
        const TIM1_EN    = 1 << 0;
    }
}

pub fn set_apb2_periph_clk(ae : u32, en : bool) -> Result<(), ()> {

}

////////////////////////////////////////////////////////////////////////////////
pub fn set_lp_ahb1_periph_clk(ae : u32, en : bool) -> Result<(), ()> {

}

pub fn set_lp_ahb2_periph_clk(ae : u32, en : bool) -> Result<(), ()> {

}

pub fn set_lp_ahb3_periph_clk(ae : u32, en : bool) -> Result<(), ()> {

}

pub fn set_lp_apb1_periph_clk(ae : u32, en : bool) -> Result<(), ()> {

}

pub fn set_lp_apb2_periph_clk(ae : u32, en : bool) -> Result<(), ()> {

}

////////////////////////////////////////////////////////////////////////////////
pub fn reset_backup_domain() {

}

pub fn set_rtc(en : bool) {

}

pub enum RtcClkSrc {
    NoClk = 0b00,
    Lse   = 0b01,
    Lsi   = 0b10,
    Hse   = 0b11,
}

pub fn set_rtc_clk_src(rcs : RtcClkSrc) {

}

pub fn enable_lse_bypass(en : bool) {

}

pub fn set_lse(en : bool) {

}

pub fn get_lse_status() -> bool {

}

////////////////////////////////////////////////////////////////////////////////
bitflags! {
    pub struct ResetFlag : u32 {
        const LPwrRst = 1 << 31;
        const WwdgRst = 1 << 30;
        const IWdgRst = 1 << 29;
        const SftRst  = 1 << 28;
        const PorRst  = 1 << 27;
        const PinRst  = 1 << 26;
        const BorRst  = 1 << 25;
    }
}

pub fn get_reset_flag() -> u32 {

}

pub fn clear_reset_flag() {

}

pub fn check_lsi_flag() -> bool {

}

pub fn set_lsi(en : bool) {

}

////////////////////////////////////////////////////////////////////////////////
pub fn set_sspm(en : bool) {

}

pub fn select_spread(down : bool) {

}

pub fn set_sscg_inc_step(is : u16) {

}

pub fn set_sscg_mod_period(mp : u16) {

}

pub fn set_i2s_pll(r : u8, s : u16) {

}
