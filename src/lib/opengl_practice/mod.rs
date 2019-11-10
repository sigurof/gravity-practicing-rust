mod opengl_practice;
use kiss3d::context::Context;
mod generate_trail;
use opengl_practice::{gen_window, render_loop};
mod triangle;

pub fn _main() {
    let ctxt = Context::get();
    let (window, events_loop) = gen_window();
    render_loop(window, events_loop);
}
