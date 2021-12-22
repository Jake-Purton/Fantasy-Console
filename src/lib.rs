mod wasm4;
use wasm4::*;

pub struct Ball {
    pub ball_x: i32,
    pub ball_y: i32,
    pub dx: i8,
    pub dy: i8

}

impl Ball {
    pub fn update(&mut self) {
        self.ball_x += self.dx as i32;
        self.ball_y += self.dy as i32;

        if self.ball_x > 159 || self.ball_x < 1 {
            self.dx = self.dx * -1;
        }

        if self.ball_y > 159 || self.ball_y < 1 {
            self.dy = self.dy * -1;
        }
    }
}

// STATIC FOR BALL 1

static mut BALL1: Ball = Ball {
    ball_x: 44,
    ball_y: 139,
    dx: 3,
    dy: 2
};

// STATIC FOR BALL 2

static mut BALL2: Ball = Ball {
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

}


#[no_mangle]
fn update() {

    // MOVES THE FIRST BALL

    let ball1 = unsafe { &mut BALL1 };
    let ball2 = unsafe { &mut BALL2 };

    ball1.update();
    ball2.update();

    // DRAWS A LINE BETWEEN THE TWO BALLS

    line(ball1.ball_x, ball1.ball_y, ball2.ball_x, ball2.ball_y)

}
