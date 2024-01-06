use std::collections::HashMap;
use crate::command_traits::Command;
pub struct CommandRegistry {
    registry: HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry{
    pub fn new() -> Self{
        let mut registry = HashMap::new();
        registry.insert("build-dir".to_string(), Box::new(BuildDirCommand {}));

        CommandRegistry { registry }
    }
    pub fn get_command(&self, command_name: &str) -> Option<&Box<dyn Command>>{
        self.registry.get(command_name)
    }
}

#[cfg(test)]
mod command_registry_test{
    use super::*;

    #[test]
    fn test_command_registry_get_command(){
        let command_registry = CommandRegistry::new();

        let build_dir = command_registry.get_command("build_dir");
        assert!(build_dir.is_some());

        let unknown_command = command_registry.get_command("unknown");
        assert!(unknown_command.is_none());
    }
    struct MockCommand;
    impl Command for MockCommand {
        fn do_the_thing(&self, args: &[String]) {
            // Mock implementation
            println!("Executing MockCommand: {:?}", args);
        }
    }

}