
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::event::EventPollIterator;
use sdl2::render::TextureQuery;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::rect::Rect;



use crate::cpu::CPU as CPU;

pub struct debugger{
    //pub sdl: Sdl,
    pub live: bool,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub ttf: Sdl2TtfContext,
}

impl debugger{
    pub fn new(sdl_context: &Sdl)->debugger{
        //let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("debugger",400,300).position(0,0).build().unwrap();
        let mut debug_canvas = window.into_canvas().build().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();

        debugger{
            live: false,
            canvas: debug_canvas,
            ttf: ttf_context,
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



            let mut font = self.ttf.load_font("/home/chris/Documents/projects/rust-8/src/FiraCode-Regular.ttf",128).unwrap();
            let texture_creator = self.canvas.texture_creator();

            let text_surface = font.render("String")
                .blended_wrapped(Color::RGBA(255,255,255,0), 320).unwrap();
            let text_texture = texture_creator.create_texture_from_surface(&text_surface).unwrap();

            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();

            self.canvas.copy(&text_texture,None,Some(Rect::new(0,0,120,10)));

            self.canvas.present();

            println!("we are debugging");
            println!("cpu PC is: {:#4x}",cpu.PC);
        }




    }
}
