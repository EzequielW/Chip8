use pixels::Error;
use winit::{
    event_loop::{ControlFlow, EventLoop}, event::VirtualKeyCode,
};
use winit_input_helper::WinitInputHelper;

use super::renderer::Renderer;

pub(crate) fn init() -> Result<(), Error>{
    env_logger::init();
    let eventLoop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut winRenderer = Renderer::new(10, &eventLoop)?;
    let x: usize = 0;
    let mut y: usize = 0;

    eventLoop.run(move |event, _, controlFlow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *controlFlow = ControlFlow::Exit;
                return;
            }

            if input.key_held(VirtualKeyCode::P) {
                winRenderer.setPixel(x, y);
                y += 1;
            }

            if winRenderer.render()
            {
                *controlFlow = ControlFlow::Exit;
                return;
            }
        }
    });
}