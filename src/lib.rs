 use std::fs;
 use std::error::Error;
use anyhow::Result;

use clap::{Parser};




  
#[derive(Debug,Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// your query string
     #[arg(short, long, value_name = "Query")]
    query: String,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    filename: String,

    /// Is case sensitive search, false by default
    #[arg(short, long)]
    case_sensitive: Option<bool>,
}


pub fn run ()->Result<(),Box<dyn Error>>{
     let args = Cli::parse();
    let contents = fs::read_to_string(args.filename)?;
    
    let results = match args.case_sensitive {
        Some(true) =>  search(&args.query, &contents),
        Some(false) => search_case_sensitive(&args.query, &contents), 
        None => search(&args.query, &contents),
    }; 
    for line in results{
        println!("{}",line);
    }
    Ok(())
}
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}
pub fn search_case_sensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(query.to_lowercase().as_str()){
            results.push(line);
        }
    }
    results
}
pub struct Config {
   pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String], case_sensitive :bool) -> Result<Config,&str> {
    if args.len() < 3 {
        return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    Ok(Config{query, filename,case_sensitive})
}


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "safe, fast, productive.
       \n Pick three.
       \n Duct tape.
       ";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }

    #[test]
    fn case_sensitive(){
        let query = "dUct";
        let contents = "safe, fast, productive.
       \n Pick three.
       \n Duct tape.
       ";
        assert_eq!(vec!["safe, fast, productive.", " Duct tape."],search_case_sensitive(query,contents));    }
}