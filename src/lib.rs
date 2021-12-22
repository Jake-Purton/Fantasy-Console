mod wasm4;
use wasm4::*;

pub struct Ball {
    pub ball_x: u8,
    pub ball_y: u8,
    pub dx: i8,
    pub dy: i8
}

static mut BALL: Ball = Ball {
    ball_x: 0,
    ball_y: 0,
    dx: 1,
    dy: 1
};

#[no_mangle]
fn start() {
    unsafe {
        *PALETTE = [0xfff6d3, 0xf9a875, 0xeb6b6f, 0x7c3f58];
    }
}

#[no_mangle]
fn update() {
    let ball = unsafe { &mut BALL };

    ball.ball_x = (ball.ball_x as i8 + ball.dx) as u8;
    ball.ball_y = (ball.ball_y as i8 + ball.dy) as u8;

    rect(ball.ball_x as i32, ball.ball_y as i32, 1, 1);

    if ball.ball_x > 159 || ball.ball_x < 1 {
        ball.dx = ball.dx * -1;
    }
}
