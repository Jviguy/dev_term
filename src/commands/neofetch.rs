use dev_term_io::command_io;
use dev_term_io::Executable;

command_io! {
    struct Neofetch : "Displays information about the current system wellbeing", "neofetch" {

    }
}

impl Executable for Neofetch {
    fn execute(&self) -> anyhow::Result<()> {
        println!("todo");
        Ok(())
    }
}