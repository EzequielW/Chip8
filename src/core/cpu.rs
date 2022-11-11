use super::{renderer::Renderer, speaker::Speaker, keyboard::Keyboard};

pub(crate) struct CPU{
    renderer: Renderer,
    keyboard: Keyboard,
    speaker: Speaker,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    delayTimer: usize,
    soundTimer: usize,
    pc: u8,
    stack: Vec<u8>,
    paused: bool,
    speed: usize
}

impl CPU {
    pub fn new(renderer: Renderer, keyboard: Keyboard, speaker: Speaker) -> CPU {
        let memory: [u8; 4096] = [0; 4096];
        let v: [u8; 16] = [0; 16];
        let i: u16 = 0;
        let delayTimer: usize = 0;
        let soundTimer: usize = 0;
        let pc: u8 = 0;
        let stack = vec![0;0];
        let paused = false;
        let speed: usize = 10;

        CPU {
            renderer,
            keyboard,
            speaker,
            memory,
            v,
            i,
            delayTimer,
            soundTimer,
            pc,
            stack,
            paused,
            speed
        }
    }
}