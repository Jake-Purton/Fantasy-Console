mod wasm4;
use wasm4::*;
use quad_rand as qrand;

pub struct Ball {
    pub ball_x: i32,
    pub ball_y: i32,
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

    let ball = unsafe { &mut BALL };

    ball.ball_x = qrand::gen_range(0, 159);
    ball.ball_y = qrand::gen_range(0, 159);
}

#[no_mangle]
fn update() {

    let ball = unsafe { &mut BALL };

    ball.ball_x += ball.dx as i32;
    ball.ball_y += ball.dy as i32;

    rect(ball.ball_x as i32, ball.ball_y as i32, 1, 1);

    if ball.ball_x > 159 || ball.ball_x < 1 {
        ball.dx = ball.dx * -1;
        trace("here")
    }

    if ball.ball_y > 159 || ball.ball_y < 1 {
        ball.dy = ball.dy * -1;
        trace("here")
    }
}
