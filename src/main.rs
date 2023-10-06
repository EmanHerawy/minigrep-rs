use std::env;
 use std::process;
use minigrep::{Config,run};
  fn main() {
    let args : Vec<String>= env::args().collect();
    let case_sensitive =match  env::var("CASE_INSENSITIVE") {
        Err(_) => {
            false
        },
        Ok(val) => {
           val.parse().unwrap_or(false)
        }
    }; 
    
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


