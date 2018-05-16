use stm32f429::{
    LTDC,
};

pub enum Layer {
    First,
    Second,
}

pub fn set_layer_position(l : Layer, xpos : u16, ypos : u16) {
    let ltdc = unsafe{&*LTDC::ptr()};

    unsafe {
        match l {
            Layer::First => {
                ltdc.l1whpcr.modify(|r, w| {
                    w.bits(r.bits() & !0xFFFF0FFF)
                });
                ltdc.l1wvpcr.modify(|r, w| {
                    w.bits(r.bits() & !0xFFFF0FFF)
                });
            },
            Layer::Second => {
                ltdc.l2whpcr.modify(|r, w| {
                    w.bits(r.bits() & !0xFFFF0FFF)
                });
                ltdc.l2wvpcr.modify(|r, w| {
                    w.bits(r.bits() & !0xFFFF0FFF)
                });
            }
        }
    }

    let tmpreg = match l {
        Layer::First => ltdc.l1pfcr.read().bits(),
        Layer::Second => ltdc.l2pfcr.read().bits(),
    };
    let hstart = (tmpreg >> 16) + 1 + xpos as u32;
    let vstart = (tmpreg & 0xFFFF) + 1 + ypos as u32;

    let tmpreg = match l {
        Layer::First => {
            ltdc.l1pfcr.read().bits()
        },
        Layer::Second => {
            ltdc.l2pfcr.read().bits()
        }
    };


    let mut tmp = match tmpreg {
        0 => 4,     // ARGB8888
        1 => 3,     // RGB888
        2...4 => 2,  // ARGB4444, RGB565, ARGB1555,
        7 => 2,     // AL88
        _ => 1,     // other
    };

    let tmpreg = match l {
        Layer::First => {
            ltdc.l1cfblr.read().bits()
        },
        Layer::Second => {
            ltdc.l2cfblr.read().bits()
        }
    };

    let hstop = (((tmpreg & 0x7FF) - 3)/tmp) + hstart - 1;

    let tmpreg = match l {
        Layer::First => {
            ltdc.l1cfblnr.read().bits()
        },
        Layer::Second => {
            ltdc.l2cfblnr.read().bits()
        }
    };

    let vstop = (tmpreg & 0x7FF) + vstart - 1;

    unsafe {
        match l {
            Layer::First => {
                ltdc.l1whpcr.write(|w| w.bits(hstart | (hstop << 16)));
                ltdc.l1wvpcr.write(|w| w.bits(vstart | (vstop << 16)));
            },
            Layer::Second => {
                ltdc.l2whpcr.write(|w| w.bits(hstart | (hstop << 16)));
                ltdc.l2wvpcr.write(|w| w.bits(vstart | (vstop << 16)));
            }
        }
    }
}

pub fn set_layer_size(ln : u8, xsize : u16, ysize : u16) {

}

pub enum LayerReloadMethod {
    ImReload,
    VbReload,
}

pub fn reload_configuration(rm : LayerReloadMethod) {

}
