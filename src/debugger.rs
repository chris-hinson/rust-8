pub mod debugger{

    pub struct debugger{
        //cpu:CPU,
    }

    impl debugger{
        pub fn new()->debugger{
            println!("called new debugger");
            return debugger{};
        }
    }
}
