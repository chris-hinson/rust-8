pub mod debugger{
    use sdl2::Sdl;
    use sdl2::pixels::Color;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::event::EventPollIterator;

    pub struct debugger{
        //pub sdl: Sdl,
        pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    }

    impl debugger{
        pub fn new(sdl_context: &Sdl)->debugger{
            //let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();
            let window = video_subsystem.window("debugger",400,300).position(0,0).build().unwrap();
            let mut debug_canvas = window.into_canvas().build().unwrap();

            debugger{
                canvas: debug_canvas
            }
        }

        pub fn run(&mut self){

            //let mut event_pump = sdl.event_pump().unwrap();

            self.canvas.set_draw_color(Color::GREEN);
            self.canvas.clear();
            self.canvas.present();

            /*loop{
                for event in event_pump.poll_iter(){
                    match event {
                        Event::Quit { .. } | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break ,
                        _ => {},
                    }
                }
            }*/


        }
    }
}
