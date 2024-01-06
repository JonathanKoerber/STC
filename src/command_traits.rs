
    pub trait Command {
        fn do_the_thing(&self, args: &[String]);
    }
