use dev_term_io::command_io;
use dev_term_io::Executable;
use figlet_rs::FIGfont;

command_io! {
    struct Ascii: "Generates ASCII art from a given string!", "ascii \"Dev Term\"" {
        pub ascii: String, "The given text to be made into ASCII art",
        pub font_file: Option<String>, "The file in which a custom font is stored for usage",
    }
}

impl Executable for Ascii {
    fn execute(&self) -> std::io::Result<()> {
        let font = match &self.font_file {
            Some(path) => {
                FIGfont::from_file(path.as_str()).unwrap()
            }
            None => FIGfont::standand().unwrap(),
        };
        println!("{}", font.convert(self.ascii.as_str()).unwrap());
        Ok(())
    }
}