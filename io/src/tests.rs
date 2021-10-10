use crate::command_io;
use crate::CommandIo;

command_io! {
    enum Command : String {
        TestCommand = "test",
    }
}

command_io! {
    struct Test : "description", "usage" {
        pub embed1: String, "another little test",
    }
}

command_io! {
    struct TestCommand : "description", "usage" {
        pub arg1: String, "the string to be echoed",
        pub arg2: i32, "a random number for testing",
        pub arg3: Test, "a embedded struct for testing purposes",
    }
}

impl TestCommand {
    pub fn execute(&self) -> std::io::Result<()> {
        println!("{} {} {}", self.arg1, self.arg2, self.arg3.embed1);
        Ok(())
    }
}

#[test]
fn command() -> std::io::Result<()> {
    let _cmd = Command::read(&mut "test sex 69 sex".split(" ").into_iter())?;
    Ok(())
}

#[test]
fn command_args_desc() -> std::io::Result<()> {
    println!("{}", TestCommand::args_usage());
    Ok(())
}

#[test]
fn command_desc() -> std::io::Result<()> {
    assert_eq!(TestCommand::description(), "description".to_string());
    Ok(())
}