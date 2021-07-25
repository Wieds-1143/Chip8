use rand::{thread_rng, Rng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::Read;

#[allow(unused)]
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    CLS,
    RET,
    SYS,
    JP_A,
    CALL,
    SE_VB,
    SNE_VB,
    SE_VV,
    LD_VB,
    ADD_VB,
    LD_VV,
    OR,
    AND,
    XOR,
    ADD_VV,
    SUB,
    SHR,
    SUBN,
    SHL,
    SNE_VV,
    LD_IA,
    JP_VA,
    RND,
    DRW,
    SKP,
    SKNP,
    LD_VDT,
    LD_VK,
    LD_DTV,
    LD_STV,
    ADD_IV,
    LD_FV,
    LD_BV,
    LD_IV,
    LD_VI,
}

#[derive(Debug)]
pub struct Chip8 {
    // 0x000-0x1ff chip 8 interperter
    // 0x050-0x0a0 built in pixel font set
    // 0x200-0xfff program ROM and work RAM
    memory: [u8; 4096],

    //gerneral purpose registers
    pub v_register: [u8; 16],

    //index register
    #[allow(non_snake_case)]
    I: u16,

    //program counter
    pc: u16,

    //graphics
    pub gfx: [u8; 64 * 32],

    //timers that count down to zero once per second when
    //greater then zero
    delay_timer: u8,
    sound_timer: u8,

    //stack to store the stack pointer before a jump
    stack: [u16; 16],
    sp: u16,

    //hex based keypad 0 - f
    pub key: [bool; 16],
}

const INIT_MEMORY: [u8; 4096] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 144, 144, 144, 240, 32, 96, 32, 32, 112,
    240, 16, 240, 128, 240, 240, 16, 240, 16, 240, 144, 144, 240, 16, 16, 240, 128, 240, 16, 240,
    240, 128, 240, 144, 240, 240, 16, 32, 64, 64, 240, 144, 240, 144, 240, 240, 144, 240, 16, 240,
    240, 144, 240, 144, 144, 224, 144, 224, 144, 224, 240, 128, 128, 128, 240, 224, 144, 144, 144,
    224, 240, 128, 240, 128, 240, 240, 128, 240, 128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

//split

impl Chip8 {
    pub fn init() -> Chip8 {
        Chip8 {
            memory: INIT_MEMORY,
            v_register: [0; 16],
            //v0  : 0,
            //v1  : 0,
            //v2  : 0,
            //v3  : 0,
            //v4  : 0,
            //v5  : 0,
            //v6  : 0,
            //v7  : 0,
            //v8  : 0,
            //v9  : 0,
            //v10 : 0,
            //v11 : 0,
            //v12 : 0,
            //v13 : 0,
            //v14 : 0,
            //v15 : 0,
            I: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            gfx: [0u8; 64 * 32],
            delay_timer: 0u8,
            sound_timer: 0u8,
            key: [false; 16],
        }
    }

    pub fn load_game(&mut self, path: String) {
        let mut file = File::open(path).unwrap();
        let mut buf = [0u8; 0xfff - 0x200];
        file.read(&mut buf).unwrap();
        let mut i = 0x200;
        for byte in &buf {
            self.memory[i] = *byte;
            i += 1;
        }
    }

    pub fn check_key_state(&mut self, user_event: sdl2::event::Event) {
        // this will handle key input. maybe add an exit to it that might need to be somewhere else though
        match user_event {
            // can be refactored useing whats in the match above this in the main code
            Event::KeyDown {
                keycode: Some(Keycode::Num1),
                ..
            } => self.key[0x1] = true, //1

            Event::KeyUp {
                keycode: Some(Keycode::Num1),
                ..
            } => self.key[0x1] = false, //1

            Event::KeyDown {
                keycode: Some(Keycode::Num2),
                ..
            } => self.key[0x2] = true, //2

            Event::KeyUp {
                keycode: Some(Keycode::Num2),
                ..
            } => self.key[0x2] = false, //2

            Event::KeyDown {
                keycode: Some(Keycode::Num3),
                ..
            } => self.key[0x3] = true, //3

            Event::KeyUp {
                keycode: Some(Keycode::Num3),
                ..
            } => self.key[0x3] = false, //3

            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => self.key[0x4] = true, //4

            Event::KeyUp {
                keycode: Some(Keycode::Q),
                ..
            } => self.key[0x4] = false, //4

            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => self.key[0x5] = true, //5

            Event::KeyUp {
                keycode: Some(Keycode::W),
                ..
            } => self.key[0x5] = false, //5

            Event::KeyDown {
                keycode: Some(Keycode::E),
                ..
            } => self.key[0x6] = true, //6

            Event::KeyUp {
                keycode: Some(Keycode::E),
                ..
            } => self.key[0x6] = false, //6

            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => self.key[0x7] = true, //7

            Event::KeyUp {
                keycode: Some(Keycode::A),
                ..
            } => self.key[0x7] = false, //7

            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => self.key[0x8] = true, //8

            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } => self.key[0x8] = false, //8

            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => self.key[0x9] = true, //9

            Event::KeyUp {
                keycode: Some(Keycode::D),
                ..
            } => self.key[0x9] = false, //9

            Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => self.key[0xa] = true, //A

            Event::KeyUp {
                keycode: Some(Keycode::Z),
                ..
            } => self.key[0xa] = false, //A

            Event::KeyDown {
                keycode: Some(Keycode::X),
                ..
            } => self.key[0x0] = true, //0

            Event::KeyUp {
                keycode: Some(Keycode::X),
                ..
            } => self.key[0x0] = false, //0

            Event::KeyDown {
                keycode: Some(Keycode::C),
                ..
            } => self.key[0xb] = true, //B

            Event::KeyUp {
                keycode: Some(Keycode::C),
                ..
            } => self.key[0xb] = false, //B

            Event::KeyDown {
                keycode: Some(Keycode::Num4),
                ..
            } => self.key[0xc] = true, //C

            Event::KeyUp {
                keycode: Some(Keycode::Num4),
                ..
            } => self.key[0xc] = false, //C

            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => self.key[0xd] = true, //D

            Event::KeyUp {
                keycode: Some(Keycode::R),
                ..
            } => self.key[0xd] = false, //D

            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => self.key[0xe] = true, //E

            Event::KeyUp {
                keycode: Some(Keycode::F),
                ..
            } => self.key[0xe] = false, //E

            Event::KeyDown {
                keycode: Some(Keycode::V),
                ..
            } => self.key[0xf] = true, //F

            Event::KeyUp {
                keycode: Some(Keycode::V),
                ..
            } => self.key[0xf] = false, //F

            _ => (),
        }
    }

    pub fn fetch_opcode(&self) -> [u8; 2] {
        let mut opcode = [0u8; 2];
        opcode[0] = self.memory[self.pc as usize];
        opcode[1] = self.memory[(self.pc + 1) as usize];
        //maybe increment pc now
        opcode
    }
    pub fn execute_opcode(&mut self, opcode: (Opcode, [u8; 2])) {
        //put this here since it will modify the chip8struct
        // not sure if I should use to option here or handle it in main
        match opcode.0 {
            Opcode::CLS => {
                self.gfx = [0; 64 * 32];
                self.pc += 2;
            }
            Opcode::RET => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
                self.pc += 2;
            }
            Opcode::SYS => self.pc += 2, //apparently this is ignored these days ,
            Opcode::JP_A => {
                let mut addr: u16 = opcode.1[0] as u16;
                addr -= 0x10;
                addr = addr << 8;
                addr += opcode.1[1] as u16;
                self.pc = addr;
            }
            Opcode::CALL => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                let mut addr: u16 = opcode.1[0] as u16;
                addr -= 0x20;

                addr = addr << 8;

                addr += opcode.1[1] as u16;

                self.pc = addr;
            }
            Opcode::SE_VB => {
                if self.v_register[(opcode.1[0] - 0x30) as usize] == opcode.1[1] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::SNE_VB => {
                if self.v_register[(opcode.1[0] - 0x40) as usize] != opcode.1[1] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::SE_VV => {
                if self.v_register[(opcode.1[0] - 0x50) as usize]
                    == self.v_register[(opcode.1[1] >> 4) as usize]
                {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::LD_VB => {
                self.v_register[(opcode.1[0] - 0x60) as usize] = opcode.1[1];
                self.pc += 2;
            }
            Opcode::ADD_VB => {
                self.v_register[(opcode.1[0] - 0x70) as usize] = self.v_register
                    [(opcode.1[0] - 0x70) as usize]
                    .overflowing_add(opcode.1[1])
                    .0;
                self.pc += 2
            }
            Opcode::LD_VV => {
                self.v_register[(opcode.1[0] - 0x80) as usize] =
                    self.v_register[(opcode.1[1] >> 4) as usize];
                self.pc += 2;
            }
            Opcode::OR => {
                self.v_register[(opcode.1[0] - 0x80) as usize] = self.v_register
                    [(opcode.1[0] - 0x80) as usize]
                    | self.v_register[(opcode.1[1] >> 4) as usize];
                self.pc += 2;
            }
            Opcode::AND => {
                self.v_register[(opcode.1[0] - 0x80) as usize] = self.v_register
                    [(opcode.1[0] - 0x80) as usize]
                    & self.v_register[(opcode.1[1] >> 4) as usize];
                self.pc += 2;
            }
            Opcode::XOR => {
                self.v_register[(opcode.1[0] - 0x80) as usize] = self.v_register
                    [(opcode.1[0] - 0x80) as usize]
                    ^ self.v_register[(opcode.1[1] >> 4) as usize];
                self.pc += 2;
            }
            Opcode::ADD_VV => {
                let overflow_add = self.v_register[(opcode.1[0] - 0x80) as usize]
                    .overflowing_add(self.v_register[(opcode.1[1] >> 4) as usize]);
                self.v_register[(opcode.1[0] - 0x80) as usize] = overflow_add.0;
                if overflow_add.1 {
                    self.v_register[15] = 1;
                } else {
                    self.v_register[15] = 0;
                }
                self.pc += 2;
            }
            Opcode::SUB => {
                let overflow_sub = self.v_register[(opcode.1[0] - 0x80) as usize]
                    .overflowing_sub(self.v_register[(opcode.1[1] >> 4) as usize]);
                self.v_register[(opcode.1[0] - 0x80) as usize] = overflow_sub.0;
                if overflow_sub.1 {
                    self.v_register[15] = 0;
                } else {
                    self.v_register[15] = 1;
                }
                self.pc += 2;
            }
            Opcode::SHR => {
                if self.v_register[(opcode.1[0] - 0x80) as usize].trailing_zeros() < 1 {
                    self.v_register[15] = 1;
                } else {
                    self.v_register[15] = 0
                };
                self.v_register[(opcode.1[0] - 0x80) as usize] =
                    self.v_register[(opcode.1[0] - 0x80) as usize] >> 1;
                self.pc += 2;
            }
            Opcode::SUBN => {
                let overflow_sub = self.v_register[(opcode.1[1] >> 4) as usize]
                    .overflowing_sub(self.v_register[(opcode.1[0] - 0x80) as usize]);
                self.v_register[(opcode.1[0] - 0x80) as usize] = overflow_sub.0;
                if overflow_sub.1 {
                    self.v_register[15] = 0;
                } else {
                    self.v_register[15] = 1;
                }
                self.pc += 2;
            }
            Opcode::SHL => {
                if self.v_register[(opcode.1[0] - 0x80) as usize] > 0x80 {
                    self.v_register[15] = 1;
                } else {
                    self.v_register[15] = 0
                };
                self.v_register[(opcode.1[0] - 0x80) as usize] =
                    self.v_register[(opcode.1[0] - 0x80) as usize] << 1;
                self.pc += 2;
            }
            Opcode::SNE_VV => {
                if self.v_register[(opcode.1[0] - 0x90) as usize]
                    != self.v_register[(opcode.1[1] >> 4) as usize]
                {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::LD_IA => {
                self.I = (opcode.1[0] as u16 - 0xa0 << 8) + opcode.1[1] as u16;
                self.pc += 2;
            }
            Opcode::JP_VA => {
                self.pc = (((opcode.1[0] as u16 - 0xb0) << 8) + opcode.1[1] as u16)
                    + self.v_register[0] as u16;
            }
            Opcode::RND => {
                let mut rng = thread_rng();
                let random_num: u8 = rng.gen();
                let combo = opcode.1[1] & random_num;
                self.v_register[(opcode.1[0] - 0xc0) as usize] = combo;
                self.pc += 2;
            }
            Opcode::DRW => {
                // read value at I into a buffer one byte at a time and increment I. once xor newvalue with taht put the result into location of I and decrement it
                //thisresults in I being the same as before and not needing a diferent value to track it
                for row in 0..(opcode.1[1] % 16) {
                    //this gets just the lowest byte
                    let new_sprite: u8 = self.memory[(self.I + row as u16) as usize];
                    let mut current_sprite: u8 = 0;
                    for index in 0..8 {
                        current_sprite = current_sprite << 1;
                        current_sprite += self.gfx[xy_coord(
                            (self.v_register[(opcode.1[0] - 0xd0) as usize] + index) as u32,
                            (self.v_register[(opcode.1[1] >> 4) as usize] + row) as u32,
                        ) as usize];
                    }

                    let mut new_bytes = current_sprite ^ new_sprite;
                    self.v_register[0x0f as usize] = 0;
                    for index in (0..8).rev() {
                        if self.gfx[xy_coord(
                            (self.v_register[(opcode.1[0] - 0xd0) as usize] + index) as u32,
                            (self.v_register[(opcode.1[1] >> 4) as usize] + row) as u32,
                        ) as usize]
                            == 1
                            && new_bytes % 2 == 0
                        {
                            self.v_register[0x0f as usize] = 1;
                        }
                        self.gfx[xy_coord(
                            (self.v_register[(opcode.1[0] - 0xd0) as usize] + index) as u32,
                            (self.v_register[(opcode.1[1] >> 4) as usize] + row) as u32,
                        ) as usize] = new_bytes % 2;
                        new_bytes /= 2;
                    }
                }
                self.pc += 2;
            }
            Opcode::SKP => {
                if self.key[self.v_register[(opcode.1[0] - 0xe0) as usize] as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::SKNP => {
                if !self.key[self.v_register[(opcode.1[0] - 0xe0) as usize] as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::LD_VDT => {
                self.v_register[(opcode.1[0] - 0xf0) as usize] = self.delay_timer;
                self.pc += 2;
            }
            Opcode::LD_VK => {
                let mut pause = true;

                for button in 0..16 {
                    if self.key[button] == true {
                        pause = false;
                        self.v_register[(opcode.1[0] - 0xf0) as usize] = button as u8;
                    }
                }

                if !pause {
                    self.pc += 2;
                }
            }
            Opcode::LD_DTV => {
                self.v_register[(opcode.1[0] - 0xf0) as usize] = self.delay_timer;
                self.pc += 2;
            }
            Opcode::LD_STV => {
                self.sound_timer = self.v_register[(opcode.1[0] - 0xf0) as usize];
                self.pc += 2;
            }
            Opcode::ADD_IV => {
                self.I += self.v_register[(opcode.1[0] - 0xf0) as usize] as u16;
                self.pc += 2;
            }
            Opcode::LD_FV => {
                self.I = 0x050 + (self.v_register[(opcode.1[0] - 0xf0) as usize] * 5) as u16;
                self.pc += 2;
            }
            Opcode::LD_BV => {
                self.memory[self.I as usize] = self.v_register[(opcode.1[0] - 0xf0) as usize] / 100;
                let buf: u8 = self.v_register[(opcode.1[0] - 0xf0) as usize] % 100;
                self.memory[(self.I + 1) as usize] = buf / 10;
                self.memory[(self.I + 2) as usize] = buf % 10;
                self.pc += 2;
            }
            Opcode::LD_IV => {
                for register in 0..=(opcode.1[0] - 0xf0) {
                    self.memory[(self.I + register as u16) as usize] =
                        self.v_register[register as usize]
                }
                self.pc += 2;
            }
            Opcode::LD_VI => {
                for register in 0..=(opcode.1[0] - 0xf0) {
                    self.v_register[register as usize] =
                        self.memory[(self.I + register as u16) as usize]
                }
                self.pc += 2;
            }
        }
        //self.pc += 2; //this may need to move as i think jump instructions probably should not increment past the location or those just need to coutner act by subtracting 2
    }
    pub fn decrease_timers(&mut self) {
        if self.delay_timer != 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer != 0 {
            self.sound_timer -= 1
        }
    }
}

fn xy_coord(x: u32, y: u32) -> u32 {
    // should maybe set a value for screen width in case it gets changed. this returns the index of a value when give its xy corrds
    let mut final_x = 0;
    let mut final_y = 0;
    if x >= 64 {
        final_x = x - 64;
    } else {
        final_x = x;
    }
    if y >= 32 {
        final_y = y - 32;
    } else {
        final_y = y
    }
    (64 * final_y) + final_x
}

fn highest_hex_value(test_value: u8, wanted_value: u8) -> bool {
    //the wanted value can have any second byte value this only check the left most hex byte
    //this is done with an xor and if it matches it gets the first value to be zero and returns true
    if (test_value ^ wanted_value) <= 0x0f {
        return true;
    }
    false
}

fn lowest_hex_value(test_value: u8, wanted_value: u8) -> bool {
    highest_hex_value(test_value << 4, wanted_value << 4)
}

fn both_hex_values(test_value: u8, wanted_value: u8) -> bool {
    test_value == wanted_value
}

pub fn decode_opcode(opcode: [u8; 2]) -> Option<(Opcode, [u8; 2])> {
    if highest_hex_value(opcode[0], 0x00) {
        if both_hex_values(opcode[1], 0xe0) {
            return Some((Opcode::CLS, opcode));
        } else if both_hex_values(opcode[1], 0xee) {
            return Some((Opcode::RET, opcode));
        }
        return Some((Opcode::SYS, opcode));
    } else if highest_hex_value(opcode[0], 0x10) {
        return Some((Opcode::JP_A, opcode));
    } else if highest_hex_value(opcode[0], 0x20) {
        return Some((Opcode::CALL, opcode));
    } else if highest_hex_value(opcode[0], 0x30) {
        return Some((Opcode::SE_VB, opcode));
    } else if highest_hex_value(opcode[0], 0x40) {
        return Some((Opcode::SNE_VB, opcode));
    } else if highest_hex_value(opcode[0], 0x50) {
        return Some((Opcode::SE_VV, opcode));
    } else if highest_hex_value(opcode[0], 0x60) {
        return Some((Opcode::LD_VB, opcode));
    } else if highest_hex_value(opcode[0], 0x70) {
        return Some((Opcode::ADD_VB, opcode));
    } else if highest_hex_value(opcode[0], 0x80) {
        if lowest_hex_value(opcode[1], 0x00) {
            return Some((Opcode::LD_VV, opcode));
        } else if lowest_hex_value(opcode[1], 0x01) {
            return Some((Opcode::OR, opcode));
        } else if lowest_hex_value(opcode[1], 0x02) {
            return Some((Opcode::AND, opcode));
        } else if lowest_hex_value(opcode[1], 0x03) {
            return Some((Opcode::XOR, opcode));
        } else if lowest_hex_value(opcode[1], 0x04) {
            return Some((Opcode::ADD_VV, opcode));
        } else if lowest_hex_value(opcode[1], 0x05) {
            return Some((Opcode::SUB, opcode));
        } else if lowest_hex_value(opcode[1], 0x06) {
            return Some((Opcode::SHR, opcode));
        } else if lowest_hex_value(opcode[1], 0x07) {
            return Some((Opcode::SUBN, opcode));
        } else if lowest_hex_value(opcode[1], 0x0e) {
            return Some((Opcode::SHL, opcode));
        }
        return None;
    } else if highest_hex_value(opcode[0], 0x90) {
        return Some((Opcode::SNE_VV, opcode));
    } else if highest_hex_value(opcode[0], 0xA0) {
        return Some((Opcode::LD_IA, opcode));
    } else if highest_hex_value(opcode[0], 0xB0) {
        return Some((Opcode::JP_VA, opcode));
    } else if highest_hex_value(opcode[0], 0xC0) {
        return Some((Opcode::RND, opcode));
    } else if highest_hex_value(opcode[0], 0xD0) {
        return Some((Opcode::DRW, opcode));
    } else if highest_hex_value(opcode[0], 0xE0) {
        if both_hex_values(opcode[1], 0x9e) {
            return Some((Opcode::SKP, opcode));
        } else if both_hex_values(opcode[1], 0xa1) {
            return Some((Opcode::SKNP, opcode));
        }
        return None;
    } else if highest_hex_value(opcode[0], 0xF0) {
        if both_hex_values(opcode[1], 0x07) {
            return Some((Opcode::LD_VDT, opcode));
        } else if both_hex_values(opcode[1], 0x0a) {
            return Some((Opcode::LD_VK, opcode));
        } else if both_hex_values(opcode[1], 0x15) {
            return Some((Opcode::LD_DTV, opcode));
        } else if both_hex_values(opcode[1], 0x18) {
            return Some((Opcode::LD_STV, opcode));
        } else if both_hex_values(opcode[1], 0x1e) {
            return Some((Opcode::ADD_IV, opcode));
        } else if both_hex_values(opcode[1], 0x29) {
            return Some((Opcode::LD_FV, opcode));
        } else if both_hex_values(opcode[1], 0x33) {
            return Some((Opcode::LD_BV, opcode));
        } else if both_hex_values(opcode[1], 0x55) {
            return Some((Opcode::LD_IV, opcode));
        } else if both_hex_values(opcode[1], 0x65) {
            return Some((Opcode::LD_VI, opcode));
        }
        return None;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_ret() {
        let decoded = decode_opcode([0x00, 0xee]);
        assert_eq!(decoded.unwrap(), (Opcode::RET, [0x00u8, 0xeeu8]));
    }
    #[test]
    fn execute_ret() {
        let mut test_chip8 = Chip8::init();
        test_chip8.sp += 1;
        test_chip8.stack[test_chip8.sp as usize] = 32;
        test_chip8.execute_opcode((Opcode::RET, [0x00, 0xee]));
        assert_eq!(test_chip8.sp, 0);
        assert_eq!(test_chip8.pc, 34);
    }
    #[test]
    fn decode_jp_a() {
        let decoded = decode_opcode([0x1f, 0xe0]);
        assert_eq!(decoded.unwrap(), (Opcode::JP_A, [0x1fu8, 0xe0u8]));
    }

    #[test]
    fn execute_jp_a() {
        let mut test_chip8 = Chip8::init();
        test_chip8.execute_opcode((Opcode::JP_A, [0x13, 0x33]));
        assert_eq!(test_chip8.pc, 0x0333);
    }

    #[test]
    fn decode_call() {
        let decoded = decode_opcode([0x2e, 0xf4]);
        assert_eq!(decoded.unwrap(), (Opcode::CALL, [0x2eu8, 0xf4u8]));
    }

    #[test]
    fn execute_call() {
        let mut test_chip8 = Chip8::init();
        test_chip8.pc = 0x44;
        test_chip8.execute_opcode((Opcode::CALL, [0x22, 0x22]));
        assert_eq!(test_chip8.sp, 1);
        assert_eq!(test_chip8.stack[test_chip8.sp as usize], 0x44);
        assert_eq!(test_chip8.pc, 0x222);
    }

    #[test]
    fn decode_se_vb() {
        let decoded = decode_opcode([0x35, 0xca]);
        assert_eq!(decoded.unwrap(), (Opcode::SE_VB, [0x35u8, 0xcau8]));
    }

    #[test]
    fn execute_se_vb() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[0xa] = 0xef;
        test_chip8.execute_opcode((Opcode::SE_VB, [0x3a, 0xef]));
        assert_eq!(test_chip8.pc, 0x204);
    }

    #[test]
    fn decode_sne_vb() {
        let decoded = decode_opcode([0x4a, 0x8f]);
        assert_eq!(decoded.unwrap(), (Opcode::SNE_VB, [0x4au8, 0x8fu8]));
    }

    #[test]
    fn execute_sne_vb() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[0xa] = 0xfe;
        test_chip8.execute_opcode((Opcode::SNE_VB, [0x4a, 0xef]));
        assert_eq!(test_chip8.pc, 0x204);
    }

    #[test]
    fn decode_se_vv() {
        let decoded = decode_opcode([0x5f, 0xa0]);
        assert_eq!(decoded.unwrap(), (Opcode::SE_VV, [0x5fu8, 0xa0u8]));
    }

    #[test]
    fn exectue_se_vv() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[0] = 4;
        test_chip8.v_register[0xa] = 4;
        test_chip8.execute_opcode((Opcode::SE_VV, [0x50, 0xa0]));
        assert_eq!(test_chip8.pc, 0x204);
    }

    #[test]
    fn decode_ld_vb() {
        let decoded = decode_opcode([0x6e, 0xab]);
        assert_eq!(decoded.unwrap(), (Opcode::LD_VB, [0x6eu8, 0xabu8]));
    }

    #[test]
    fn execute_ld_vb() {
        let mut test_chip8 = Chip8::init();
        test_chip8.execute_opcode((Opcode::LD_VB, [0x65, 0xe3]));
        assert_eq!(test_chip8.v_register[5], 0xe3);
    }

    #[test]
    fn decode_add_vb() {
        let decoded = decode_opcode([0x74, 0xde]);
        assert_eq!(decoded.unwrap(), (Opcode::ADD_VB, [0x74u8, 0xdeu8]));
    }

    #[test]
    fn execute_add_vb_no_overflow() {
        let mut test_chip8 = Chip8::init();
        test_chip8.execute_opcode((Opcode::ADD_VB, [0x77, 0x25]));
        assert_eq!(test_chip8.v_register[7], 0x25);
    }

    #[test]
    fn execute_add_vb_overflow() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[7] = 0xff;
        test_chip8.execute_opcode((Opcode::ADD_VB, [0x77, 0x25]));
        assert_eq!(test_chip8.v_register[7], 0x24);
    }

    #[test]
    fn decode_ld_vv() {
        let decoded = decode_opcode([0x83, 0xc0]);
        assert_eq!(decoded.unwrap(), (Opcode::LD_VV, [0x83u8, 0xc0u8]));
    }

    #[test]
    fn execute_ld_vv() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[4] = 0xff;
        test_chip8.execute_opcode((Opcode::LD_VV, [0x88, 0x40]));
        assert_eq!(test_chip8.v_register[8], 0xff);
    }

    #[test]
    fn decode_or() {
        let decoded = decode_opcode([0x87, 0x31]);
        assert_eq!(decoded.unwrap(), (Opcode::OR, [0x87u8, 0x31u8]));
    }

    #[test]
    fn execute_or() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[8] = 0xaa;
        test_chip8.v_register[4] = 0x99;
        test_chip8.execute_opcode((Opcode::OR, [0x88, 0x41]));
        assert_eq!(test_chip8.v_register[8], 0xbb);
    }

    #[test]
    fn decode_and() {
        let decoded = decode_opcode([0x82, 0x72]);
        assert_eq!(decoded.unwrap(), (Opcode::AND, [0x82u8, 0x72u8]));
    }

    #[test]
    fn execute_and() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[8] = 0xaa;
        test_chip8.v_register[4] = 0xc3;
        test_chip8.execute_opcode((Opcode::AND, [0x88, 0x42]));
        assert_eq!(test_chip8.v_register[8], 0x82);
    }

    #[test]
    fn decode_xor() {
        let decoded = decode_opcode([0x8a, 0xf3]);
        assert_eq!(decoded.unwrap(), (Opcode::XOR, [0x8au8, 0xf3u8]));
    }

    #[test]
    fn execute_xor() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[8] = 0xaa;
        test_chip8.v_register[4] = 0xc3;
        test_chip8.execute_opcode((Opcode::XOR, [0x88, 0x43]));
        assert_eq!(test_chip8.v_register[8], 0x69);
    }

    #[test]
    fn decode_add_vv() {
        let decoded = decode_opcode([0x89, 0x44]);
        assert_eq!(decoded.unwrap(), (Opcode::ADD_VV, [0x89u8, 0x44u8]));
    }

    #[test]
    fn execute_add_vv_no_overflow() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[8] = 0x0a;
        test_chip8.v_register[4] = 0xa0;
        test_chip8.execute_opcode((Opcode::ADD_VV, [0x88, 0x44]));
        assert_eq!(test_chip8.v_register[8], 0xaa);
        assert_eq!(test_chip8.v_register[0xf], 0x00);
    }

    #[test]
    fn execute_add_vv_overflow() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[8] = 0xaa;
        test_chip8.v_register[4] = 0xa0;
        test_chip8.execute_opcode((Opcode::ADD_VV, [0x88, 0x44]));
        assert_eq!(test_chip8.v_register[8], 0x4a);
        assert_eq!(test_chip8.v_register[0xf], 0x01);
    }

    #[test]
    fn decode_sub() {
        let decoded = decode_opcode([0x80, 0x35]);
        assert_eq!(decoded.unwrap(), (Opcode::SUB, [0x80u8, 0x35u8]));
    }

    #[test]
    fn execute_sub_no_underflow() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[8] = 0xaa;
        test_chip8.v_register[4] = 0x0a;
        test_chip8.execute_opcode((Opcode::SUB, [0x88, 0x45]));
        assert_eq!(test_chip8.v_register[8], 0xa0);
        assert_eq!(test_chip8.v_register[0xf], 0x01);
    }

    #[test]
    fn execute_sub_underflow() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[8] = 0x0a;
        test_chip8.v_register[4] = 0xaa;
        test_chip8.execute_opcode((Opcode::SUB, [0x88, 0x45]));
        assert_eq!(test_chip8.v_register[8], 0x60);
        assert_eq!(test_chip8.v_register[0xf], 0x00);
    }

    #[test]
    fn decode_shr() {
        let decoded = decode_opcode([0x88, 0x76]);
        assert_eq!(decoded.unwrap(), (Opcode::SHR, [0x88u8, 0x76u8]));
    }

    #[test]
    fn execute_shr_no_overflow() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[8] = 0xee;
        test_chip8.execute_opcode((Opcode::SHR, [0x88, 0x46]));
        assert_eq!(test_chip8.v_register[8], 0x77);
        assert_eq!(test_chip8.v_register[0xf], 0x00);
    }

    #[test]
    fn execute_shr_overflow() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[8] = 0xef;
        test_chip8.execute_opcode((Opcode::SHR, [0x88, 0x46]));
        assert_eq!(test_chip8.v_register[8], 0x77);
        assert_eq!(test_chip8.v_register[0xf], 0x01);
    }

    #[test]
    fn decode_subn() {
        let decoded = decode_opcode([0x8e, 0xd7]);
        assert_eq!(decoded.unwrap(), (Opcode::SUBN, [0x8eu8, 0xd7u8]));
    }

    #[test]
    fn execute_subn_no_overflow() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[8] = 0x0a;
        test_chip8.v_register[4] = 0xaa;
        test_chip8.execute_opcode((Opcode::SUBN, [0x88, 0x47]));
        assert_eq!(test_chip8.v_register[8], 0xa0);
        assert_eq!(test_chip8.v_register[0xf], 0x01);
    }

    #[test]
    fn execute_subn_overflow() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[8] = 0xaa;
        test_chip8.v_register[4] = 0xa0;
        test_chip8.execute_opcode((Opcode::SUBN, [0x88, 0x47]));
        assert_eq!(test_chip8.v_register[8], 0xf6);
        assert_eq!(test_chip8.v_register[0xf], 0x00);
    }

    #[test]
    fn decode_shl() {
        let decoded = decode_opcode([0x86, 0xae]);
        assert_eq!(decoded.unwrap(), (Opcode::SHL, [0x86u8, 0xaeu8]));
    }

    #[test]
    fn execute_shl_overflow() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[4] = 0xaa;
        test_chip8.execute_opcode((Opcode::SHL, [0x84, 0x3e]));
        assert_eq!(test_chip8.v_register[4], 0x54);
        assert_eq!(test_chip8.v_register[0xf], 1);
    }

    #[test]
    fn execute_shl_no_overflow() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[4] = 0x1a;
        test_chip8.execute_opcode((Opcode::SHL, [0x84, 0x3e]));
        assert_eq!(test_chip8.v_register[4], 0x34);
        assert_eq!(test_chip8.v_register[0xf], 0);
    }

    #[test]
    fn decode_sne_vv() {
        let decoded = decode_opcode([0x91, 0xb0]);
        assert_eq!(decoded.unwrap(), (Opcode::SNE_VV, [0x91u8, 0xb0u8]));
    }

    #[test]
    fn execute_sne_VV_not_equal() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[4] = 0xaa;
        test_chip8.v_register[3] = 0xba;
        test_chip8.execute_opcode((Opcode::SNE_VV, [0x94, 0x30]));
        assert_eq!(test_chip8.pc, 0x204);
    }

    #[test]
    fn execute_sne_VV_equal() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[4] = 0xaa;
        test_chip8.v_register[3] = 0xaa;
        test_chip8.execute_opcode((Opcode::SNE_VV, [0x94, 0x30]));
        assert_eq!(test_chip8.pc, 0x202);
    }

    #[test]
    fn decode_ld_ia() {
        let decoded = decode_opcode([0xaa, 0xaa]);
        assert_eq!(decoded.unwrap(), (Opcode::LD_IA, [0xaau8, 0xaau8]));
    }

    #[test]
    fn execute_ld_ia() {
        let mut test_chip8 = Chip8::init();
        test_chip8.execute_opcode((Opcode::LD_IA, [0xA4, 0x30]));
        assert_eq!(test_chip8.I, 0x430);
    }

    #[test]
    fn decode_jp_va() {
        let decoded = decode_opcode([0xbb, 0xbb]);
        assert_eq!(decoded.unwrap(), (Opcode::JP_VA, [0xbbu8, 0xbbu8]));
    }

    #[test]
    fn execute_jp_va() {
        let mut test_chip8 = Chip8::init();
        test_chip8.v_register[0] = 0xa;
        test_chip8.execute_opcode((Opcode::JP_VA, [0xb4, 0x30]));
        assert_eq!(test_chip8.pc, 0x43a);
    }

    #[test]
    fn decode_rnd() {
        let decoded = decode_opcode([0xcc, 0xcc]);
        assert_eq!(decoded.unwrap(), (Opcode::RND, [0xccu8, 0xccu8]));
    }

    #[test]
    fn decode_drw() {
        let decoded = decode_opcode([0xdd, 0xdd]);
        assert_eq!(decoded.unwrap(), (Opcode::DRW, [0xddu8, 0xddu8]));
    }

    #[test]
    fn decode_skp() {
        let decoded = decode_opcode([0xee, 0x9e]);
        assert_eq!(decoded.unwrap(), (Opcode::SKP, [0xeeu8, 0x9eu8]));
    }

    #[test]
    fn execute_skp_pressed() {
        let mut test_chip8 = Chip8::init();
        test_chip8.key[0xe] = true;
        test_chip8.v_register[0xe] = 0xe;
        test_chip8.execute_opcode((Opcode::SKP, [0xee, 0x9e]));
        assert_eq!(test_chip8.pc, 0x204);
    }

    #[test]
    fn execute_skp_not_pressed() {
        let mut test_chip8 = Chip8::init();
        test_chip8.key[0xe] = false;
        test_chip8.v_register[0x4] = 0xe;
        test_chip8.execute_opcode((Opcode::SKP, [0xee, 0x9e]));
        assert_eq!(test_chip8.pc, 0x202);
    }

    #[test]
    fn decode_sknp() {
        let decoded = decode_opcode([0xe4, 0xa1]);
        assert_eq!(decoded.unwrap(), (Opcode::SKNP, [0xe4u8, 0xa1u8]));
    }

    #[test]
    fn execute_sknp_pressed() {
        let mut test_chip8 = Chip8::init();
        test_chip8.key[0x4] = true;
        test_chip8.v_register[4] = 4;
        test_chip8.execute_opcode((Opcode::SKNP, [0xe4, 0xa1]));
        assert_eq!(test_chip8.pc, 0x202);
    }

    #[test]
    fn execute_sknp_not_pressed() {
        let mut test_chip8 = Chip8::init();
        test_chip8.key[0x4] = false;
        test_chip8.v_register[4] = 4;
        test_chip8.execute_opcode((Opcode::SKNP, [0xe4, 0xa1]));
        assert_eq!(test_chip8.pc, 0x204);
    }

    #[test]
    fn decode_ld_vdt() {
        let decoded = decode_opcode([0xf3, 0x07]);
        assert_eq!(decoded.unwrap(), (Opcode::LD_VDT, [0xf3u8, 0x07u8]));
    }

    #[test]
    fn decode_ld_vk() {
        let decoded = decode_opcode([0xf0, 0x0a]);
        assert_eq!(decoded.unwrap(), (Opcode::LD_VK, [0xf0u8, 0x0au8]));
    }

    #[test]
    fn decode_ld_dtv() {
        let decoded = decode_opcode([0xfa, 0x15]);
        assert_eq!(decoded.unwrap(), (Opcode::LD_DTV, [0xfau8, 0x15u8]));
    }

    #[test]
    fn decode_ld_stv() {
        let decoded = decode_opcode([0xfe, 0x18]);
        assert_eq!(decoded.unwrap(), (Opcode::LD_STV, [0xfeu8, 0x18u8]));
    }

    #[test]
    fn decode_add_iv() {
        let decoded = decode_opcode([0xfe, 0x1e]);
        assert_eq!(decoded.unwrap(), (Opcode::ADD_IV, [0xfeu8, 0x1eu8]));
    }

    #[test]
    fn decode_ld_fv() {
        let decoded = decode_opcode([0xf3, 0x29]);
        assert_eq!(decoded.unwrap(), (Opcode::LD_FV, [0xf3u8, 0x29u8]));
    }

    #[test]
    fn decode_ld_bv() {
        let decoded = decode_opcode([0xfe, 0x33]);
        assert_eq!(decoded.unwrap(), (Opcode::LD_BV, [0xfeu8, 0x33u8]));
    }

    #[test]
    fn decode_ld_iv() {
        let decoded = decode_opcode([0xfa, 0x55]);
        assert_eq!(decoded.unwrap(), (Opcode::LD_IV, [0xfau8, 0x55u8]));
    }

    #[test]
    fn decode_vi() {
        let decoded = decode_opcode([0xfb, 0x65]);
        assert_eq!(decoded.unwrap(), (Opcode::LD_VI, [0xfbu8, 0x65u8]));
    }
}
