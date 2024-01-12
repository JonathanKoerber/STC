use std::env;
mod command_registry;
mod command_traits;
use command_registry::CommandRegistry;
//mod build_dir;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: stc <command> **args");
        return;
    }
    // Todo add check to handle --help of -h print list of commands

    let command_name = &args[1];
    let command_registry = CommandRegistry::new();
    match command_registry.get_command(command_name){
        Some(command) => {
            command.do_the_thing(&args[2..]);
        }
        None => {
            println!("Unknown command: {}", command_name);
        }
    }
}

// #[cfg(test)]
// mod main_function_tests {
//     use crate::command_registry::CommandRegistry;
//     use super::super::*;
//     use crate::command_traits::Command;
//
//
//     #[test]
//     fn test_main_build_dir_command() {
//         // Create a mock command registry
//         let mut test_registry = CommandRegistry::new();
//         match test_registry.get_command(){
//             Some(command) => {
//                 command.do_the_thing(&args[2..]);
//             }
//             None => {
//                 println!("Unknown command: {}", command_name);
//             }
//         }
//
//         // Run the main function with the mock registry
//         process_command(&mock_registry, vec!["stc".to_string(), "build-dir".to_string()]);
//
//         // Assertions or additional testing logic can be added here
//     }
// }
