mod wasm4;
use wasm4::*;

#[no_mangle]
fn start() {
    trace("I've been called");
    unsafe {
        *PALETTE = [0xff0000, 0x00ff00, 0x0000ff, 0x000000];
    }
}

#[no_mangle]
fn update() {
    let mut colour = 1;
    for n in 0..160 {
        for x in 0..160 {
            colour = colour + 1;
            if colour > 3 {
                colour = 1
            }
            unsafe { *DRAW_COLORS = colour }
            rect(n, x, 1, 1);
        }
    }
}
