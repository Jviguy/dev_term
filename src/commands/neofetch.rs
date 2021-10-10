use dev_term_io::command_io;
use crate::commands::Executable;

command_io! {
    struct Neofetch : "Displays information about the current system wellbeing.", "neofetch" {

    }
}

impl Executable for Neofetch {
    fn execute(&self) -> std::io::Result<()> {
        println!("todo");
        Ok(())
    }
}