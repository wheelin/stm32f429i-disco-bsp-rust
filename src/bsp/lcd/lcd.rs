use super::fonts;
use misc;
use stm32f429::{
    SPI5,
    RCC,
    GPIOA, GPIOB, GPIOC, GPIOD, GPIOF, GPIOG,
    LTDC,
};
use sdram;

pub enum LcdError {
    OutOfFrame,
}

pub struct Point {
    x : i16,
    y : i16,
}

pub const LCD_WIDTH             : u16 = 240;
pub const LCD_HEIGHT            : u16 = 320;

const LCD_FRAME_BUFFER_START    : u32 = 0xD0000000;
const LCD_BUFFER_OFFSET         : u32 = 0x50000;

pub enum Register {
    LcdSleepOut      = 0x11, /* Sleep out register */
    LcdGamma         = 0x26, /* Gamma register */
    LcdDisplayOff    = 0x28, /* Display off register */
    LcdDisplayOn     = 0x29, /* Display on register */
    LcdColumnAddr    = 0x2A, /* Colomn address register */
    LcdPageAddr      = 0x2B, /* Page address register */
    LcdGRam          = 0x2C, /* GRAM register */
    LcdMac           = 0x36, /* Memory Access Control register*/
    LcdPixelFormat   = 0x3A, /* Pixel Format register */
    LcdWDB           = 0x51, /* Write Brightness Display register */
    LcdWCD           = 0x53, /* Write Control Display register*/
    LcdRGBInterface  = 0xB0, /* RGB Interface Signal Control */
    LcdFRC           = 0xB1, /* Frame Rate Control register */
    LcdBPC           = 0xB5, /* Blanking Porch Control register*/
    LcdDFC           = 0xB6, /* Display Function Control register*/
    LcdPower1        = 0xC0, /* Power Control 1 register */
    LcdPower2        = 0xC1, /* Power Control 2 register */
    LcdVCOM1         = 0xC5, /* VCOM Control 1 register */
    LcdVCOM2         = 0xC7, /* VCOM Control 2 register */
    LcdPowerA        = 0xCB, /* Power control A register */
    LcdPowerB        = 0xCF, /* Power control B register */
    LcdPGamma        = 0xE0, /* Positive Gamma Correction register*/
    LcdNGamma        = 0xE1, /* Negative Gamma Correction register*/
    LcdDTCA          = 0xE8, /* Driver timing control A */
    LcdDTCB          = 0xEA, /* Driver timing control B */
    LcdPowerSeq      = 0xED, /* Power on sequence register */
    Lcd3GammaEn      = 0xF2, /* 3 Gamma enable register */
    LcdInterface     = 0xF6, /* Interface control register */
    LcdPRC           = 0xF7, /* Pump ratio control register */
}

#[derive(Copy, Clone)]
pub enum Color {
    White   = 0xFFFF,
    Black   = 0x0000,
    Grey    = 0xF7DE,
    Blue    = 0x001F,
    Blue2   = 0x051F,
    Red     = 0xF800,
    Magenta = 0xF81F,
    Green   = 0x07E0,
    Cyan    = 0x7FFF,
    Yellow  = 0xFFE0,
}

pub enum Direction {
    Horizontal = 0x0000,
    Vertical   = 0x0001,
}

pub enum Layer {
    Background = 0x0000,
    Foreground = 0x0001,
}

pub fn assemble_rgb(r : u16, g : u16, b : u16) -> u16 {
    (((r & 0xF8) << 8) | ((g << & 0xFC) << 3) | ((b & 0xF8) >> 3))
}

enum CtrlLinePort {
    PortB,
    PortD,
}

enum CtrlLine {
    Ncs,
    Nwr,
    Rs,
}

pub struct Lcd {
    current_font         : &'static fonts::Font,
    current_text_color   : u16,
    current_back_color   : u16,
    current_frame_buffer : u32,
    current_layer        : Layer,
}

impl Lcd {
    pub fn new() -> Lcd {
        Lcd {
            current_font            : &fonts::FONT_8_X_12,
            current_text_color      : Color::Black,
            current_back_color      : Color::White,
            current_frame_buffer    : LCD_FRAME_BUFFER_START,
            current_layer           : Layer::Background,
        }
    }

    pub fn deinit(&mut self) {
        let spi = unsafe {&*SPI5.get()};
        let rcc = unsafe {&*RCC.get()};

        let pa = unsafe {&*GPIOA.get()};
        let pb = unsafe {&*GPIOB.get()};
        let pc = unsafe {&*GPIOC.get()};
        let pd = unsafe {&*GPIOD.get()};
        let pf = unsafe {&*GPIOF.get()};
        let pg = unsafe {&*GPIOG.get()};

        self.display_off();

        spi.cr1.modify(|_, W| w.spe().bit(false));
        spi.cr1.reset();
        spi.cr2.reset();

        rcc.apb2enr.modify(|_, w| w.spi5en().bit(false));

        // GPIOA ###################################################
        pa.moder.modify(|_, w| unsafe {
            w.moder3().bits(0)
             .moder4().bits(0)
             .moder6().bits(0)
             .moder11().bits(0)
             .moder12().bits(0)
        });
        pa.pupdr.modify(|_, w| unsafe {
            w.pupdr3().bits(0)
             .pupdr4().bits(0)
             .pupdr6().bits(0)
             .pupdr11().bits(0)
             .pupdr12().bits(0)
        });

        pa.afrl.modify(|_, w| unsafe {
            w.afrl3().bits(0)
             .afrl4().bits(0)
             .afrl6().bits(0)
        });
        pa.afrh.modify(|_, w| unsafe {
            w.afrh11().bits(0)
             .afrh12().bits(0)
        });

        // GPIOB ###################################################
        pb.moder.modify(|_, w| unsafe {
            w.moder0().bits(0)
             .moder1().bits(0)
             .moder8().bits(0)
             .moder9().bits(0)
             .moder10().bits(0)
             .moder11().bits(0)
        });
        pb.pupdr.modify(|_, w| unsafe {
            w.pupdr0().bits(0)
             .pupdr1().bits(0)
             .pupdr8().bits(0)
             .pupdr9().bits(0)
             .pupdr10().bits(0)
             .pupdr11().bits(0)
        });

        pb.afrl.modify(|_, w| unsafe {
            w.afrl0().bits(0)
             .afrl1().bits(0)
        });
        pb.afrh.modify(|_, w| unsafe {
            w.afrh8().bits(0)
             .afrh9().bits(0)
             .afrh10().bits(0)
             .afrh11().bits(0)
        });

        // GPIOC ###################################################
        pc.moder.modify(|_, w| unsafe {
            w.moder2().bits(0)
             .moder6().bits(0)
             .moder7().bits(0)
             .moder10().bits(0)
        });
        pc.pupdr.modify(|_, w| unsafe {
            w.pupdr2().bits(0)
             .pupdr6().bits(0)
             .pupdr7().bits(0)
             .pupdr10().bits(0)
        });

        pc.afrl.modify(|_, w| unsafe {
            w.afrl2().bits(0)
             .afrl6().bits(0)
             .afrl7().bits(0)
        });
        pc.afrh.modify(|_, w| unsafe {
            w.afrh10().bits(0)
        });

        // GPIOD ###################################################
        pd.moder.modify(|_, w| unsafe {
            w.moder3().bits(0)
             .moder6().bits(0)
        });
        pd.pupdr.modify(|_, w| unsafe {
            w.pupdr3().bits(0)
             .pupdr6().bits(0)
        });

        pd.afrl.modify(|_, w| unsafe {
            w.afrl3().bits(0)
             .afrl6().bits(0)
        });

        // GPIOF ###################################################
        pf.moder.modify(|_, w| unsafe {
            w.moder7().bits(0)
             .moder8().bits(0)
             .moder9().bits(0)
             .moder10().bits(0)
        });
        pf.pupdr.modify(|_, w| unsafe {
            w.pupdr7().bits(0)
             .pupdr8().bits(0)
             .pupdr9().bits(0)
             .pupdr10().bits(0)
        });

        pf.afrl.modify(|_, w| unsafe {
            w.afrl7().bits(0)
        });
        pf.afrh.modify(|_, w| unsafe {
            w.afrh8().bits(0)
             .afrh9().bits(0)
             .afrh10().bits(0)
        });

        // GPIOG ###################################################
        pg.moder.modify(|_, w| unsafe {
            w.moder6().bits(0)
             .moder7().bits(0)
             .moder10().bits(0)
             .moder11().bits(0)
             .moder12().bits(0)
        });
        pg.pupdr.modify(|_, w| unsafe {
            w.pupdr6().bits(0)
             .pupdr7().bits(0)
             .pupdr10().bits(0)
             .pupdr11().bits(0)
             .pupdr12().bits(0)
        });

        pg.afrl.modify(|_, w| unsafe {
            w.afrl6().bits(0)
             .afrl7().bits(0)
        });
        pg.afrh.modify(|_, w| unsafe {
            w.afrh10().bits(0)
             .afrh11().bits(0)
             .afrh12().bits(0)
        });
    }

    pub fn init(&mut self) {
        let rcc = unsafe {&*RCC.get()};
        let ltdc = unsafe {&*LTDC.get()};

        configure_ctrl_lines();

        self.chip_select(true);
        self.chip_select(false);

        configure_spi();

        self.power_on();

        rcc.apb2enr.modify(|_, w| w.ltdcen().bit(true));
        rcc.ahb1enr.modify(|_, w| w.dma2den().bit(true));

        configure_alt_fn_gpios();

        sdram::init();

        rcc.pllsaicfgr.modify(|_, w| unsafe {
            w.pllsain().bits(192)
             .pllsaiq().bits(7)
             .pllsair().bits(4)
        });

        rcc.dckcfgr.modify(|_, w| unsafe {
            w.pllsaidivr().bits(2)  // divide by 8
        });

        rcc.cr.modify(|_, w| w.pllsaion().bit(true));
        while rcc.cr.read().pllsairdy().bit() == false {}

        ltdc.gcr.modify(|_, w| unsafe {
            w.hspol().bit(false)    // active low
             .vspol().bit(false)    // active low
             .depol().bit(false)    // active low
             .pcpol().bit(false)    // same as input pixel clock
        });

        ltdc.bccr.modify(|_, w| unsafe {
            w.bcred().bits(0)
             .bcgreen().bits(0)
             .bcblue().bits(0)
        });

        ltdc.twcr.modify(|_, w| unsafe{
            w.totalw().bits(279)    // hsync width + hbp + active width + hfp - 1
             .totalh().bits(327)    // vsync height + vbp + active height + vfp - 1
        });

        ltdc.awcr.modify(|_, w| unsafe{
            w.aaw().bits(269)       // hsync width + hbp + active width - 1
             .aah().bits(323)       // vsync height + vhp + active heigh - 1
        });

        ltdc.bpcr.modify(|_, w| unsafe {
            w.ahbp().bits(29)       // hsync width + hbp - 1
             .avpb().bits(3)        // vsync height + vbp - 1
        });

        ltdc.sscr.modify(|_, w| unsafe {
            w.hsw().bits(9)
             .vsh().bits(1)
        });
    }

    pub fn init_layers(&mut self) {
        let ltdc = unsafe {&*LTDC.get()};
        ////////////////////////////////////////////////////////////////
        // first layer configuration
        ltdc.l1whpcr.modify(|_, w| unsafe {
            w.whsppos().bits(LCD_WIDTH + 30 - 1)
             .whstpos().bits(30)
        });

        ltdc.l1wvpcr.modify(|_, w| unsafe {
            w.wvsppos().bits(LCD_HEIGHT + 4 - 1)
             .wvstpos().bits(4)
        });

        // configure pixel format
        ltdc.l1pfcr.modify(|_, w| unsafe {
            w.pf().bits(0b010)      // Pixel format : RGB565
        });

        // constant alpha
        ltdc.l1cacr.modify(|_, w| unsafe {
            w.consta().bits(255)    // totally opaque
        });

        // default colors
        ltdc.l1dccr.modify(|_, w| unsafe {
            w.dcalpha().bits(0)
             .dcred().bits(0)
             .dcgreen().bits(0)
             .dcblue().bits(0)
        });

        ltdc.l1bfcr.modify(|_, w| unsafe {
            w.bf1().bits(0b100)     // constant alpha
             .bf2().bits(0b100)
        });

        // configure start address of the color frame buffer
        ltdc.l1cfbar.write(|w| unsafe {
            w.bits(LCD_FRAME_BUFFER_START)
        });

        ltdc.l1cfblr.modify(|_, w| unsafe {
            w.cfbp().bits(LCD_WIDTH * 2)
             .cfbll().bits((LCD_WIDTH * 2) + 3)
        });

        ltdc.l1cfblnr.modify(|_, w| unsafe {
            w.cfblnbr().bits(LCD_HEIGHT)
        });
        ////////////////////////////////////////////////////////////////
        // second layer configuration

        ltdc.l2whpcr.modify(|_, w| unsafe {
            w.whsppos().bits(LCD_WIDTH + 30 - 1)
             .whstpos().bits(30)
        });

        ltdc.l2wvpcr.modify(|_, w| unsafe {
            w.wvsppos().bits(LCD_HEIGHT + 4 - 1)
             .wvstpos().bits(4)
        });

        // configure pixel format
        ltdc.l2pfcr.modify(|_, w| unsafe {
            w.pf().bits(0b010)      // Pixel format : RGB565
        });

        // constant alpha
        ltdc.l2cacr.modify(|_, w| unsafe {
            w.consta().bits(255)    // totally opaque
        });

        // default colors
        ltdc.l2dccr.modify(|_, w| unsafe {
            w.dcalpha().bits(0)
             .dcred().bits(0)
             .dcgreen().bits(0)
             .dcblue().bits(0)
        });

        // blending factor, change from layer 1
        ltdc.l2bfcr.modify(|_, w| unsafe {
            w.bf1().bits(0b110)     // pixel alpha * constant alpha
             .bf2().bits(0b110)
        });

        // configure start address of the color frame buffer, change from layer 1
        ltdc.l2cfbar.write(|w| unsafe {
            w.bits(LCD_FRAME_BUFFER_START + LCD_BUFFER_OFFSET)
        });

        ltdc.l2cfblr.modify(|_, w| unsafe {
            w.cfbp().bits(LCD_WIDTH * 2)
             .cfbll().bits((LCD_WIDTH * 2) + 3)
        });

        ltdc.l2cfblnr.modify(|_, w| unsafe {
            w.cfblnbr().bits(LCD_HEIGHT)
        });

        // set reload mode
        ltdc.srcr.modify(|_, w| unsafe {
            w.imr().bit(true)       // immediate reload mode
        });

        // enable layer 1 and 2
        ltdc.l1cr.modify(|_, w| w.len().bit(true));
        ltdc.l2cr.modify(|_, w| w.len().bit(true));

        // set default font
        self.current_font = &fonts::FONT_16_X_24;

        // enable dither
        ltdc.gcr.modify(|_, w| w.den().bit(true));
    }

    fn chip_select(&self, en : bool) {
        let pc = unsafe{&*GPIOC.get()};
        if en {
            pc.bsrr.write(|w| w.bs2().bit(true));
        } else {
            pc.bsrr.write(|w| w.br2().bit(true));
        }
    }

    pub fn set_layer(&mut self, l : Layer) {
        match l {
            Layer::Background => {
                self.current_frame_buffer = LCD_FRAME_BUFFER_START;
            },
            Layer::Foreground => {
                self.current_frame_buffer = LCD_FRAME_BUFFER_START + LCD_BUFFER_OFFSET;
            }
        };
        self.current_layer = l;
    }

    pub fn set_colors(&mut self, tc : Color, bc : Color) {
        self.current_back_color = bc;
        self.current_text_color = tc;
    }

    pub fn get_colors(&self) -> (Color, Color) {
        (self.current_text_color, self.current_back_color)
    }

    pub fn set_text_color(&mut self, tc : Color) {

    }

    pub fn set_back_color(&mut self, bc : Color) {

    }

    pub fn set_transparency(&self, tr : u8) {

    }

    pub fn clean_line(&self, line : u16) {

    }

    pub fn clear(&mut self, color : Color) {

    }

    pub fn set_cursor(&mut self, x : u16, y : u16) -> Result<(), LcdError> {

        Ok(())
    }

    pub fn set_color_keying(&self, rgv_val : u32) {

    }

    pub fn reset_color_keying(&self) {

    }

    fn draw_char(&self, x : u16, y : u16, c : &u16) {

    }

    pub fn display_char(&self, line : u16, col : u16, ascii : char) {

    }

    pub fn display_string_line(&self, line : u16, s : &str) {

    }

    pub fn set_display_window(&self, x : u16, y : u16, w : u16, h : u16) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn disable_window_mode(&self) {

    }

    pub fn draw_line(&self, x : u16, y : u16, l : u16, dir : Direction) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_rect(&self, x : u16, y : u16, w : u16, h : u16) -> Result<(), LcdError> {

    }

    pub fn draw_circle(&self, x : u16, y : u16, r : u16) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_full_ellipse(&self, x : u16, y : u16, r1 : u16, r2 : u16) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_ellipse(&self, x : u16, y : u16, r1 : u16, r2 : u16) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_mono_picture(&self, pic_addr : &u32) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_bmp(&self, pic_addr : &u32) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_full_rect(&self, x : u16, y : u16, w : u16, h : u16) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_full_circle(&self, x : u16, y : u16, r : u16) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_uniline(&self, p1 : Point, p2 : Point) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_triangle(&self, p1 : Point, p2 : Point, p3 : Point) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_full_triangle(&self, p1 : Point, p2 : Point, p3 : Point) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_polyline_closed(&self, p_list : &[Point], closed : bool) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_polyline_relative_closed(&self, p_list : &[Point], closed : bool) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn draw_full_polyline(&self, p_list : &[Point]) -> Result<(), LcdError> {
        Ok(())
    }

    pub fn send_command(&self, cmd : Register) {

    }

    pub fn send_data(&self, val : u8) {

    }

    pub fn power_on(&self) {

    }

    pub fn display_on(&self) {

    }

    pub fn display_off(&self) {

    }

    fn configure_ctrl_lines() {

    }

    fn write_ctrl_line(p : CtrlLinePort, l : CtrlLine, state : bool) {

    }

    fn configure_spi() {

    }

    fn configure_alt_fn_gpios() {

    }

    fn put_pixel(x : u16, y : u16) -> Result<(), LcdError> {

    }
}
