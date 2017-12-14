use super::fonts;
use misc;
use stm32f429::{SPI5, RCC, GPIOA, GPIOB, GPIOC, GPIOD, GPIOF, GPIOG};

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
        configure_ctrl_lines();

        self.chip_select()
    }

    pub fn init_layers(&mut self) {

    }

    fn chip_select(&self, en : bool) {

    }

    pub fn set_layer(&mut self, l : Layer) {

    }

    pub fn set_colors(&mut self, tc : Color, bc : Color) {

    }

    pub fn get_colors(&self) -> (Color, Color) {

        (Color::Black, Color::White)
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