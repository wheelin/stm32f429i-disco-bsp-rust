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
    OutOfScreen,
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

enum CtrlLine {
    Ncs,
    Nwr,
}

pub struct Lcd {
    current_font         : &'static fonts::Font,
    current_text_color   : Color,
    current_back_color   : Color,
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

        spi.cr1.modify(|_, w| w.spe().bit(false));
        spi.cr1.reset();
        spi.cr2.reset();

        rcc.apb2enr.modify(|_, w| w.spi5enr().bit(false));

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

        Lcd::configure_ctrl_lines();

        self.chip_select(true);
        self.chip_select(false);

        Lcd::configure_spi();

        self.power_on();

        rcc.apb2enr.modify(|_, w| w.ltdcen().bit(true));
        rcc.ahb1enr.modify(|_, w| w.dma2den().bit(true));

        Lcd::configure_alt_fn_gpios();

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
            w.bits(0)
        });

        ltdc.twcr.modify(|_, w| unsafe{
            w.totalw().bits(279)    // hsync width + hbp + active width + hfp - 1
             .totalh().bits(327)    // vsync height + vbp + active height + vfp - 1
        });

        ltdc.awcr.modify(|_, w| unsafe{
            w.aav().bits(323)       // vsync height + vhp + active heigh - 1
             .aah().bits(269)       // hsync width + hbp + active width - 1
        });

        ltdc.bpcr.modify(|_, w| unsafe {
            w.ahbp().bits(29)       // hsync width + hbp - 1
             .avbp().bits(3)        // vsync height + vbp - 1
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
        self.current_text_color = tc;
    }

    pub fn set_back_color(&mut self, bc : Color) {
        self.current_back_color = bc;
    }

    pub fn set_transparency(&self, tr : u8) {
        let ltdc = unsafe {&*LTDC.get()};
        match self.current_layer {
            Layer::Background => {
                ltdc.l1cacr.modify(|_, w| unsafe{
                    w.consta().bits(tr)
                });
            },
            Layer::Foreground => {
                ltdc.l2cacr.modify(|_, w| unsafe{
                    w.consta().bits(tr)
                });
            },
        };
        ltdc.srcr.modify(|_, w| w.imr().bit(true));
    }

    pub fn get_font(&self) -> &'static fonts::Font {
        self.current_font
    }

    pub fn clear_line(&self, line : u16) {
        let mut ref_col = 0;
        while (ref_col < LCD_WIDTH) &&
                (((ref_col + self.current_font.width) & 0xFFFF) >= self.current_font.width) {
            self.display_char(line, ref_col, ' ');
            ref_col += self.current_font.width;
        }
    }

    pub fn clear(&mut self, color : Color) {
        for i in 0..LCD_BUFFER_OFFSET {
            unsafe {
                let mem_loc = (self.current_frame_buffer + (2 * i)) as *mut u16;
                *mem_loc = color as u16;
            }
        }
    }

    pub fn set_cursor(&mut self, x : u16, y : u16) -> u32 {
        self.current_frame_buffer + 2*(x as u32 + (LCD_WIDTH as u32 * y as u32))
    }

    pub fn set_color_keying(&self, rgb_val : u32) {
        let ltdc = unsafe{&*LTDC.get()};
        match self.current_layer {
            Layer::Background => {
                ltdc.l1ckcr.modify(|_, w| unsafe {
                    w.ckred().bits(((rgb_val >> 16) & 0xFF) as u8)
                     .ckgreen().bits(((rgb_val >> 8) & 0xFF) as u8)
                     .ckblue().bits((rgb_val & 0xFF) as u8)
                });
                ltdc.l1cr.modify(|_, w| w.colken().bit(true));
            },
            Layer::Foreground => {
                ltdc.l2ckcr.modify(|_, w| unsafe {
                    w.ckred().bits(((rgb_val >> 16) & 0xFF) as u16)
                     .ckgreen().bits(((rgb_val >> 8) & 0xFF) as u8)
                     .ckblue().bits((rgb_val & 0xFF) as u8)
                });
                ltdc.l2cr.modify(|_, w| w.colken().bit(true));
            },
        };
        ltdc.srcr.modify(|_, w| w.imr().bit(true));
    }

    pub fn reset_color_keying(&self) {
        let ltdc = unsafe{&*LTDC.get()};
        match self.current_layer {
            Layer::Background => {
                ltdc.l1ckcr.modify(|_, w| unsafe {
                    w.ckred().bits(0)
                     .ckgreen().bits(0)
                     .ckblue().bits(0)
                });
                ltdc.l1cr.modify(|_, w| w.colken().bit(true));
            },
            Layer::Foreground => {
                ltdc.l2ckcr.modify(|_, w| unsafe {
                    w.ckred().bits(0)
                     .ckgreen().bits(0)
                     .ckblue().bits(0)
                });
                ltdc.l2cr.modify(|_, w| w.colken().bit(true));
            },
        };
        ltdc.srcr.modify(|_, w| w.imr().bit(true));
    }

    fn draw_char(&self, x : u16, y : u16, c : &[u16]) {
        let x = x * LCD_WIDTH * 2;
        let mut x_addr = y as u32;
        for index in 0..(self.current_font.height) {
            for cntr in 0..(self.current_font.width) {
                let mem_loc = (self.current_frame_buffer + (2 * x_addr) + x as u32) as *mut u16;
                if (((c[index as usize] & ((0x80 << ((self.current_font.width / 12) * 8)) >> cntr)) == 0x00) &&
                        self.current_font.width <= 12) ||
                        (((c[index as usize] & 0x01 << cntr)) == 0x00) &&
                        self.current_font.width > 12 {
                    unsafe {
                        *mem_loc = self.current_back_color as u16;
                    }
                } else {
                    unsafe {
                        *mem_loc = self.current_text_color as u16;
                    }
                }
            }
            x_addr += LCD_WIDTH as u32 - self.current_font.width as u32;
        }
    }

    pub fn display_char(&self, line : u16, col : u16, ascii : char) {
        let pos = (ascii as u8) - 32;
        self.draw_char(
            line,
            col,
            &self.current_font.table[
                (pos as u16 * self.current_font.height) as usize ..
                    (pos as u16 * self.current_font.height + self.current_font.height) as usize
            ]
        );
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

        Ok(())
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
        let rcc = unsafe{&*RCC.get()};
        let pc  = unsafe{&*GPIOC.get()};
        let pd  = unsafe{&*GPIOD.get()};

        rcc.ahb1enr.modify(|_, w|
            w.gpiocen().bit(true)
             .gpioden().bit(true)
        );

        pc.moder.modify(|_, w| unsafe{
            w.moder2().bits(1)
        });
        pd.moder.modify(|_, w| unsafe{
            w.moder13().bits(1)
        });

        Lcd::write_ctrl_line(CtrlLine::Ncs, true);
    }

    fn write_ctrl_line(l : CtrlLine, state : bool) {
        let pc = unsafe{&*GPIOC.get()};
        let pd = unsafe{&*GPIOD.get()};
        match l {
            CtrlLine::Ncs => pc.odr.modify(|_, w| w.odr2().bit(state)),
            CtrlLine::Nwr => pd.odr.modify(|_, w| w.odr13().bit(state)),
        };
    }

    fn configure_spi() {
        let rcc = unsafe{&*RCC.get()};
        let pf = unsafe{&*GPIOF.get()};
        let spi = unsafe{&*SPI5.get()};

        rcc.ahb1enr.modify(|_, w| w.gpiofen().bit(true));
        rcc.apb2enr.modify(|_, w| w.spi5enr().bit(true));

        // configure as alt fn
        pf.moder.modify(|_, w| unsafe {
            w.moder7().bits(2)
             .moder8().bits(2)
             .moder9().bits(2)
        });

        // medium high speed
        pf.ospeedr.modify(|_, w| unsafe {
            w.ospeedr7().bits(2)
             .ospeedr8().bits(2)
             .ospeedr9().bits(2)
        });

        pf.afrl.modify(|_, w| unsafe {
            w.afrl7().bits(5)
        });
        pf.afrh.modify(|_, w| unsafe {
            w.afrh8().bits(5)
             .afrh9().bits(5)
        });

        // do not reconfigure if already enabled
        if spi.cr1.read().spe().bit() == false {
            spi.cr1.modify(|_, w| unsafe {
                w.bidimode().bit(false)     // full duplex
                 .crcen().bit(false)        // disable crc
                 .dff().bit(false)          // 8 bits frame len
                 .rxonly().bit(false)       // rxonly disabled
                 .lsbfirst().bit(false)     // msb first
                 .br().bits(0b011)          // baudrate / 16
                 .mstr().bit(true)          // configured as master
                 .spe().bit(true)           // enable spi
            });
        }
    }

    fn configure_alt_fn_gpios() {
        let pa = unsafe {&*GPIOA.get()};
        let pb = unsafe {&*GPIOB.get()};
        let pc = unsafe {&*GPIOC.get()};
        let pd = unsafe {&*GPIOD.get()};
        let pf = unsafe {&*GPIOF.get()};
        let pg = unsafe {&*GPIOG.get()};

        let rcc = unsafe{&*RCC.get()};

        // GPIOA ###################################################
        pa.moder.modify(|_, w| unsafe {
            w.moder3().bits(0b10)
             .moder4().bits(0b10)
             .moder6().bits(0b10)
             .moder11().bits(0b10)
             .moder12().bits(0b10)
        });
        pa.ospeedr.modify(|_, w| unsafe {
            w.ospeedr3().bits(0b10)
             .ospeedr4().bits(0b10)
             .ospeedr6().bits(0b10)
             .ospeedr11().bits(0b10)
             .ospeedr12().bits(0b10)
        });

        pa.afrl.modify(|_, w| unsafe {
            w.afrl3().bits(0x0E)
             .afrl4().bits(0x0E)
             .afrl6().bits(0x0E)
        });
        pa.afrh.modify(|_, w| unsafe {
            w.afrh11().bits(0x0E)
             .afrh12().bits(0x0E)
        });

        // GPIOB ###################################################
        pb.moder.modify(|_, w| unsafe {
            w.moder0().bits(0b10)
             .moder1().bits(0b10)
             .moder8().bits(0b10)
             .moder9().bits(0b10)
             .moder10().bits(0b10)
             .moder11().bits(0b10)
        });
        pb.ospeedr.modify(|_, w| unsafe {
            w.ospeedr0().bits(0b10)
             .ospeedr1().bits(0b10)
             .ospeedr8().bits(0b10)
             .ospeedr9().bits(0b10)
             .ospeedr10().bits(0b10)
             .ospeedr11().bits(0b10)
        });

        pb.afrl.modify(|_, w| unsafe {
            w.afrl0().bits(0x09)
             .afrl1().bits(0x09)
        });
        pb.afrh.modify(|_, w| unsafe {
            w.afrh8().bits(0x0E)
             .afrh9().bits(0x0E)
             .afrh10().bits(0x0E)
             .afrh11().bits(0x0E)
        });

        // GPIOC ###################################################
        pc.moder.modify(|_, w| unsafe {
            w.moder6().bits(0b10)
             .moder7().bits(0b10)
             .moder10().bits(0b10)
        });
        pc.ospeedr.modify(|_, w| unsafe {
            w.ospeedr6().bits(0b10)
             .ospeedr7().bits(0b10)
             .ospeedr10().bits(0b10)
        });

        pc.afrl.modify(|_, w| unsafe {
            w.afrl6().bits(0x0E)
             .afrl7().bits(0x0E)
        });
        pc.afrh.modify(|_, w| unsafe {
            w.afrh10().bits(0x0E)
        });

        // GPIOD ###################################################
        pd.moder.modify(|_, w| unsafe {
            w.moder3().bits(0b10)
             .moder6().bits(0b10)
        });
        pd.ospeedr.modify(|_, w| unsafe {
            w.ospeedr3().bits(0b10)
             .ospeedr6().bits(0b10)
        });

        pd.afrl.modify(|_, w| unsafe {
            w.afrl3().bits(0x0E)
             .afrl6().bits(0x0E)
        });

        // GPIOF ###################################################
        pf.moder.modify(|_, w| unsafe {
            w.moder10().bits(0b10)
        });
        pf.ospeedr.modify(|_, w| unsafe {
            w.ospeedr10().bits(0b10)
        });

        pf.afrh.modify(|_, w| unsafe {
            w.afrh10().bits(0x0E)
        });

        // GPIOG ###################################################
        pg.moder.modify(|_, w| unsafe {
            w.moder6().bits(0b10)
             .moder7().bits(0b10)
             .moder10().bits(0b10)
             .moder11().bits(0b10)
             .moder12().bits(0b10)
        });
        pg.ospeedr.modify(|_, w| unsafe {
            w.ospeedr6().bits(0b10)
             .ospeedr7().bits(0b10)
             .ospeedr10().bits(0b10)
             .ospeedr11().bits(0b10)
             .ospeedr12().bits(0b10)
        });

        pg.afrl.modify(|_, w| unsafe {
            w.afrl6().bits(0x0E)
             .afrl7().bits(0x0E)
        });
        pg.afrh.modify(|_, w| unsafe {
            w.afrh10().bits(0x09)
             .afrh11().bits(0x0E)
             .afrh12().bits(0x09)
        });

    }

    fn put_pixel(&self, x : u16, y : u16) -> Result<(), LcdError> {
        if x > 239 || y > 319 {
            return Err(LcdError::OutOfScreen);
        }
        self.draw_line(x, y, 1, Direction::Horizontal);
        Ok(())
    }
}
