extern crate sdl2;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;

use rand::Rng;

use std::time::Instant;

//mod debugger;
use debugger::*;


//---------------------------------------------ROM-------------------------------------------------
struct ROM {
    buffer: Vec<u8>,
}

impl ROM {
    fn new(filename: &str) -> ROM {
        let mut f = File::open(&filename).expect("file not found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        return ROM { buffer };
        //DEBUG: print vec as bytes
        //println!("{:#04x?}", buffer);
    }
}

//-------------------------------------------------------------------------------------------------

//---------------------------------------------Memory----------------------------------------------
struct Memory {
    //memory
    mem: [u8; 4096],
    program_base: i32,

    //registers
    Vregs: [u8; 16],
    I: u16,
    Vd: u8,
    Vs: u8,

    //stack
    SP: u8,
    stack: [u16; 16],
}

impl Memory {
    fn new() -> Memory {
        Memory {
            mem: [0; 4096],
            program_base: 512,
            Vregs: [0; 16],
            I: 0,
            Vd: 0,
            Vs: 0,
            SP: 0,
            stack: [0; 16],
        }
    }

    fn init(&mut self, rom: &ROM) {
        //copy ROM data to memory starting at 0x200(512)
        self.mem
            [self.program_base as usize..(self.program_base + (rom.buffer.len() as i32)) as usize]
            .copy_from_slice(&rom.buffer);

        /* font sprite raw data
        0 graphic 	F0 90 90 90 F0
        1 graphic 	20 60 20 20 70
        2 graphic 	F0 10 F0 80 F0
        3 graphic 	F0 10 F0 10 F0
        4 graphic 	90 90 F0 10 10
        5 graphic 	F0 80 F0 10 F0
        6 graphic 	F0 80 F0 90 F0
        7 graphic 	F0 10 20 40 40
        8 graphic 	F0 90 F0 90 F0
        9 graphic 	F0 90 F0 10 F0
        A graphic 	F0 90 F0 90 90
        B graphic 	E0 90 E0 90 E0
        C graphic 	F0 80 80 80 F0
        D graphic 	E0 90 90 90 E0
        E graphic 	F0 80 F0 80 F0
        F graphic 	F0 80 F0 80 80 */
        //TODO: reformat this lol
        let font_sprites: Vec<u8> = vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];

        self.mem[0..font_sprites.len()].copy_from_slice(&font_sprites);
    }

    //dump memory to console
    fn dump(&self) {
        //we are always printing 4KB of ROM
        //16 bytes at a time

        for i in 0..256 {
            print!("{:04x}\t", (i * 16));

            for j in 0..8 {
                print!(
                    "{:02x}{:02x} ",
                    self.mem[(i * 16) + (j * 2)],
                    self.mem[(i * 16) + ((j * 2) + 1)]
                );
            }

            println!("");
        }
    }
}
//-------------------------------------------------------------------------------------------------

//-----------------------------------------------Display-------------------------------------------
struct Display {
    pixels: [[ScreenPixel; 64]; 32],
    //NOTE: nessecary for display to actually draw things
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Display {
    fn new(sdl_context: Sdl) -> Display {
        let mut pixels: [[ScreenPixel; 64]; 32] =
            [[ScreenPixel::new(Rect::new(0, 0, 10, 10), false); 64]; 32];

        //TODO: can we do this in the initializer?
        for i in 0..32 {
            for j in 0..64 {
                pixels[i][j].pixel.x = (j * 10) as i32;
                pixels[i][j].pixel.y = (i * 10) as i32;
            }
        }

        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        Display {
            pixels: pixels,
            canvas: canvas
        }
    }

    fn updateDisp(&mut self) {
        for row in self.pixels {
            for pixel in row {
                if pixel.state {
                    self.canvas.set_draw_color(Color::RED);
                    self.canvas.fill_rect(pixel.pixel);
                }
                else{
                    self.canvas.set_draw_color(Color::BLACK);
                    self.canvas.fill_rect(pixel.pixel);
                }
            }
        }
        self.canvas.present();
    }

    fn clearDisp(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        for row in self.pixels {
            for mut pixel in row {
                pixel.state = false;
                self.canvas.fill_rect(pixel.pixel);
            }
        }
        self.canvas.present();
    }

    //XORs a sprite into display buffer
    fn pushSprite(&mut self, sprite: sprite) {
        for pixel in sprite.pixels {
            let x = (pixel.pixel.x / 10) as usize;
            let y = (pixel.pixel.y / 10) as usize;
            //println!("XORing pixel at x:{}, y:{}", x, y);
            self.pixels[y][x].state = self.pixels[y][x].state ^ pixel.state;
            //println!("pixel at x:{}, y:{} is now: {}", xIndex, yIndex, self.pixels[xIndex][yIndex].state);
        }
    }
}
//----------------------
//represents a pixels as a Rect so SDL can draw it and a boolean to represent if its on or not
#[derive(Copy, Clone)]
struct ScreenPixel {
    pixel: Rect,
    state: bool,
}
impl ScreenPixel {
    fn new(pixel: Rect, state: bool) -> ScreenPixel {
        ScreenPixel {
            pixel: pixel,
            state: state,
        }
    }
}
//----------------------

struct sprite {
    pixels: Vec<ScreenPixel>,
    x: i32,
    y: i32,
}
impl sprite {
    //expects a u8 vector to construct the sprite from
    fn new(sprite: Vec<u8>, x: i32, y: i32) -> sprite {
        //pixels of the sprite, represented as sdl rects
        let mut pixels: Vec<ScreenPixel> = Vec::new();

        //each u8 is a sprite line
        for i in 0..sprite.len() {
            //println!("{:#0b}",sprite[i]);
            //iterate over the bits of the sprite line
            for j in 0..8 {
                //get bit i of line j using bitwise ops
                let temp: u8 = (sprite[i] >> j) & 0x1;

                //if this bit is set, put a square at [x+i][y+j]
                let rectX = (x * 10 + ((7 - j) * 10) as i32) % 640;
                let rectY = (y * 10 + (i * 10) as i32) % 320;

                if temp == 1 {
                    //println!("found bit! adding rect at x = {}, y = {}", rectX, rectY);
                    pixels.push(ScreenPixel::new(Rect::new(rectX, rectY, 10, 10), true));
                } else {
                    pixels.push(ScreenPixel::new(Rect::new(rectX, rectY, 10, 10), false));
                }
            }
        }

        sprite {
            pixels: pixels,
            x: x * 10,
            y: y * 10,
        }
    }
}
//-------------------------------------------------------------------------------------------------

//-----------------------------------------------Sound---------------------------------------------
struct Sound {
    //freq in Hz at which the times should decrease while non-zero
    //should always by 60
    freq: f32,
    DT:u8,
    ST:u8,
    //instant representing the last time the timers were updatd
    //use this in conjunction with .elapsed to update @ 60Hz
    DT_lu: Instant,
    ST_lu: Instant,
}

impl Sound {
    fn new() -> Sound {
        Sound { freq:60.0, DT:0, ST:0,DT_lu: Instant::now(), ST_lu: Instant::now()}
    }
}
//-------------------------------------------------------------------------------------------------

//-----------------------------------------------Input----------------------------------------------
struct Input {
    keys: [bool;16],
}

impl Input {
    fn new() -> Input {
        Input { keys: [false;16]}
    }

    fn update(&mut self, event: sdl2::event::Event) {
            match event{
                Event::KeyDown{keycode: Some(keycodevar), ..} =>
                    match keycodevar{
                        Keycode::Num1 =>self.keys[1] = true,
                        Keycode::Num2 =>self.keys[2] = true,
                        Keycode::Num3 =>self.keys[3] = true,
                        Keycode::Num4 =>self.keys[0xc] = true,
                        Keycode::Q => self.keys[4] = true,
                        Keycode::W => self.keys[5] = true,
                        Keycode::E => self.keys[6] = true,
                        Keycode::R => self.keys[0xd] = true,
                        Keycode::A => self.keys[7] = true,
                        Keycode::S => self.keys[8] = true,
                        Keycode::D => self.keys[9] = true,
                        Keycode::F => self.keys[0xe] = true,
                        Keycode::Z => self.keys[0xa] = true,
                        Keycode::X => self.keys[0] = true,
                        Keycode::C => self.keys[0xb] = true,
                        Keycode::V => self.keys[0xf] = true,
                        _ =>{}
                    },
                Event::KeyUp{keycode: Some(keycodevar), ..} =>
                    match keycodevar{
                        Keycode::Num1 =>self.keys[1] = false,
                        Keycode::Num2 =>self.keys[2] = false,
                        Keycode::Num3 =>self.keys[3] = false,
                        Keycode::Num4 =>self.keys[0xc] = false,
                        Keycode::Q => self.keys[4] = false,
                        Keycode::W => self.keys[5] = false,
                        Keycode::E => self.keys[6] = false,
                        Keycode::R => self.keys[0xd] = false,
                        Keycode::A => self.keys[7] = false,
                        Keycode::S => self.keys[8] = false,
                        Keycode::D => self.keys[9] = false,
                        Keycode::F => self.keys[0xe] = false,
                        Keycode::Z => self.keys[0xa] = false,
                        Keycode::X => self.keys[0] = false,
                        Keycode::C => self.keys[0xb] = false,
                        Keycode::V => self.keys[0xf] = false,
                        _ =>{}
                    },
                _ => {}
        }

    }

    fn dump(&mut self){
        println!("{} {} {} {}",self.keys[1],self.keys[2],self.keys[3],self.keys[0xc]);
        println!("{} {} {} {}",self.keys[4],self.keys[5],self.keys[6],self.keys[0xd]);
        println!("{} {} {} {}",self.keys[7],self.keys[8],self.keys[9],self.keys[0xe]);
        println!("{} {} {} {}",self.keys[0xa],self.keys[0],self.keys[0xb],self.keys[0xf]);
    }
}
//-------------------------------------------------------------------------------------------------

//-----------------------------------------------CPU-----------------------------------------------
struct CPU {
    //PC
    PC: u16,

    //DEPRECATED
    //clock freq in Hz
    //note: we are aiming for 700 instructions per second
    //which works out to 0.0014 sec(1.4ms) per instruction.
    //so the f/d/e cycle should pause at the end to hit this goal if we are not yet longer
    //freq: i32,

    //clock freq in Hz, along with an instant representing the time of the last clock cycle
    //we can fence the cpu to this freq by simply checking the elapsed time every cycle
    freq:f32,
    LCC: Instant,

    //Memory
    mem: Memory,

    //Display,
    disp: Display,

    //Sound
    sound: Sound,

    //Input
    input: Input,

}

impl CPU {
    //cpu initialization
    //sets PC at beginning of ROM, and sets our freq
    fn new(mem: Memory, disp: Display, sound: Sound, input: Input) -> CPU {
        CPU {
            PC: 512,
            //500 Mhz
            //freq: 500_000_000,
            //DEBUG freq 25 Hz for testing
            freq:50.0,
            LCC: Instant::now(),
            mem: mem,
            disp: disp,
            sound: sound,
            input: input
        }
    }

    //get instruction at PC
    fn fetch(&mut self) -> u16 {
        //println!("fetching opcode at {:#X}",self.PC);
        //instructions are two bytes long, highest byte first in memory
        return (self.mem.mem[(self.PC) as usize] as u16) << 8
            | self.mem.mem[(self.PC + 1) as usize] as u16;
    }
    //decode instruction, return fn pointer
    fn decodeAndExecute(&mut self, op: u16) {
        //DEBUG
        //println!("decoding: {:#X}",op);
        print!("{:#x} ",op);

        //first split the func into 4 nibbles
        let n1:u16 = (op & 0b1111000000000000) >> 12;
        let n2:u16 = (op & 0b0000111100000000) >> 8;
        let n3:u16 = (op & 0b0000000011110000) >> 4;
        let n4:u16 = (op & 0b0000000000001111);

        //println!("nibbles: {:#X}, {:#X}, {:#X}, {:#X}",n1,n2,n3,n4);

        //first we match on the highest nibble to get the general opcode typ
        match n1 {
            0x0 => {
                if n2 == 0x0 && n3 == 0xE && n4 == 0x0 {
                    CPU::CLS(self);
                    return;
                }
                if n2 == 0x0 && n3 == 0xE && n4 == 0xE {
                    CPU::RET(self);
                    return;
                }
                CPU::bad_op(self,op);
            }
            0x1 => CPU::JP(self, (n2 << 8 | n3 << 4 | n4)),
            0x2 => CPU::CALL(self, (n2 << 8 | n3 << 4 | n4)),
            0x3 => CPU::SEI(self, n2, (n3<<4 | n4)),
            0x4 => CPU::SNEI(self,n2, (n3<<4 | n4)),
            0x5 => CPU::SER(self),
            0x6 => CPU::LDI(self,n2,(n3<<4|n4)),
            0x7 => CPU::ADDI(self,n2, (n3 <<4 | n4)),
            0x8 => {
                match n4 {
                    0x0 => CPU::LDR(self),
                    0x1 => CPU::OR(self),
                    0x2 => CPU::AND(self),
                    0x3 => CPU::XOR(self),
                    0x4 => CPU::ADDR(self),
                    0x5 => CPU::SUBR(self),
                    0x6 => CPU::SHR(self),
                    0x7 => CPU::SUBN(self),
                    0x8 => CPU::SHL(self),
                    _ => CPU::bad_op(self,op),
                }
            }
            0x9 => CPU::SNER(self),
            0xA => CPU::LDIReg(self, (n2 << 8 | n3 << 4 | n4)),
            0xB => CPU::JPV(self),
            0xC => CPU::RND(self, n2,(n3 <<8 | n4)),
            0xD => CPU::DRW(self,n2,n3,n4),
            0xE => {
                if n3 == 0x9 && n4 == 0xE {
                    CPU::SKP(self,n2);
                    return;
                }
                if n3 == 0xA && n4 == 0x1 {
                    CPU::SKNP(self,n2);
                    return;
                }
                CPU::bad_op(self,op);
            }
            0xF => {
                let lower_byte = (n3<<4 | n4);

                match lower_byte {
                    0x07 => CPU::LDD(self),
                    0x0A => CPU::LDK(self),
                    0x15 => CPU::SD(self,n3),
                    0x18 => CPU::SS(self),
                    0x1E => CPU::ADDIReg(self,n2),
                    0x29 => CPU::LDSprite(self),
                    0x33 => CPU::BCD(self),
                    0x55 => CPU::STSeq(self),
                    0x65 => CPU::LDSeq(self),
                    _ => CPU::bad_op(self, op),
                };
            }

            _ => CPU::bad_op(self, op),
        }

    }


//----------------opcode funcs-------------------
fn test_op(&mut self) {
    println!("opcode called");
    println!("cpu PC: {:#X}", self.PC);
}
fn deprecated_op(&mut self) {
    println!("this opcode is deprecated and im lazy so i didnt implement it");
}
fn bad_op(&mut self, op: u16) {
    println!("BAD OPCODE. {:#x} !!THIS IS BAD!! BAD OPCODE.",op);
}
fn CLS(&mut self) {
    println!("clear screen");
    self.disp.clearDisp();
    self.PC +=2;
}
fn RET( &mut self) {
    println!("return");

    self.PC = self.mem.stack[self.mem.SP as usize];
    //Sprintln!("returning to {:#x}",self.mem.stack[self.mem.SP as usize]);
    self.mem.SP-=1;

}
fn JP(&mut self, addr: u16) {
    println!("JP");
    self.PC = addr;
}
fn CALL( &mut self, addr: u16) {
    println!("CALL: {:#X}",addr);
    self.PC +=2;

    self.mem.SP+=1;
    self.mem.stack[self.mem.SP as usize] = self.PC;
    //println!("stack {:#x} is now {:#x}",self.mem.SP as usize,self.PC);
    self.PC = addr;
}
fn SEI( &mut self, reg: u16, imm: u16) {
    println!("Skip if V{:#x} Equal Lit {:#x}",reg,imm);
    //let bytes = imm.to_be_bytes();
    //if self.mem.Vregs[reg as usize] == bytes[1]
    if self.mem.Vregs[reg as usize] == (imm as u8)
    {
        self.PC +=2;
    }
    self.PC +=2;
}
fn SNEI( &mut self, reg:u16, imm: u16) {
    println!("Skip if V{:#x} Equal Lit {:#x}",reg,imm);

    if self.mem.Vregs[reg as usize] != (imm as u8)
    {
        self.PC +=2;
    }
    self.PC +=2;

}
fn SER( &mut self) {
    println!("Skip Equal Reg");
}
fn LDI( &mut self,reg:u16, byte: u16) {
    println!("Load imm: {:#x} into reg {:#x}",byte, reg);
    let bytes = byte.to_be_bytes();
    self.mem.Vregs[reg as usize] = bytes[1];
    self.PC +=2;
}
fn ADDI(&mut self, reg: u16, byte: u16) {
    println!("Add imm: {:#x} into reg {:#x}",byte,reg);
    let bytes = byte.to_be_bytes();
    let sum:u16 = (self.mem.Vregs[reg as usize] as u16 + bytes[1] as u16) & 0x00FF;
    self.mem.Vregs[reg as usize] = sum.to_be_bytes()[1];
    self.PC +=2;

}
fn LDR( &mut self) {
    println!("Load Reg");
}
fn OR( &mut self) {
    println!("OR");
}
fn AND( &mut self) {
    println!("AND");
}
fn XOR( &mut self) {
    println!("XOR");
}
fn ADDR( &mut self) {
    println!("ADD regs");
}
fn SUBR( &mut self) {
    println!("Sub regs");
}
fn SHR( &mut self) {
    println!("Shift Right");
}
fn SUBN( &mut self) {
    println!("Sub not");
}
fn SHL( &mut self) {
    println!("Shift left");
}
fn SNER( &mut self) {
    println!("Skip not equal registers");
}
fn LDIReg( &mut self, imm: u16) {
    println!("load imm into I register");
    self.mem.I = imm;
    self.PC +=2;
}
fn JPV( &mut self) {
    println!("jump to imm + V0");
}
fn RND( &mut self, reg: u16, imm: u16) {
    println!("rand");
    let mut rng = rand::thread_rng();
    let value:u16 = rng.gen_range(0..256);

    self.mem.Vregs[reg as usize] = (value & imm) as u8;

    self.PC +=2;

}
fn DRW( &mut self, Vx:u16, Vy:u16, length: u16) {
    let x = self.mem.Vregs[Vx as usize];
    let y = self.mem.Vregs[Vy as usize];
    println!("draw! sprite from addr: {:#x}, length: {:#x}, x:{:#x}, y:{:#x}",self.mem.I,length, x,y);

    let mut lines:Vec<u8> = Vec::new();

    for i in 0..length
    {
        let spriteline:u8 = self.mem.mem[self.mem.I as usize];
        //println!("spriteline {}, is {:#x}",i,spriteline);
        lines.push(spriteline);
    }

    let sprite = sprite::new(lines,x.into(),y.into());
    self.disp.pushSprite(sprite);
    self.disp.updateDisp();

    self.PC+=2;
}
fn SKP( &mut self, reg: u16) {
    println!("skip if key {:x} pressed",self.mem.Vregs[reg as usize]);
    //DEBUG: DUMP KEYSTATES
    self.input.dump();

    if self.input.keys[self.mem.Vregs[reg as usize] as usize] == true
    {
        self.PC +=2;
    }
    self.PC +=2;
}
fn SKNP( &mut self, reg: u16) {
    println!("skip if key {:x} NOT pressed",self.mem.Vregs[reg as usize]);
    //DEBUG: DUMP KEYSTATES
    self.input.dump();

    if self.input.keys[self.mem.Vregs[reg as usize] as usize] != true
    {
        self.PC +=2;
    }
    self.PC +=2;
}
fn LDD( &mut self) {
    println!("Load delay timer");
}
fn LDK( &mut self) {
    println!("Load Keystroke");
}
fn SD( &mut self, reg:u16) {
    println!("Set delay timer to contents of V{:x}",reg);

    self.sound.DT = self.mem.Vregs[reg as usize];

    self.PC +=2;

}
fn SS( &mut self) {
    println!("Set Sound timer");
}
fn ADDIReg( &mut self, reg: u16) {
    println!("Add Vx to I reg");
    self.mem.I = self.mem.I + self.mem.Vregs[reg as usize] as u16;
    self.PC +=2;
}
fn LDSprite( &mut self) {
    println!("Set I = location of font sprite Vx")
}
fn BCD( &mut self) {
    println!("Store BCD rep of VX at I");
}
fn STSeq( &mut self) {
    println!("Store all regs at I");
}
fn LDSeq( &mut self) {
    println!("Read all regs from I")
}
//-----------------------------------------------
}
//-------------------------------------------------------------------------------------------------

fn main() {
    //-------------------------------------------SDL setup-----------------------------------------
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    //---------------------------------------------------------------------------------------------

    let video_subsystem = sdl_context.video().unwrap();
    let debugger = video_subsystem.window("debugger",400,300).position(0,0).build().unwrap();
    let mut debug_canvas = debugger.into_canvas().build().unwrap();
    debug_canvas.set_draw_color(Color::RGB(255,0,0));
    debug_canvas.clear();
    debug_canvas.present();


    //------------------------------------User Input-----------------------------------------------
    let args: Vec<String> = env::args().collect();
    //make sure we got a rom filename
    if args.len() != 2 {
        println!("must pass a rom filename!\nExiting.");
        return;
    };

    let filename = "/home/chris/Documents/projects/rust-8/roms/".to_owned() + &args[1];
    //DEBUG: print full filepath to ROM
    //println!("{}",filename);
    //---------------------------------------------------------------------------------------------

    //---------------------------------Component instatiation-------------------------------------
    //Memory - includes regs and rom
    let rom = ROM::new(&filename);
    let mut mem = Memory::new();
    mem.init(&rom);

    //Disp
    let disp = Display::new(sdl_context);

    //Sound
    let sound = Sound::new();

    //Input
    let input = Input::new();

    //---------------------------------------------------------------------------------------------

    //------------------------------------CPU main loop--------------------------------------------
    let mut cpu = CPU::new(mem, disp, sound, input);
    //println!("{:#04x}",cpu.fetch());
    //cpu.decode()();
    //cpu.mem.dump();
    //println!("display: {}",cpu.disp.exists);
    //println!("sound: {}", cpu.sound.exists);
    //println!("input: {}", cpu.input.exists);
    //---------------------------------------------------------------------------------------------

    //DEBUG: testing printing sprites to screen and XOR functionality
    /*let test_sprite:Vec<u8> =   vec![0b11111111,
                                     0b10000001,
                                     0b10000001,
                                     0b10000001,
                                     0b10000001,
                                     0b10000001,
                                     0b10000001,
                                     0b11111111];
    let mut test_sprite_2:Vec<u8> = vec![0;test_sprite.len()];
    test_sprite_2.copy_from_slice(&test_sprite.to_owned());

    let mut test_sprite_3:Vec<u8> = vec![0;test_sprite.len()];
    test_sprite_3.copy_from_slice(&test_sprite.to_owned());

    let mut test_sprite_4:Vec<u8> = vec![0;test_sprite.len()];
    test_sprite_4.copy_from_slice(&test_sprite.to_owned());

    cpu.disp.pushSprite(sprite::new(test_sprite,0,0));
    cpu.disp.pushSprite(sprite::new(test_sprite_2,10,10));
    cpu.disp.pushSprite(sprite::new(test_sprite_3,30,30));
    cpu.disp.pushSprite(sprite::new(test_sprite_4,30,30));*/

    'running: loop {
        //event matching for quiting
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                //on key press or key release, update our input bool struct
                Event::KeyDown{keycode: Some(keycode),..}
                | Event::KeyUp{keycode: Some(keycode),..}
                    => cpu.input.update(event),
                _ => {},
            }
        }


        //execute cpu op
        let raw_op = cpu.fetch();
        print!("{:#03x}: ",cpu.PC);
        cpu.decodeAndExecute(raw_op);



        //fencing for the cpu clock
        while(cpu.LCC.elapsed().as_nanos() < ((1.0/cpu.freq)*1_000_000_000.0) as u128)
        {
            ::std::thread::sleep(Duration::from_nanos(1));
        }
        //update cpu's LCC
        cpu.LCC = Instant::now();

        //fencing for timers
        //TODO: i dont think this is ns accurate. check the math
        if(cpu.sound.ST > 0 && cpu.sound.ST_lu.elapsed().as_nanos() > ((1.0/cpu.sound.freq)*1_000_000_000.0) as u128)
        {
            cpu.sound.ST -=1;
            cpu.sound.ST_lu = Instant::now();
        }

        if(cpu.sound.DT > 0 && cpu.sound.DT_lu.elapsed().as_nanos() > ((1.0/cpu.sound.freq)*1_000_000_000.0) as u128)
        {
            cpu.sound.DT -=1;
            cpu.sound.DT_lu = Instant::now();
        }
    }
    //---------------------------------------------------------------------------------------------
}
