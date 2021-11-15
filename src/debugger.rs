
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::event::EventPollIterator;
use sdl2::render::TextureQuery;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::rect::Rect;
use sdl2::rect::Point;

use std::time::Instant;

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
        let window = video_subsystem.window("debugger",600,600).position(0,0).build().unwrap();
        let mut debug_canvas = window.into_canvas().build().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();

        debugger{
            live: false,
            canvas: debug_canvas,
            ttf: ttf_context,
        }
    }

    pub fn run(&mut self, pump: &mut sdl2::EventPump, cpu: &mut CPU){

        let mut run:bool = false;
        let mut font = self.ttf.load_font("/home/chris/Documents/projects/rust-8/src/FiraCode-Regular.ttf",128).unwrap();
        let texture_creator = self.canvas.texture_creator();

        'running: loop{
            //clear debug screen at beginning of every loop iteration
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();
            self.canvas.set_draw_color(Color::WHITE);

            //draw current pc and instruction
            let raw_op = cpu.fetch();
            let pc_str = format!("{:#04x}: {:04x}",cpu.PC, raw_op);
            let text_surface = font.render(&pc_str).solid(Color::RGBA(255,255,255,0)).unwrap();
            let text_texture = texture_creator.create_texture_from_surface(&text_surface).unwrap();
            self.canvas.copy(&text_texture,None,Some(Rect::new(0,0,100,25)));
            self.canvas.draw_line(Point::new(0,26),Point::new(101,26));
            self.canvas.draw_line(Point::new(101,26),Point::new(101,0));

            //draw Vregs
            for (i,x) in cpu.mem.Vregs.iter().enumerate()
            {
                let value = format!("V{}: {:0>8b} : {:#x}",i,x,x);
                let cur_vreg_surface = font.render(&value).blended(Color::WHITE).unwrap();
                let cur_vreg_texture = texture_creator.create_texture_from_surface(&cur_vreg_surface).unwrap();
                self.canvas.copy(&cur_vreg_texture,None,Some(Rect::new(0,(26 + (15 * i+1)) as i32,150,15)));
            }
            self.canvas.draw_line(Point::new(0,26),Point::new(151,26));
            self.canvas.draw_line(Point::new(151,26),Point::new(151,266));
            //I reg
            let value = format!("I: {:0>16b} : {:#x}",cpu.mem.I,cpu.mem.I);
            let cur_Ireg_surface = font.render(&value).solid(Color::WHITE).unwrap();
            let cur_Ireg_texture = texture_creator.create_texture_from_surface(&cur_Ireg_surface).unwrap();
            self.canvas.copy(&cur_Ireg_texture,None,Some(Rect::new(0,266,250,15)));
            //timers
            let value = format!("DT: {:0>8b} : {:#x}",cpu.sound.DT,cpu.sound.DT);
            let cur_DT_surface = font.render(&value).solid(Color::WHITE).unwrap();
            let cur_DT_texture = texture_creator.create_texture_from_surface(&cur_DT_surface).unwrap();
            self.canvas.copy(&cur_DT_texture,None,Some(Rect::new(0,282,150,15)));
            let value = format!("ST: {:0>8b} : {:#x}",cpu.sound.ST,cpu.sound.ST);
            let cur_ST_surface = font.render(&value).solid(Color::WHITE).unwrap();
            let cur_ST_texture = texture_creator.create_texture_from_surface(&cur_ST_surface).unwrap();
            self.canvas.copy(&cur_ST_texture,None,Some(Rect::new(0,297,150,15)));


            //process all events in queue
            for event in pump.poll_iter(){
                match event {
                    Event::Quit { .. } | Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        //set debugger to dead and update timers before returning to main function
                        self.live = false;
                        cpu.LCC = Instant::now();
                        cpu.sound.DT_lu = Instant::now();
                        cpu.sound.ST_lu = Instant::now();
                        break 'running},
                    Event::KeyDown{keycode: Some(Keycode::N),..} => run = true,
                    _ => {},
                }
            }

            //if we hit n key, want to run one cpu cycle
            if run
            {
                let op = cpu.fetch();
                cpu.decodeAndExecute(op);

                //ST and DT should update cpu_freq/60 times per cpu cycle
                if cpu.sound.ST > 0
                {
                    cpu.sound.ST -= (cpu.freq/60.0) as u8;
                }
                if cpu.sound.DT > 0
                {
                    cpu.sound.DT -= (cpu.freq/60.0) as u8;
                }
                run = false;
            }


            self.canvas.present();

        }




    }
}
