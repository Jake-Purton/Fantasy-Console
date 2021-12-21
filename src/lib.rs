mod wasm4;
use wasm4::*;

#[no_mangle]
fn start () {
    trace("I've been called");
    unsafe {
        *PALETTE = [
            0xd2e7af,
            0x5aded9,
            0x2979b6,
            0x081935,
        ];
    }
}

#[no_mangle]
fn update () {
    unsafe { *DRAW_COLORS = 2 }
    rect(10, 10, 32, 32);
    unsafe { *DRAW_COLORS = 3 }
    rect(52, 10, 32, 32);
    unsafe { *DRAW_COLORS = 4 }
    rect(94, 10, 32, 32);

    rect(100,100,1,1)
}
