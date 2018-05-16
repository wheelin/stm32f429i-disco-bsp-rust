use stm32f429::{SPI1, SPI2, SPI3, SPI4, SPI5};

pub enum DataFrameFormat {
    Frame8Bits = 0,
    Frame16Bits = 1,
}

pub enum ClkPhaCfg {
    CphaFirst,
    CphaSecond,
}

pub enum ClkPolCfg {
    CpolLow,
    CpolHigh,
}

pub enum BaudRateDivider {
    DivBy2 = 0,
    DivBy2 = 1,
    DivBy2 = 2,
    DivBy2 = 3,
    DivBy2 = 4,
    DivBy2 = 5,
    DivBy2 = 6,
    DivBy2 = 7,
}

pub struct SpiBuilder {
    bidimode_en         : bool,
    bidioe_en           : bool,
    crc_en              : bool,
    crc_next            : bool,
    data_frame          : DataFrameFormat,
    rx_only             : bool,
    sw_slave_mgmt       : bool,
    lsb_first           : bool,
    baudrate_freq_div   : u8,
    clock_polarity      : ClkPolCfg,
    clock_edge          : ClkPhaCfg,
    master              : bool,
    spi_periph          : Option<stm32f429::spi1::RegisterBlock>,
}

impl SpiBuilder {
    pub fn new() -> SpiBuilder {
        SpiBuilder {
            bidimode_en         : false,
            bidioe_en           : false,
            crc_en              : false,
            data_frame          : DataFrameFormat::Frame8Bits,
            rx_only             : false,
            sw_slave_mgmt       : false,
            lsb_first           : false,
            baudrate_freq_div   : 2,
            clock_polarity      : ClkPolCfg::CpolHigh,
            clock_edge          : ClkPhaCfg::CphaFirst,
            master              : true,
            spi_periph          : None,
        }
    }

    pub fn bidimode(self, en : bool) -> SpiBuilder {
        self.bidimode_en = en;
        self
    }

    pub fn bidioe(self, en : bool) -> SpiBuilder {
        self.bidioe_en = en;
        self
    }

    pub fn crc(self, en : bool) -> SpiBuilder {
        self.crc_en = en;
        self
    }

    pub fn data_frame_length(self, dff : DataFrameFormat) -> SpiBuilder {
        self.data_frame = dff;
        self
    }

    pub fn rx_only(self, en : bool) -> SpiBuilder {
        self.rx_only = en;
        self
    }

    pub fn sw_slave_mgmt(self, en : bool) -> SpiBuilder {
        self.sw_slave_mgmt = en;
        self
    }

    pub fn lsb_first(self, en : bool) -> SpiBuilder {
        self.lsb_first = en;
        self
    }

    pub fn baudrate_freq_div(self, div : BaudRateDivider) -> SpiBuilder {
        self.baudrate_freq_div = div as u8;
        self
    }

    pub fn clock_polarity(self, cpol : ClkPolCfg) -> SpiBuilder {
        self.clock_polarity = cpol;
        self
    }

    pub fn clock_edge(self, cpha : ClkPhaCfg) -> SpiBuilder {
        self.clock_edge = cpha;
        self
    }

    pub fn master(self, en : bool) -> SpiBuilder {
        self.master = en;
        self
    }

    pub fn spi_periph(self, sp : stm32f429::spi1::RegisterBlock) -> SpiBuilder {
        self.spi_periph = Some(sp);
        self
    }

    pub fn configure(self) -> Spi {
        Spi {self}
    }
}

pub struct Spi {
    spi_cfg : SpiBuilder,
}

impl Spi {
    pub fn read(&self, reg : u8, buf : &mut u16) -> Result<(), ()> {
        Ok(())
    }

    // write a complete buffer to the peripheral. Because some peripherals need a 16 bits wide
    // frame, we directly use a 16 bits buffer.
    pub fn write(&self, reg : u8, data : &[u16]) -> Result<(), ()>{
        Ok(())
    }

    // If another peripheral needs another configuration for the same spi, a call to this
    // method reconfigures the corresponding spi to the last used parameters.
    pub fn reconfigure(&self) {

    }
}
