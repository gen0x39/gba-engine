use crate::rgb::RGB;

pub trait GBAColor {
    fn convert_u16_color(&self) -> u16;
}

// Color Format : 16bit (? BBBBB GGGGG RRRRR)
impl GBAColor for RGB {
    fn convert_u16_color(&self) -> u16{
        return  (((self.b & 0x1F) as u16) << 10) +
                (((self.g & 0x1F) as u16) << 5) +
                (self.r & 0x1F) as u16;
    }
}