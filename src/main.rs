
#![no_std]
#![feature(start)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod rgb;
mod gba_color;
mod graphics;
mod font;
mod font_def;
mod io;

use rgb::RGB;
use rgb::RGBDef;
use gba_color::GBAColor;
use graphics::Graphics;
use crate::io::key_up_is_pressed;
use crate::io::key_a_is_pressed;


struct object_attributes {
    attribute_zero: u16,
    attribute_one:  u16,
    attribute_two:  u16,
    dummy:          u16,
}



const MEM_IO:       u32 = 0x04000000;   // I/Oレジスタ
const MEM_PAL:      u32 = 0x05000000;   // 1KB カラーパレット
const MEM_VRAM:     u32 = 0x06000000;   // 96KB VRAM (ビデオRAM)
const MEM_OAM:      u32 = 0x07000000;   // 1KB OAM RAM (オブジェクト属性メモリ)

const REG_DISPLAY       : *const u32 = (MEM_IO) as *const u32;
const REG_DISPLAY_VCOUNT: *const u32 = (MEM_IO | 0x00000006) as *const u32;

const oam_memory: *mut object_attributes = MEM_OAM as *mut object_attributes;
const tile_memory: *mut u16 = MEM_VRAM as *mut u16;
const object_palette_memory: *mut u16 = (MEM_PAL | 0x00000200) as *mut u16;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    init_graphic();


    // Plot RGB dot
    
    let red: RGB = RGB::red();
    let mut offset: u32 = ((80 * 240) + 115) as u32;
    let mut vram: *mut u16 = (MEM_VRAM + (offset * 2)) as *mut u16;
    /*
    unsafe {
        *vram = red.convert_u16_color();
    }*/

    let graphics: Graphics = Graphics::new();
    graphics.draw_string("Hello, World!", 10, 10, &RGB::white());
    graphics.draw_box(10, 30, 30, 30, &RGB::blue());
    
    let green: RGB = RGB::green();
    let blue: RGB = RGB::blue();
    let white: RGB = RGB::white();
    let mut x: u16 = 100;
    let mut y: u16 = 100;
    offset = ((y * 240) + x) as u32;
    vram = (MEM_VRAM + (offset * 2)) as *mut u16;
    /*
    unsafe {
        *vram = white.convert_u16_color();
    }*/

    //let paddle_tile_memory: *mut u16 = tile_memory[4][1] as *mut u16;

    // カラーパレットメモリーの最初の16色パレット(インデックスは0)に、
    // スプライトで使うカラーパレットを書き込みます
    unsafe{
        *object_palette_memory.offset(1) = RGB::white().convert_u16_color();
        *object_palette_memory.offset(2) = RGB::magenta().convert_u16_color();
    }

    // オブジェクト属性をOAMメモリに書き込むことで、スプライトを生成します
    let paddle_attributes = object_attributes {
        attribute_zero: 0x8000,
        attribute_one: 0x4000,
        attribute_two: 1,
        dummy: 0
    };

    let ball_attributes = object_attributes {
        attribute_zero: 0,
        attribute_one: 0,
        attribute_two: 5,
        dummy: 0
    };

    /*
    unsafe {
        *oam_memory.offset(0) = paddle_attributes;
        *oam_memory.offset(1) = ball_attributes;
    }
    */

    // IO
    let mut key_state: u32 = 0;
    loop {
        unsafe {
            wait_for_vsync();

            // A Buttonが押されたとき
            if (key_a_is_pressed()) {
                graphics.draw_string("A Button is Pressed!", 10, 70, &RGB::white());
            }
            else {
                //*vram = blue.convert_u16_color();
            }
        }
    }
    0
}

fn init_graphic() {
    let video_mode: *mut u8 = MEM_IO as *mut u8;
    let bg: *mut u8 = (MEM_IO + 1 ) as *mut u8;
    unsafe {
        *video_mode = 0x03; // mode3
        *bg = 0x04; // BG2
    }
}

fn wait_for_vsync() {
    unsafe{
        while (*REG_DISPLAY_VCOUNT >= 160){;}
        while (*REG_DISPLAY_VCOUNT < 160) {;}
    }
}