use std::{error::Error, fs, env};


pub fn run (config:Config)-> Result<(),Box<dyn Error>>{
    let result = fs::read_to_string(config.data_path)?;

    let search_result = if config.case_sensitive{
        search(&config.query,&result)
    }else{
        search_case_insensitive(&config.query,&result)
    };
    
   
    for line in search_result{
        println!("{}",line);
    }
    Ok(())
}

pub struct Config{
    pub query:String,
    pub data_path:String,
    pub case_sensitive:bool,
}

impl Config {

    pub fn new(args:&[String])-> Result<Config,& str>{
        if args.len() < 3 {
            return Err("ERROR:Args length is not valid !!");
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_ok();
        Ok(
        Config { query: args[1].clone(), data_path: args[2].clone(),case_sensitive})
    }
}

fn search<'a>(query:&str,content:&'a str)-> Vec<&'a str>{
    let mut search_result = Vec::new();

    for line in content.lines(){
        if line.contains(&query){
            search_result.push(line);
        }
    }
    search_result
}

fn search_case_insensitive<'a>(query:&str,content:&'a str)-> Vec<&'a str>{
    let mut search_result = Vec::new();
    let query = query.to_lowercase();

    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            search_result.push(line);
        }
    }
    search_result
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_search(){
        let query = "test";
        let content = "this is a test content";
        assert_eq!(vec!["this is a test content"],search(query,content));
    }

    #[test]
    fn test_case_insensitive_search(){
        let query = "test";
        let content = "this is a test content";
        assert_eq!(vec!["this is a test content"],search_case_insensitive(query,content));
    }
    
}
