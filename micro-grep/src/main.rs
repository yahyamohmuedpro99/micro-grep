
use std::{env, process};
use micro_grep::{run, Arguments};

fn main() {
    // take the args given for runing this bin
    let args:Vec<String>=env::args().collect();
    let arguments=Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!("Error happend at parsing args: {:?}",err);
        process::exit(1)
    });

    if let Err(e) = run(arguments){
        eprintln!("Application Error : {:?} ",e);
        process::exit(1)
    }
  
}
