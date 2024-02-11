use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Parser)]
struct CLI{
   pattern: String,
   path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CLI::parse();

    println!("Pattern: {:?}\nPath: {:?}\n", args.pattern, args.path);

    let file = match File::open(&args.path){
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could Not Open File: {}", err);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);

    for (i,line) in reader.lines().enumerate(){
        match line {
            Ok(line) => {
                if line.contains(&args.pattern){
                    println!("{} : {}", i, line);
                }
            },
            Err(err) => eprintln!("Error Reading Line {}: {}", i, err),
        }
    }

    Ok(())
}
