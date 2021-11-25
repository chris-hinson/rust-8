use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::Sdl;

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
        //for i in 0..32 {
        for (i, row) in pixels.iter_mut().enumerate() {
            //for j in 0..64 {
            for (j, column) in row.iter_mut().enumerate() {
                column.pixel.x = (j * 10) as i32;
                column.pixel.y = (i * 10) as i32;
            }
        }

        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Display {
            pixels: pixels,
            canvas: canvas,
        }
    }

    pub fn update_disp(&mut self) {
        for row in self.pixels {
            for pixel in row {
                if pixel.state {
                    self.canvas.set_draw_color(Color::RED);
                } else {
                    self.canvas.set_draw_color(Color::BLACK);
                }
                self.canvas.fill_rect(pixel.pixel).unwrap();
            }
        }
        self.canvas.present();
    }

    pub fn clear_disp(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        for row in self.pixels {
            for mut pixel in row {
                pixel.state = false;
                self.canvas.fill_rect(pixel.pixel).unwrap();
            }
        }
        self.canvas.present();
    }

    //XORs a sprite into display buffer
    pub fn push_sprite(&mut self, sprite: Sprite) {
        for pixel in sprite.pixels {
            let x = (pixel.pixel.x / 10) as usize;
            let y = (pixel.pixel.y / 10) as usize;
            //println!("XORing pixel at x:{}, y:{}", x, y);
            self.pixels[y][x].state ^= pixel.state;
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

pub struct Sprite {
    pub pixels: Vec<ScreenPixel>,
    pub x: i32,
    pub y: i32,
}
impl Sprite {
    //expects a u8 vector to construct the sprite from
    pub fn new(sprite: Vec<u8>, x: i32, y: i32) -> Sprite {
        //pixels of the sprite, represented as sdl rects
        let mut pixels: Vec<ScreenPixel> = Vec::new();

        //each u8 is a sprite line
        //for i in 0..sprite.len() {
        for (i, line) in sprite.iter().enumerate() {
            //println!("{:#0b}",sprite[i]);
            //iterate over the bits of the sprite line
            for j in 0..8 {
                //for (j, bit) in line.iter().enumerate() {
                //get bit i of line j using bitwise ops
                let temp: u8 = (line >> j) & 0x1;

                //if this bit is set, put a square at [x+i][y+j]
                let rect_x = (x * 10 + ((7 - j) * 10) as i32) % 640;
                let rect_y = (y * 10 + (i * 10) as i32) % 320;

                if temp == 1 {
                    //println!("found bit! adding rect at x = {}, y = {}", rectX, rectY);
                    pixels.push(ScreenPixel::new(Rect::new(rect_x, rect_y, 10, 10), true));
                } else {
                    pixels.push(ScreenPixel::new(Rect::new(rect_x, rect_y, 10, 10), false));
                }
            }
        }

        Sprite {
            pixels: pixels,
            x: x * 10,
            y: y * 10,
        }
    }
}
//-------------------------------------------------------------------------------------------------
