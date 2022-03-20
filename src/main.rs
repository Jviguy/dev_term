mod commands;
use dev_term_io::CommandIo;
use std::env::{current_dir};
use std::fs;
use std::io::{stdout, stdin, Write, Result};
use colored::Colorize;
use std::process::Command;
use regex::Regex;
use serde_json::{json, Value};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const ASCII: &str =             r#"
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


fn main() -> Result<()> {
    println!("{}", "         / \\---------------------------,
         \\_,|                          |
            |    Welcome to Dev Term   |
            |  ,-------------------------
            \\_/________________________/ ".green());
    if cfg!(windows) {
        if let Err(e) = windows_terminal_profile() {
            println!("Error setting terminal profile: {}", e);
        }
    }
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
    if let Some(name) = args.next() {
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
    }
}

fn windows_terminal_profile() -> Result<()>{
    let path = dirs::data_local_dir().expect("no data dir").join("Packages/Microsoft.WindowsTerminal_8wekyb3d8bbwe/LocalState/settings.json");

    let data = fs::read_to_string(path.clone())?;
    let mut json: Value = serde_json::from_str(&*data)?;
    let mut d: Vec<Value> = json["profiles"]["list"].as_array().unwrap().to_vec();

    for i in d.clone() {
        if i["guid"] == "{577a3f6f-49a4-47c7-bbe5-d698aab06e0e}" {
            return Ok(())
        }
    }
    d.push(json!({
        "guid": "{577a3f6f-49a4-47c7-bbe5-d698aab06e0e}",
        "name": "DevTerm",
        "commandline": "dev_term.exe",
        "hidden": false,
    }));

    json["profiles"]["list"] = Value::Array(d);
    fs::write(path, serde_json::to_string_pretty(&json).unwrap())?;
    Ok(())
}