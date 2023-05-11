mod parser;
use parser::*;
use std::error::Error;

fn main() {
    let username;
    
    match std::env::args().nth(1){
        Some(arg) => {username = arg},
        _ => {println!("Invalid input. Here is the proper syntax: snooper [username]"); std::process::exit(1)}
        
    }

    let browser = load_browser().ok()
        .expect("Error assigning browser object.");

    let mut output:Vec<Result<WebReturn, Box<dyn Error>>> = Vec::new();


    // This really needs a better solution but I cannae be assed right now.
    let targets = targets_from_yaml("targets.yaml".to_string()).expect("Make sure you have a targets.yaml file (It's in the home directory)");
    
    for (_k, v) in targets.iter(){
        output.push(parse_page(&browser, &username, v));
    }
    
    for entry in output.iter() {
        match entry {
            Ok(value) => {
                match value {
                    WebReturn::Success{username, url, other} => {println!("[{}]: Username: {}, Bio: {}", url, username, other)},
                    WebReturn::Failure{..} => {}}
            },
            Err(error) => {println!("Error: {:?}", error)},
            
        }
    }

}
