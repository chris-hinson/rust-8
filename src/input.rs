pub mod input{
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    //-----------------------------------------------Input----------------------------------------------
    pub struct Input {
        pub keys: [bool;16],
    }

    impl Input {
        pub fn new() -> Input {
            Input { keys: [false;16]}
        }

        pub fn update(&mut self, event: sdl2::event::Event) {
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

        pub fn dump(&mut self){
            println!("{} {} {} {}",self.keys[1],self.keys[2],self.keys[3],self.keys[0xc]);
            println!("{} {} {} {}",self.keys[4],self.keys[5],self.keys[6],self.keys[0xd]);
            println!("{} {} {} {}",self.keys[7],self.keys[8],self.keys[9],self.keys[0xe]);
            println!("{} {} {} {}",self.keys[0xa],self.keys[0],self.keys[0xb],self.keys[0xf]);
        }
    }
    //-------------------------------------------------------------------------------------------------
}
