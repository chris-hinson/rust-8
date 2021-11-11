pub mod sound{
    use std::time::Instant;
    //-----------------------------------------------Sound---------------------------------------------
    pub struct Sound {
        //freq in Hz at which the times should decrease while non-zero
        //should always by 60
        pub freq: f32,
        pub DT:u8,
        pub ST:u8,
        //instant representing the last time the timers were updatd
        //use this in conjunction with .elapsed to update @ 60Hz
        pub DT_lu: Instant,
        pub ST_lu: Instant,
    }

    impl Sound {
        pub fn new() -> Sound {
            Sound { freq:60.0, DT:0, ST:0,DT_lu: Instant::now(), ST_lu: Instant::now()}
        }
    }
    //-------------------------------------------------------------------------------------------------
}
