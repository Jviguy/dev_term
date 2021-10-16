use crate::command_io;
use crate::CommandIo;
use crate::Executable;

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

impl Executable for TestCommand {
    fn execute(&self) -> std::io::Result<()> {
        println!("{} {} {}", self.arg1, self.arg2, self.arg3.embed1);
        Ok(())
    }
}

#[test]
fn command() -> std::io::Result<()> {
    let regex = regex::Regex::new(r#"(?m)("[^"]+"|[^\s"]+)"#).unwrap();
    let mut args = regex.find_iter("test sex 69 sex");
    let _cmd = Command::read(&mut args)?;
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