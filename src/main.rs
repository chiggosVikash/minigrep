use std::env;
use std::process;
use minigrep;
fn main() {
    
    let args: Vec<String> = env::args().collect(); 

    let config = minigrep::Config::new(&args).unwrap_or_else(|_err|{
        panic!("{}",_err)
    });

    
    if let Err(e) = minigrep::run(config){
        println!("Application Error {e}");
        process::exit(1);

    };

}
