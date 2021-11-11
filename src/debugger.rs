
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::event::EventPollIterator;
use crate::cpu::CPU as CPU;

pub struct debugger{
    //pub sdl: Sdl,
    pub live: bool,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl debugger{
    pub fn new(sdl_context: &Sdl)->debugger{
        //let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("debugger",400,300).position(0,0).build().unwrap();
        let mut debug_canvas = window.into_canvas().build().unwrap();

        debugger{
            live: false,
            canvas: debug_canvas
        }
    }

    pub fn run(&mut self, pump: &mut sdl2::EventPump, cpu: &mut CPU){

        'running: loop{
            for event in pump.poll_iter(){
                match event {
                    Event::Quit { .. } | Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {self.live = false; break 'running},
                    _ => {},
                }
            }

            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();
            self.canvas.present();

            println!("we are debugging");
            println!("cpu PC is: {:#4x}",cpu.PC);
        }




    }
}
