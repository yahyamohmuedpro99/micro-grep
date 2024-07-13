use std::error::Error;
use std::{env, fs};
use std::{process};

pub struct Arguments {
    pub query: String,
    pub filename: String,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &str> {
        // check that args set correctly
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: <script> <query> <filename>");
        }

        Ok(Arguments {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

pub fn get_content_from(filename: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error to get the file : {:?}", err);
        process::exit(1)
    });
    Ok(content)
}

// i use box here to add the error happend when ever the type of it
pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    let case_sensetive=env::var("CASE_SENSITIVE").is_err();


    let content = get_content_from(&args.filename).unwrap_or_else(|err| {
        eprintln!("error happend because he can't get the file {:?}", err);
        process::exit(1)
    });
    let result=search(&args.query, &content,case_sensetive);
    println!("found those : {:?}", result);
    Ok(())
}

// add the life time for the content bcz the result i return is live in content
pub fn search<'a>(query: &str, content: &'a str,case_sensitive:bool) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];
    for line in content.lines() {
        if case_sensitive && line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }else if line.contains(&query) {
            result.push(line);
        }
    }
    result
}

// ------------- test --------------

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_result() {
        let query = "something";
        let content = "i have something here\n
            another line that not have some thing\n
            another Somethings but cap
        ";

    }

    #[test]
    fn not_case_senstive(){
        let query = "SoMeThing";
        let content: &str = "i have something here\nanother line that not have some thing\nanother Somethings but cap\n";

        assert_eq!(vec!["i have something here","another Somethings but cap"], search(query, content,true))
        }

}
