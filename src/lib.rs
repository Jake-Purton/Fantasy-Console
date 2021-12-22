mod wasm4;
use wasm4::*;

pub struct Ball {
    pub ball_x: u8,
    pub ball_y: u8
}

static mut BALL: Ball = Ball {
    ball_x: 0,
    ball_y: 0
};

#[no_mangle]
fn start() {
    trace("I've been called");
    unsafe {
        *PALETTE = [0xff0000, 0x00ff00, 0x0000ff, 0x000000];
    }
}

#[no_mangle]
fn update() {
    let ball = unsafe { &mut BALL };

    ball.ball_y = 10;
}
