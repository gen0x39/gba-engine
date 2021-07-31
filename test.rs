fn main() {
    const MEM_PAL:      u32 = 0x05000000;   // 1KB カラーパレット
    const p: *const u16 = (MEM_PAL | 0x00000200) as *const u16;
    println!("{:p}", p);
    unsafe {
        for i in 0 .. 8 {
            println!("i = {}, p:{:p} ", i, p.offset(i));
        }
    }
}