pub use std::fs;
use std::fs::File;
use std::io::Read;
//---------------------------------------------ROM---------------------------------------------
pub struct ROM {
    pub buffer: Vec<u8>,
}

impl ROM {
    pub fn new(filename: &str) -> ROM {
        let mut f = File::open(&filename).expect("file not found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read_exact(&mut buffer).expect("buffer overflow");

        ROM { buffer }
        //DEBUG: print vec as bytes
        //println!("{:#04x?}", buffer);
    }
}

//---------------------------------------------------------------------------------------------
