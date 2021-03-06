use crate::rom::ROM;

//---------------------------------------------Memory----------------------------------------------
#[allow(non_snake_case)]
pub struct Memory {
    //memory
    pub mem: [u8; 4096],
    pub program_base: i32,

    //registers
    pub v_regs: [u8; 16],
    pub I: u16,

    //stack
    pub sp: u8,
    pub stack: [u16; 16],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; 4096],
            program_base: 512,
            v_regs: [0; 16],
            I: 0,
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn init(&mut self, rom: &ROM) {
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
    #[allow(dead_code)]
    pub fn dump(&self) {
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

            println!();
        }
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    //takes base and number of lines of 8 bytes to print
    pub fn inspect(&self, base: usize, lines: usize) -> String {
        let mut line = String::new();

        for i in 0..lines {
            for j in 0..8 {
                line.push_str(&*format!(
                    "{:02x}{:02x} ",
                    self.mem[base + (i * 16) + (j * 2)],
                    self.mem[base + (i * 16) + ((j * 2) + 1)],
                ));
            }
        }

        //println!("{}", line);
        return line;
    }
}
//-------------------------------------------------------------------------------------------------
