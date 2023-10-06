use std::env;
 use std::process;
use minigrep::{Config,run};
  fn main() {
    let args : Vec<String>= env::args().collect();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    let config:Config =Config::new(&args,case_sensitive).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}",err);
        process::exit(1);
    });
 
    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);

    if let Err(e) = run(config){
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
}


