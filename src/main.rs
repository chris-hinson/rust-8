extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::env;
use std::time::Duration;

use std::time::Instant;

//modules for components
mod debugger;
use crate::debugger::Debugger as Debug;

mod rom;
use crate::rom::ROM;

mod memory;
use crate::memory::Memory;

mod display;
use crate::display::Display;
use crate::display::Sprite;

mod sound;
use crate::sound::Sound;

mod input;
use crate::input::Input;

mod cpu;
use crate::cpu::CPU;

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
    if args.len() == 3 {
        start_debugging = true;
    }

    //let filename = "/home/chris/Documents/projects/rust-8/roms/".to_owned() + &args[1];
    let filename = "./".to_owned() + &args[1];
    //DEBUG: print full filepath to ROM
    //println!("{}",filename);
    //---------------------------------------------------------------------------------------------

    //---------------------------------Component instatiation-------------------------------------
    //debugger
    let mut debugger = Debug::new(&sdl_context);
    if start_debugging {
        debugger.live = true;
    }

    //Memory - includes regs and rom
    let rom = ROM::new(&filename);
    let mut mem = Memory::new();
    mem.init(&rom);
    //mem.dump();

    //Disp
    let disp = Display::new(&sdl_context);

    //DEBUG push a sprite and manually refresh display

    //Sound
    let sound = Sound::new();

    //Input
    let input = Input::new();

    //---------------------------------------------------------------------------------------------

    //------------------------------------CPU main loop--------------------------------------------
    let mut cpu = CPU::new(mem, disp, sound, input);

    let mut test_sprite: Vec<u8> = Vec::new();
    test_sprite.push(0b11111111);
    test_sprite.push(0b10000001);
    test_sprite.push(0b10000001);
    test_sprite.push(0b11111111);
    let test_sprite_2: Vec<u8> = test_sprite.clone();
    println!("test sprite 1: {:?}", test_sprite);
    println!("test sprite 2: {:?}", test_sprite_2);

    println!("drawing test sprite");
    cpu.disp
        .push_sprite(Sprite::new(test_sprite, 0, 0), &mut cpu.mem);
    cpu.disp.update_disp();
    //::std::thread::sleep(Duration::from_secs(5));

    /*println!("drawing sprite again to test xor");
    cpu.disp
        .push_sprite(Sprite::new(test_sprite_2, 0, 0), &mut cpu.mem);
    cpu.disp.update_disp();
    ::std::thread::sleep(Duration::from_secs(5));*/

    println!("clearing display!");
    cpu.disp.clear_disp();
    //::std::thread::sleep(Duration::from_secs(5));

    for i in 0..32 {
        for j in 0..64 {
            assert_eq!(cpu.disp.pixels[i][j].state, false);
        }
    }

    println!("calling update to make sure we actually cleared");
    cpu.disp.update_disp();
    //::std::thread::sleep(Duration::from_secs(5));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    debugger.live = true;
                    break;
                }
                //on key press or key release, update our input bool struct
                Event::KeyDown { .. } | Event::KeyUp { .. } => cpu.input.update(event),
                _ => {}
            }
        }

        if debugger.live {
            println!("going to debugger");
            debugger.run(&mut event_pump, &mut cpu);
        }

        //execute cpu op
        let raw_op = cpu.fetch();
        print!("{:#03x}: ", cpu.pc);
        cpu.decode_and_execute(raw_op);
        if cpu.crashed {
            //debugger.live = true;
            //debugger.run(&mut event_pump, &mut cpu);
            break 'running;
        }

        //this executes once per clock cycle
        //always increment the #clock cycles since last update
        cpu.sound.dt_lu += 1;
        //if we have gone through 500/60 = 8.33  cycles, and the reg is non-zero, decrement
        if cpu.sound.dt_lu as f32 >= (cpu.freq / 60.0) as f32 && cpu.sound.dt > 0 {
            //println!("WE ARE DECREMENTING DT");
            //::std::thread::sleep(Duration::from_secs(5));
            cpu.sound.dt -= 1;
        }
        //println!("dt is {} and dt_lu is {}", cpu.sound.dt, cpu.sound.dt_lu);

        cpu.sound.st_lu += 1;
        if cpu.sound.st_lu as f32 >= (cpu.freq / 60.0) as f32 && cpu.sound.st > 0 {
            cpu.sound.st -= 1;
        }

        //fencing for the cpu clock
        //if it has not been 1/freq * 1.0x10^9 seconds since the last cycle,wait a ns until it is
        while cpu.lcc.elapsed().as_nanos() < ((1.0 / cpu.freq) * 1_000_000_000.0) as u128 {
            ::std::thread::sleep(Duration::from_nanos(1));
        }
        //update cpu's LCC
        cpu.lcc = Instant::now();
    }
    //---------------------------------------------------------------------------------------------
}
