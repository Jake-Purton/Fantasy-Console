mod wasm4;
use wasm4::*;

#[no_mangle]
fn update () {
    rect(10, 10, 32, 32);
}
