mod wasm4;
use wasm4::*;

pub struct Ball1 {
    pub ball_x: i32,
    pub ball_y: i32,
    pub dx: i8,
    pub dy: i8

}pub struct Ball2 {
    pub ball_x: i32,
    pub ball_y: i32,
    pub dx: i8,
    pub dy: i8
}

// STATIC FOR BALL 1

static mut BALL1: Ball1 = Ball1 {
    ball_x: 44,
    ball_y: 139,
    dx: 3,
    dy: 2
};

// STATIC FOR BALL 2

static mut BALL2: Ball2 = Ball2 {
    ball_x: 79,
    ball_y: 92,
    dx: 1,
    dy: -3
};

#[no_mangle]
fn start() {
    unsafe { *DRAW_COLORS = 2 }
    unsafe {
        *PALETTE = [0x000000, 0xffffff, 0xeb6b6f, 0x7c3f58];
    }

    let _ball1 = unsafe { &mut BALL1 };
    let _ball2 = unsafe { &mut BALL2 };

}


#[no_mangle]
fn update() {

    // MOVES THE FIRST BALL

    let ball1 = unsafe { &mut BALL1 };

    ball1.ball_x += ball1.dx as i32;
    ball1.ball_y += ball1.dy as i32;

    if ball1.ball_x > 159 || ball1.ball_x < 1 {
        ball1.dx = ball1.dx * -1;
    }

    if ball1.ball_y > 159 || ball1.ball_y < 1 {
        ball1.dy = ball1.dy * -1;
    }

    // MOVES THE SECOND BALL
    
    let ball2 = unsafe { &mut BALL2 };

    ball2.ball_x += ball2.dx as i32;
    ball2.ball_y += ball2.dy as i32;

    if ball2.ball_x > 159 || ball2.ball_x < 1 {
        ball2.dx = ball2.dx * -1;
    }

    if ball2.ball_y > 159 || ball2.ball_y < 1 {
        ball2.dy = ball2.dy * -1;
    }

    // DRAWS A LINE BETWEEN THE TWO BALLS

    line(ball1.ball_x, ball1.ball_y, ball2.ball_x, ball2.ball_y)

}
