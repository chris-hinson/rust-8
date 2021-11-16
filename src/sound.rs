use std::time::Instant;
//-----------------------------------------------Sound---------------------------------------------
pub struct Sound {
    //freq in Hz at which the times should decrease while non-zero
    //should always by 60
    pub freq: f32,
    pub dt:u8,
    pub st:u8,
    //instant representing the last time the timers were updatd
    //use this in conjunction with .elapsed to update @ 60Hz
    pub dt_lu: Instant,
    pub st_lu: Instant,
}

impl Sound {
    pub fn new() -> Sound {
        Sound { freq:60.0, dt:0, st:0,dt_lu: Instant::now(), st_lu: Instant::now()}
    }
}
//-------------------------------------------------------------------------------------------------
