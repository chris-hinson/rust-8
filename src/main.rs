extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


use std::time::Duration;
use std::env;

use std::time::Instant;


//modules for components
mod debugger;
use crate::debugger::Debugger as Debug;

mod rom;
use crate::rom::ROM as ROM;

mod memory;
use crate::memory::Memory as Memory;

mod display;
use crate::display::Display as Display;

mod sound;
use crate::sound::Sound as Sound;

mod input;
use crate::input::Input as Input;

mod cpu;
use crate::cpu::CPU as CPU;


fn main() {
    //-------------------------------------------SDL setup-----------------------------------------
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    //---------------------------------------------------------------------------------------------




    //------------------------------------User Input-----------------------------------------------
    let args: Vec<String> = env::args().collect();
    //make sure we got a rom filename
    if args.len() < 2 {
        println!("must pass a rom filename!\nExiting.");
        return;
    };

    let mut start_debugging = false;
    if args.len() == 3{
        start_debugging = true;
    }

    let filename = "/home/chris/Documents/projects/rust-8/roms/".to_owned() + &args[1];
    //DEBUG: print full filepath to ROM
    //println!("{}",filename);
    //---------------------------------------------------------------------------------------------

    //---------------------------------Component instatiation-------------------------------------
    //debugger
    let mut debugger = Debug::new(&sdl_context);
    if start_debugging
    {
        debugger.live = true;
    }

    //Memory - includes regs and rom
    let rom = ROM::new(&filename);
    let mut mem = Memory::new();
    mem.init(&rom);

    //Disp
    let disp = Display::new(&sdl_context);

    //Sound
    let sound = Sound::new();

    //Input
    let input = Input::new();

    //---------------------------------------------------------------------------------------------

    //------------------------------------CPU main loop--------------------------------------------
    let mut cpu = CPU::new(mem, disp, sound, input);

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown{keycode: Some(Keycode::Space),..} => {debugger.live = true; break},
                //on key press or key release, update our input bool struct
                Event::KeyDown{..}
                | Event::KeyUp{..}
                    => cpu.input.update(event),
                _ => {},
            }
        }

        if debugger.live
        {
            println!("going to debugger");
            debugger.run(&mut event_pump, &mut cpu);
        }


        //execute cpu op
        let raw_op = cpu.fetch();
        print!("{:#03x}: ",cpu.pc);
        cpu.decode_and_execute(raw_op);



        //fencing for the cpu clock
        while cpu.lcc.elapsed().as_nanos() < ((1.0/cpu.freq)*1_000_000_000.0) as u128
        {
            ::std::thread::sleep(Duration::from_nanos(1));
        }
        //update cpu's LCC
        cpu.lcc = Instant::now();

        //fencing for timers
        //TODO: i dont think this is ns accurate. check the math
        if cpu.sound.st > 0 && cpu.sound.st_lu.elapsed().as_nanos() > ((1.0/cpu.sound.freq)*1_000_000_000.0) as u128
        {
            cpu.sound.st -=1;
            cpu.sound.st_lu = Instant::now();
        }

        if cpu.sound.dt > 0 && cpu.sound.dt_lu.elapsed().as_nanos() > ((1.0/cpu.sound.freq)*1_000_000_000.0) as u128
        {
            cpu.sound.dt -=1;
            cpu.sound.dt_lu = Instant::now();
        }
    }
    //---------------------------------------------------------------------------------------------
}
