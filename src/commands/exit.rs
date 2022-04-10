use dev_term_io::command_io;
use dev_term_io::Executable;

command_io! {
    struct Exit : "Exits the current instance of dev_term", "exit" {
    }
}

impl Executable for Exit {
    fn execute(&self) -> anyhow::Result<()> {
        println!("Goodbye!");
        std::process::exit(0);
    }
}