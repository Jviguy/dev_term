mod commands;
use dev_term_io::CommandIo;
use std::env::{current_dir};
use std::io::{stdout, stdin, Write};
use colored::Colorize;
use std::process::Command;
use regex::Regex;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const ASCII: &'static str =             r#"           
                                                 ,----,                                  
                                               ,/   .`|                                  
    ,---,                                    ,`   .'  :                           ____   
  .'  .' `\                                ;    ;     /                         ,'  , `. 
,---.'     \                             .'___,/    ,'           __  ,-.     ,-+-,.' _ | 
|   |  .`\  |               .---.        |    :     |          ,' ,'/ /|  ,-+-. ;   , || 
:   : |  '  |   ,---.     /.  ./|        ;    |.';  ;   ,---.  '  | |' | ,--.'|'   |  || 
|   ' '  ;  :  /     \  .-' . ' |        `----'  |  |  /     \ |  |   ,'|   |  ,', |  |, 
'   | ;  .  | /    /  |/___/ \: |            '   :  ; /    /  |'  :  /  |   | /  | |--'  
|   | :  |  '.    ' / |.   \  ' .            |   |  '.    ' / ||  | '   |   : |  | ,     
'   : | /  ; '   ;   /| \   \   '            '   :  |'   ;   /|;  : |   |   : |  |/      
|   | '` ,/  '   |  / |  \   \               ;   |.' '   |  / ||  , ;   |   | |`-'       
;   :  .'    |   :    |   \   \ |            '---'   |   :    | ---'    |   ;/           
|   ,.'       \   \  /     '---"                      \   \  /          '---'            
'---'          `----'                                  `----'                            
                                                                                         

            "#;


fn main() -> std::io::Result<()> {
    println!("{}", "         / \\---------------------------,
         \\_,|                          |
            |    Welcome to Dev Term   |
            |  ,-------------------------
            \\_/________________________/ ".green());
    let regex = Regex::new(r#"(?m)("[^"]+"|[^\s"]+)"#).unwrap();
    loop {
        let wd = current_dir()?;
        print!("[{}] ➜ ", wd.display().to_string().as_str().blue());
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        if input.starts_with("./") || input.starts_with(".\\") {
            exec_os_command(&regex, input);
            continue;
        }
        let mut args = regex.find_iter(input.trim());
        let command = crate::commands::Command::read(&mut args);
        if let Ok(inner) = command {
                match inner.execute() {
                    Ok(_) => (),
                    Err(e) => {
                        println!("An error occurred while running that command!");
                        println!("Error: {}", e.to_string())
                    }
                }
        } else if let Err(_e) = command {
            exec_os_command(&regex, input)
        }
    }
}

fn exec_os_command(regex: &regex::Regex, input: String) {
    let mut args = regex.find_iter(input.trim());
    match args.next() {
        //check error type as to determine whether we should closest_match
        Some(name) => {
            let mut win_cmd = Command::new(name.as_str());
            let mut vec = vec![];
            for arg in args {
                vec.push(arg.as_str())
            }
            match win_cmd.args(vec).status() {
                Ok(_) => (),
                Err(e) => {
                    println!("An error occurred while running that command!");
                    println!("Error: {}", e.to_string())
                }
            }
        },
        None => (),
    }
}