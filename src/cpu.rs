use rand::Rng;
use std::time::Instant;

//component imports
use crate::display::Display;
use crate::display::Sprite;
use crate::input::Input;
use crate::memory::Memory;
use crate::sound::Sound;
//-----------------------------------------------CPU-----------------------------------------------
pub struct CPU {
    //PC
    pub pc: u16,

    //DEPRECATED
    //clock freq in Hz
    //note: we are aiming for 700 instructions per second
    //which works out to 0.0014 sec(1.4ms) per instruction.
    //so the f/d/e cycle should pause at the end to hit this goal if we are not yet longer
    //freq: i32,

    //clock freq in Hz, along with an instant representing the time of the last clock cycle
    //we can fence the cpu to this freq by simply checking the elapsed time every cycle
    pub freq: f32,
    pub lcc: Instant,

    //Memory
    pub mem: Memory,

    //Display,
    pub disp: Display,

    //Sound
    pub sound: Sound,

    //Input
    pub input: Input,
}

impl CPU {
    //cpu initialization
    //sets PC at beginning of ROM, and sets our freq
    pub fn new(mem: Memory, disp: Display, sound: Sound, input: Input) -> CPU {
        CPU {
            pc: 512,
            //500 Mhz
            //freq: 500_000_000.0,
            //DEBUG freq 50 Hz for testing
            freq: 60.0,
            lcc: Instant::now(),
            mem: mem,
            disp: disp,
            sound: sound,
            input: input,
        }
    }

    //get instruction at PC
    pub fn fetch(&mut self) -> u16 {
        //println!("fetching opcode at {:#X}",self.PC);
        //instructions are two bytes long, highest byte first in memory
        ((self.mem.mem[(self.pc) as usize] as u16) << 8)
            | (self.mem.mem[(self.pc + 1) as usize] as u16)
    }
    //decode instruction, return fn pointer
    pub fn decode_and_execute(&mut self, op: u16) {
        //DEBUG
        //println!("decoding: {:#X}",op);
        print!("{:#x} ", op);

        //first split the func into 4 nibbles
        let n1: u16 = (op & 0b1111000000000000) >> 12;
        let n2: u16 = (op & 0b0000111100000000) >> 8;
        let n3: u16 = (op & 0b0000000011110000) >> 4;
        let n4: u16 = op & 0b0000000000001111;

        //println!("nibbles: {:#X}, {:#X}, {:#X}, {:#X}",n1,n2,n3,n4);

        //first we match on the highest nibble to get the general opcode typ
        match n1 {
            0x0 => {
                if n2 == 0x0 && n3 == 0xE && n4 == 0x0 {
                    CPU::cls(self);
                    return;
                }
                if n2 == 0x0 && n3 == 0xE && n4 == 0xE {
                    CPU::ret(self);
                    return;
                }
                CPU::bad_op(self, op);
            }
            0x1 => CPU::jp(self, n2 << 8 | n3 << 4 | n4),
            0x2 => CPU::call(self, n2 << 8 | n3 << 4 | n4),
            0x3 => CPU::sei(self, n2, n3 << 4 | n4),
            0x4 => CPU::snei(self, n2, n3 << 4 | n4),
            0x5 => CPU::ser(self, n2, n3),
            0x6 => CPU::ldi(self, n2, n3 << 4 | n4),
            0x7 => CPU::addi(self, n2, n3 << 4 | n4),
            0x8 => match n4 {
                0x0 => CPU::ldr(self, n2, n3),
                0x1 => CPU::or(self, n2, n3),
                0x2 => CPU::and(self, n2, n3),
                0x3 => CPU::xor(self, n2, n3),
                0x4 => CPU::addr(self, n2, n3),
                0x5 => CPU::subr(self, n2, n3),
                0x6 => CPU::shr(self, n2),
                0x7 => CPU::subn(self, n2, n3),
                0xE => CPU::shl(self, n2),
                _ => CPU::bad_op(self, op),
            },
            0x9 => CPU::sner(self, n2, n3),
            0xA => CPU::ldireg(self, n2 << 8 | n3 << 4 | n4),
            0xB => CPU::jpv(self, n2 << 8 | n3 << 4 | n4),
            0xC => CPU::rnd(self, n2, n3 << 4 | n4),
            0xD => CPU::drw(self, n2, n3, n4),
            0xE => {
                if n3 == 0x9 && n4 == 0xE {
                    CPU::skp(self, n2);
                    return;
                }
                if n3 == 0xA && n4 == 0x1 {
                    CPU::sknp(self, n2);
                    return;
                }
                CPU::bad_op(self, op);
            }
            0xF => {
                let lower_byte = n3 << 4 | n4;

                match lower_byte {
                    0x07 => CPU::ldd(self, n2),
                    0x0A => CPU::ldk(self, n2),
                    0x15 => CPU::sd(self, n2),
                    0x18 => CPU::ss(self, n2),
                    0x1E => CPU::addireg(self, n2),
                    0x29 => CPU::ldsprite(self, n2),
                    0x33 => CPU::bcd(self, n2),
                    0x55 => CPU::stseq(self, n2),
                    0x65 => CPU::ldseq(self, n2),
                    _ => CPU::bad_op(self, op),
                };
            }

            _ => CPU::bad_op(self, op),
        }
    }

    //----------------opcode funcs-------------------
    /*fn test_op(&mut self) {
        println!("opcode called");
        println!("cpu PC: {:#X}", self.pc);
    }
    fn deprecated_op(&mut self) {
        println!("this opcode is deprecated and im lazy so i didnt implement it");
    }*/
    fn bad_op(&mut self, op: u16) {
        println!("BAD OPCODE. {:#x} !!THIS IS BAD!! BAD OPCODE.", op);
    }
    fn cls(&mut self) {
        println!("clear screen");
        self.disp.clear_disp();
        self.pc += 2;
    }
    fn ret(&mut self) {
        println!("return");

        self.pc = self.mem.stack[self.mem.sp as usize];
        //Sprintln!("returning to {:#x}",self.mem.stack[self.mem.SP as usize]);
        self.mem.sp -= 1;
    }
    fn jp(&mut self, addr: u16) {
        println!("JP to addr: {:x}", addr);
        self.pc = addr;
    }
    fn call(&mut self, addr: u16) {
        println!("CALL: {:#X}", addr);
        self.pc += 2;

        self.mem.sp += 1;
        self.mem.stack[self.mem.sp as usize] = self.pc;
        //println!("stack {:#x} is now {:#x}",self.mem.SP as usize,self.pc);
        self.pc = addr;
    }
    fn sei(&mut self, reg: u16, imm: u16) {
        println!("Skip if V{:#x} Equal Lit {:#x}", reg, imm);
        //let bytes = imm.to_be_bytes();
        //if self.mem.v_regs[reg as usize] == bytes[1]
        if self.mem.v_regs[reg as usize] == (imm as u8) {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn snei(&mut self, reg: u16, imm: u16) {
        println!("Skip if V{:#x} Equal Lit {:#x}", reg, imm);

        if self.mem.v_regs[reg as usize] != (imm as u8) {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn ser(&mut self, reg1: u16, reg2: u16) {
        println!("Skip if V{:x} = V{:x}", reg1, reg2);

        if self.mem.v_regs[reg1 as usize] == self.mem.v_regs[reg2 as usize] {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn ldi(&mut self, reg: u16, byte: u16) {
        println!("Load imm: {:#x} into reg {:#x}", byte, reg);
        let bytes = byte.to_be_bytes();
        self.mem.v_regs[reg as usize] = bytes[1];
        self.pc += 2;
    }
    fn addi(&mut self, reg: u16, byte: u16) {
        println!("Add imm: {:#x} into reg {:#x}", byte, reg);
        let bytes = byte.to_be_bytes();
        let sum: u16 = (self.mem.v_regs[reg as usize] as u16 + bytes[1] as u16) & 0x00FF;
        self.mem.v_regs[reg as usize] = sum.to_be_bytes()[1];
        self.pc += 2;
    }
    fn ldr(&mut self, reg1: u16, reg2: u16) {
        println!("Set V{:x} = V{:x}", reg1, reg2);
        self.mem.v_regs[reg1 as usize] = self.mem.v_regs[reg2 as usize];
        self.pc += 2;
    }
    fn or(&mut self, reg1: u16, reg2: u16) {
        println!("V{:x} = V{:x} | V{:x}", reg1, reg1, reg2);
        self.mem.v_regs[reg1 as usize] |= self.mem.v_regs[reg2 as usize];
        self.pc += 2;
    }
    fn and(&mut self, reg1: u16, reg2: u16) {
        println!("V{:x} = V{:x} & V{:x}", reg1, reg1, reg2);
        self.mem.v_regs[reg1 as usize] &= self.mem.v_regs[reg2 as usize];
        self.pc += 2;
    }
    fn xor(&mut self, reg1: u16, reg2: u16) {
        println!("V{:x} = V{:x} ^ V{:x}", reg1, reg1, reg2);
        self.mem.v_regs[reg1 as usize] ^= self.mem.v_regs[reg2 as usize];
        self.pc += 2;
    }
    fn addr(&mut self, reg1: u16, reg2: u16) {
        println!("V{:x} = V{:x} + V{:x}", reg1, reg1, reg2);
        if ((self.mem.v_regs[reg1 as usize] as i32) + (self.mem.v_regs[reg2 as usize] as i32)) > 255
        {
            println!("overflow! setting vf = 1");
            self.mem.v_regs[0xF] = 1;
        }
        self.mem.v_regs[reg1 as usize] = ((self.mem.v_regs[reg1 as usize] as i32
            + self.mem.v_regs[reg2 as usize] as i32)
            % 256) as u8;
        self.pc += 2;
    }
    fn subr(&mut self, reg1: u16, reg2: u16) {
        println!("V{:x} = V{:x} - V{:x}", reg1, reg1, reg2);
        if (self.mem.v_regs[reg1 as usize] as i32) > (self.mem.v_regs[reg2 as usize] as i32) {
            println!("underflow! setting vf = 1");
            self.mem.v_regs[0xF] = 1;
        } else {
            self.mem.v_regs[0xF] = 0;
        }
        self.mem.v_regs[reg1 as usize] = ((self.mem.v_regs[reg1 as usize] as i32
            - self.mem.v_regs[reg2 as usize] as i32)
            % 256) as u8;
        self.pc += 2;
    }
    fn shr(&mut self, reg: u16) {
        println!("Shift V{:x} Right 1", reg);
        let lsb = self.mem.v_regs[reg as usize] & 0x1;
        self.mem.v_regs[0xF] = lsb;
        self.mem.v_regs[reg as usize] >>= self.mem.v_regs[reg as usize];
        self.pc += 2;
    }
    fn subn(&mut self, reg1: u16, reg2: u16) {
        println!("V{:x} = V{:x} - V{:x}", reg1, reg2, reg1);
        if (self.mem.v_regs[reg2 as usize] as i32) > (self.mem.v_regs[reg1 as usize] as i32) {
            println!("underflow! setting vf = 1");
            self.mem.v_regs[0xF] = 1;
        } else {
            self.mem.v_regs[0xF] = 0;
        }
        self.mem.v_regs[reg1 as usize] = ((self.mem.v_regs[reg2 as usize] as i32
            - self.mem.v_regs[reg1 as usize] as i32)
            % 255) as u8;
        self.pc += 2;
    }
    fn shl(&mut self, reg: u16) {
        println!("Shift V{:x} Left 1", reg);
        let msb = self.mem.v_regs[reg as usize] & 0b10000000;
        self.mem.v_regs[15] = msb;
        self.mem.v_regs[reg as usize] <<= self.mem.v_regs[reg as usize];
        self.pc += 2;
    }
    fn sner(&mut self, reg1: u16, reg2: u16) {
        println!("Skip if V{:x} != V{:x}", reg1, reg2);

        if self.mem.v_regs[reg1 as usize] != self.mem.v_regs[reg2 as usize] {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn ldireg(&mut self, imm: u16) {
        println!("load imm into I register");
        self.mem.I = imm;
        self.pc += 2;
    }
    fn jpv(&mut self, imm: u16) {
        println!("jump to {:x} + V0", imm);
        self.pc = self.mem.v_regs[0] as u16 + imm;
    }
    fn rnd(&mut self, reg: u16, imm: u16) {
        println!("rand");
        let mut rng = rand::thread_rng();
        let value: u16 = rng.gen_range(0..256);

        self.mem.v_regs[reg as usize] = (value & imm) as u8;

        self.pc += 2;
    }
    fn drw(&mut self, vx: u16, vy: u16, length: u16) {
        let x = self.mem.v_regs[vx as usize];
        let y = self.mem.v_regs[vy as usize];
        println!(
            "draw! sprite from addr: {:#x}, length: {:#x}, x:{:#x}, y:{:#x}",
            self.mem.I, length, x, y
        );

        let mut lines: Vec<u8> = Vec::new();

        for _i in 0..length {
            let spriteline: u8 = self.mem.mem[self.mem.I as usize];
            //println!("spriteline {}, is {:#x}",i,spriteline);
            lines.push(spriteline);
        }

        let sprite = Sprite::new(lines, x.into(), y.into());
        self.disp.push_sprite(sprite);
        self.disp.update_disp();

        self.pc += 2;
    }
    fn skp(&mut self, reg: u16) {
        println!("skip if key {:x} pressed", self.mem.v_regs[reg as usize]);
        //DEBUG: DUMP KEYSTATES
        self.input.dump();

        if self.input.keys[self.mem.v_regs[reg as usize] as usize] {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn sknp(&mut self, reg: u16) {
        println!(
            "skip if key {:x} NOT pressed",
            self.mem.v_regs[reg as usize]
        );
        //DEBUG: DUMP KEYSTATES
        self.input.dump();

        if !self.input.keys[self.mem.v_regs[reg as usize] as usize] {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn ldd(&mut self, reg: u16) {
        println!("set V{:x} = delay timer", reg);

        self.sound.dt = self.mem.v_regs[reg as usize];
        self.sound.dt_lu = Instant::now();

        self.pc += 2;
    }
    fn ldk(&mut self, reg: u16) {
        println!("Load Keystroke into vx");
        //TODO: fix me jesus christ this is beyond hacky

        //to halt execution until a key is pressed, do not increment pc until a change happens in keys
        //we approximate this by only updating if key_lu < 0.5 seconds in the past
        if self.input.lu.elapsed().as_millis() < 500 {
            self.mem.v_regs[reg as usize] = self.input.last_key;
            self.pc += 2;
        }
    }
    fn sd(&mut self, reg: u16) {
        println!("Set delay timer to contents of V{:x}", reg);

        self.sound.dt = self.mem.v_regs[reg as usize];
        self.sound.dt_lu = Instant::now();

        self.pc += 2;
    }
    fn ss(&mut self, reg: u16) {
        println!("Set sound timer to contents of V{:x}", reg);

        self.sound.st = self.mem.v_regs[reg as usize];
        self.sound.st_lu = Instant::now();

        self.pc += 2;
    }
    fn addireg(&mut self, reg: u16) {
        println!("Add Vx to I reg");
        self.mem.I += self.mem.v_regs[reg as usize] as u16;
        self.pc += 2;
    }
    fn ldsprite(&mut self, reg: u16) {
        println!("Set I = location of font sprite V{:x}", reg);

        self.mem.I = (5 * self.mem.v_regs[reg as usize]).into();
        self.pc += 2;
    }
    fn bcd(&mut self, reg: u16) {
        println!("Store BCD rep of V{:x} at I", reg);
        let value: i32 = self.mem.v_regs[reg as usize].into();
        let hundreds: i32 = value / 100;
        let tens: i32 = (value % 100) / 10;
        let ones: i32 = value % 10;

        self.mem.mem[self.mem.I as usize] = hundreds as u8;
        self.mem.mem[(self.mem.I + 1) as usize] = tens as u8;
        self.mem.mem[(self.mem.I + 2) as usize] = ones as u8;

        self.pc += 2;
    }
    fn stseq(&mut self, reg: u16) {
        println!("Store Vregs 0 through V{:x} at I", reg);

        for i in 0..reg + 1 {
            self.mem.mem[(self.mem.I + i) as usize] = self.mem.v_regs[i as usize];
        }

        self.pc += 2;
    }
    fn ldseq(&mut self, reg: u16) {
        println!("Read Vregs 0 through V{:x} from I", reg);

        for i in 0..reg + 1 {
            self.mem.v_regs[i as usize] = self.mem.mem[(self.mem.I + i) as usize];
        }

        self.pc += 2;
    }
    //-----------------------------------------------
}
//-------------------------------------------------------------------------------------------------
