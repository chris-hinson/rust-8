
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;
//-----------------------------------------------Input----------------------------------------------
pub struct Input {
    pub keys: [bool;16],
    //nessecary for halt until keypress
    pub last_key: u8,
    pub lu: Instant,
}

impl Input {
    pub fn new() -> Input {
        Input { keys: [false;16], last_key: 0 ,lu: Instant::now()}
    }

    pub fn update(&mut self, event: sdl2::event::Event) {
            match event{
                Event::KeyDown{keycode: Some(keycodevar), ..} =>
                    {self.lu = Instant::now();
                    match keycodevar{
                        Keycode::Num1 =>{self.keys[1] = true; self.last_key = 1},
                        Keycode::Num2 =>{self.keys[2] =true; self.last_key = 2},
                        Keycode::Num3 =>{self.keys[3] = true; self.last_key = 3},
                        Keycode::Num4 =>{self.keys[0xc] = true; self.last_key = 0xc},
                        Keycode::Q => {self.keys[4] = true; self.last_key = 4},
                        Keycode::W => {self.keys[5] = true; self.last_key = 5},
                        Keycode::E => {self.keys[6] = true; self.last_key = 6},
                        Keycode::R => {self.keys[0xd] = true; self.last_key = 0xd},
                        Keycode::A => {self.keys[7] = true; self.last_key = 7},
                        Keycode::S => {self.keys[8] = true; self.last_key = 8},
                        Keycode::D => {self.keys[9] = true; self.last_key = 9},
                        Keycode::F => {self.keys[0xe] = true; self.last_key = 0xe},
                        Keycode::Z => {self.keys[0xa] = true; self.last_key = 0xa},
                        Keycode::X => {self.keys[0] = true; self.last_key = 0},
                        Keycode::C => {self.keys[0xb] = true; self.last_key = 0xb},
                        Keycode::V => {self.keys[0xf] = true; self.last_key = 0xf},
                        _ =>{}
                    }},
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

    pub fn dump(&mut self){
        println!("{} {} {} {}",self.keys[1],self.keys[2],self.keys[3],self.keys[0xc]);
        println!("{} {} {} {}",self.keys[4],self.keys[5],self.keys[6],self.keys[0xd]);
        println!("{} {} {} {}",self.keys[7],self.keys[8],self.keys[9],self.keys[0xe]);
        println!("{} {} {} {}",self.keys[0xa],self.keys[0],self.keys[0xb],self.keys[0xf]);
    }
}
//-------------------------------------------------------------------------------------------------
