pub mod display{
    use sdl2::Sdl;
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;

    //-----------------------------------------------Display-------------------------------------------
    pub struct Display {
        pub pixels: [[ScreenPixel; 64]; 32],
        //NOTE: nessecary for display to actually draw things
        pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    }

    impl Display {
        pub fn new(sdl_context: &Sdl) -> Display {
            let mut pixels: [[ScreenPixel; 64]; 32] =
                [[ScreenPixel::new(Rect::new(0, 0, 10, 10), false); 64]; 32];

            //TODO: can we do this in the initializer?
            for i in 0..32 {
                for j in 0..64 {
                    pixels[i][j].pixel.x = (j * 10) as i32;
                    pixels[i][j].pixel.y = (i * 10) as i32;
                }
            }

            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem
                .window("rust-sdl2 demo", 640, 320)
                .position_centered()
                .build()
                .unwrap();

            let mut canvas = window.into_canvas().build().unwrap();

            Display {
                pixels: pixels,
                canvas: canvas
            }
        }

        pub fn updateDisp(&mut self) {
            for row in self.pixels {
                for pixel in row {
                    if pixel.state {
                        self.canvas.set_draw_color(Color::RED);
                        self.canvas.fill_rect(pixel.pixel);
                    }
                    else{
                        self.canvas.set_draw_color(Color::BLACK);
                        self.canvas.fill_rect(pixel.pixel);
                    }
                }
            }
            self.canvas.present();
        }

        pub fn clearDisp(&mut self) {
            self.canvas.set_draw_color(Color::BLACK);
            for row in self.pixels {
                for mut pixel in row {
                    pixel.state = false;
                    self.canvas.fill_rect(pixel.pixel);
                }
            }
            self.canvas.present();
        }

        //XORs a sprite into display buffer
        pub fn pushSprite(&mut self, sprite: sprite) {
            for pixel in sprite.pixels {
                let x = (pixel.pixel.x / 10) as usize;
                let y = (pixel.pixel.y / 10) as usize;
                //println!("XORing pixel at x:{}, y:{}", x, y);
                self.pixels[y][x].state = self.pixels[y][x].state ^ pixel.state;
                //println!("pixel at x:{}, y:{} is now: {}", xIndex, yIndex, self.pixels[xIndex][yIndex].state);
            }
        }
    }
    //----------------------
    //represents a pixels as a Rect so SDL can draw it and a boolean to represent if its on or not
    #[derive(Copy, Clone)]
    pub struct ScreenPixel {
        pub pixel: Rect,
        pub state: bool,
    }
    impl ScreenPixel {
        pub fn new(pixel: Rect, state: bool) -> ScreenPixel {
            ScreenPixel {
                pixel: pixel,
                state: state,
            }
        }
    }
    //----------------------

    pub struct sprite {
        pub pixels: Vec<ScreenPixel>,
        pub x: i32,
        pub y: i32,
    }
    impl sprite {
        //expects a u8 vector to construct the sprite from
        pub fn new(sprite: Vec<u8>, x: i32, y: i32) -> sprite {
            //pixels of the sprite, represented as sdl rects
            let mut pixels: Vec<ScreenPixel> = Vec::new();

            //each u8 is a sprite line
            for i in 0..sprite.len() {
                //println!("{:#0b}",sprite[i]);
                //iterate over the bits of the sprite line
                for j in 0..8 {
                    //get bit i of line j using bitwise ops
                    let temp: u8 = (sprite[i] >> j) & 0x1;

                    //if this bit is set, put a square at [x+i][y+j]
                    let rectX = (x * 10 + ((7 - j) * 10) as i32) % 640;
                    let rectY = (y * 10 + (i * 10) as i32) % 320;

                    if temp == 1 {
                        //println!("found bit! adding rect at x = {}, y = {}", rectX, rectY);
                        pixels.push(ScreenPixel::new(Rect::new(rectX, rectY, 10, 10), true));
                    } else {
                        pixels.push(ScreenPixel::new(Rect::new(rectX, rectY, 10, 10), false));
                    }
                }
            }

            sprite {
                pixels: pixels,
                x: x * 10,
                y: y * 10,
            }
        }
    }
    //-------------------------------------------------------------------------------------------------
}
