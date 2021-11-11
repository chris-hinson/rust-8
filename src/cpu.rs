pub mod cpu{
    use std::time::Instant;
    use rand::Rng;

    //component imports
    use crate::rom::rom::ROM as ROM;
    use crate::memory::memory::Memory as Memory;
    use crate::display::display::Display as Display;
    use crate::display::display::sprite as sprite;
    use crate::sound::sound::Sound as Sound;
    use crate::input::input::Input as Input;
    //-----------------------------------------------CPU-----------------------------------------------
    pub struct CPU {
        //PC
        pub PC: u16,

        //DEPRECATED
        //clock freq in Hz
        //note: we are aiming for 700 instructions per second
        //which works out to 0.0014 sec(1.4ms) per instruction.
        //so the f/d/e cycle should pause at the end to hit this goal if we are not yet longer
        //freq: i32,

        //clock freq in Hz, along with an instant representing the time of the last clock cycle
        //we can fence the cpu to this freq by simply checking the elapsed time every cycle
        pub freq:f32,
        pub LCC: Instant,

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
        pub fn fetch(&mut self) -> u16 {
            //println!("fetching opcode at {:#X}",self.PC);
            //instructions are two bytes long, highest byte first in memory
            return (self.mem.mem[(self.PC) as usize] as u16) << 8
                | self.mem.mem[(self.PC + 1) as usize] as u16;
        }
        //decode instruction, return fn pointer
        pub fn decodeAndExecute(&mut self, op: u16) {
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
}
