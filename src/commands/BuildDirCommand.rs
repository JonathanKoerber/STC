use crate::command_traits::Command;
use st::fs::{self, File};
use::io::{Write};
use::path::{Path};

mod build_dir {
    use std::fs;

    pub struct BuildDirCommand {
        template_string: String,
        num_dirs: u32,
        readme_str: String,
        course_name: String,

    }

    impl Command for BuildDirCommand {
        // build dir command should h
        fn parse_args(self, args: &[String]) {
            if args.len() < 2 {
                command_error();
            }
            if args.len() != 2 || args.len() != 4{
                println!("wrong number of elements");
            }

            self.template_string = args[0];
            self.num_dirs = args[1].u32();
            if args[2] == "-r" && args.len() == 4{
                self.course_name = args[3];
                let readme_path = args[4];
                self.readme_str = File::create
            }
        }
    }
    fn do_the_thing(){
        for i in 1..=num_dirs{
            let dir_name = format!("{}{:20}", template_string, i);
            let dir_path = Path::new(".").join(&dir_name);
            fs::create_dir(&dir_path).expect("Failed to create directory");
            printnl!("Created directory: {:?}", dir_path);
            if readme_str.is_some(){

            }
        }

    }
    fn get_readme(path: &String) -> String {
        match fs::read_to_string(path){
            Ok(content) => content,
            Err(err) => {
                eprintls!("Error reading README Template {}", err);
                String::new()
            },
        }
    }
    fn command_error (){
        println!("build dir command requires two arguments.");
        println!("Use from: stc build-dir <template string> <number of directory>");
        println!("You provided stc build-dir {}", ::args);
    }
}
