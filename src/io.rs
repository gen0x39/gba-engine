const MEM_IO:       u32 = 0x04000000;

// Keypad Input 
// Reference : http://www.akkit.org/info/gbatek.htm#gbakeypadinput
const KEY_A:        u32 = 0x00000001; // 0  (0 = Pressed, 1 = Released)
const KEY_B:        u32 = 0x00000002; // 1
const KEY_SELECT:   u32 = 0x00000004; // 2
const KEY_START:    u32 = 0x00000008; // 3
const KEY_RIGHT:    u32 = 0x00000010; // 4
const KEY_LEFT:     u32 = 0x00000020; // 5
const KEY_UP:       u32 = 0x00000040; // 6
const KEY_DOWN:     u32 = 0x00000080; // 7
const KEY_R:        u32 = 0x00000100; // 8
const KEY_L:        u32 = 0x00000200; // 9 (10 ~ 15 Not Used)
const KEY_ANY:      u32 = 0x000003FF; // mask用 

const REG_KEY_INPUT: *const u32 = (MEM_IO + 0x00000130) as *const u32;


pub fn key_a_is_pressed() -> bool {
    let mut key_state: u32 = 0;
    unsafe {
        key_state = (!(*REG_KEY_INPUT)) & KEY_ANY;
    }
    if ((key_state & KEY_A) > 0) {
        return true;
    }
    return false;
}
pub fn key_right_is_pressed() -> bool {
    let mut key_state: u32 = 0;
    unsafe {
        key_state = (!(*REG_KEY_INPUT)) & KEY_ANY;
    }
    if ((key_state & KEY_RIGHT) > 0) {
        return true;
    }
    return false;
}

pub fn key_left_is_pressed() -> bool {
    let mut key_state: u32 = 0;
    unsafe {
        key_state = (!(*REG_KEY_INPUT)) & KEY_ANY;
    }
    if ((key_state & KEY_LEFT) > 0) {
        return true;
    }
    return false;
}

pub fn key_up_is_pressed() -> bool {
    let mut key_state: u32 = 0;
    unsafe {
        key_state = (!(*REG_KEY_INPUT)) & KEY_ANY;
    }
    if ((key_state & KEY_UP) > 0) {
        return true;
    }
    return false;
}

pub fn key_down_is_pressed() -> bool {
    let mut key_state: u32 = 0;
    unsafe {
        key_state = (!(*REG_KEY_INPUT)) & KEY_ANY;
    }
    if ((key_state & KEY_DOWN) > 0) {
        return true;
    }
    return false;
}
