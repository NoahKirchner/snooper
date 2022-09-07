use std::env;


mod parser;
use parser::*;


fn main() {
    let args = env::args().nth(1)
        .expect("Input should be snooper <username>");

    let browser = load_browser().ok()
        .expect("Error assigning browser object.");

    let mut output:Vec<WebReturn> = Vec::new();


    
    let targets = targets_from_yaml("targets.yaml".to_string()).unwrap();
    
    for (_k, v) in targets.iter(){
        output.push(parse_page(&browser, &args, v).unwrap());
    }
    
    println!("{:?}", output);

}
