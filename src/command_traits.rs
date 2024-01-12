
    pub trait Command {
        fn do_the_thing(&self);
        fn parse_args(&self, args: &[String]);
    }
