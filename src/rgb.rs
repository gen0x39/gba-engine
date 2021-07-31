pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait RGBDef {
    fn white() -> Self;
    fn black() -> Self;
    fn red() -> Self;
    fn green() -> Self;
    fn blue() -> Self;
    fn magenta() -> Self;
}

impl RGBDef for RGB {
    fn white() -> RGB {
        return RGB{ r:0x1F, g:0x1F, b:0x1F };
    }
    fn black() -> RGB {
        return RGB{ r:0x00, g:0x00, b:0x00 };
    }
    fn red() -> RGB {
        return RGB{ r:0x1F, g:0x00, b:0x0 };
    }
    fn green() -> RGB {
        return RGB{ r:0x00, g:0x1F, b:0x00 };
    }
    fn blue() -> RGB {
        return RGB{ r:0x00, g:0x00, b:0x1F };
    }
    fn magenta() -> RGB {
        return RGB{ r:0x1F, g:0x00, b:0x1F };
    }
}