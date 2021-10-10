use dev_term_io::command_io;
use dev_term_io::Executable;

command_io! {
    struct Exit : "exits the current instance of dev_term", "exit" {
    }
}

impl Executable for Exit {
    fn execute(&self) -> std::io::Result<()> {
        println!("Goodbye!");
        std::process::exit(0);
    }
}