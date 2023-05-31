use std::io::{Read, Write};
use std::path::Path;
use std::fs::File;
use colored::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author,version,about, long_about = None)]
struct Args {
    #[arg(short,long)]
    filename: String,

    #[arg(short,long)]
    pattern: String
}


fn main() -> std::io::Result<()>{

    let args = Args::parse();
    let filepath = Path::new(&args.filename);
    let pattern = args.pattern;

    if !filepath.exists() {
        println!("{}", "[!] File does not exist".red());
        return Ok(());
    } else if !filepath.is_file() {
        println!("{}", "[!] Not a regular file".red());
        return Ok(())
    }

    let mut file = File::open(filepath)?;

    let mut filecontent = String::new();

    let n = match file.read_to_string(&mut filecontent) {
        Ok(n) => {
            n
        }   
        Err(_) => 0
    };

    if n == 0 {
        println!("[!] File is empty");
        return Ok(())
    }

    let mut found = false;

    filecontent.lines().for_each(|line|{
        match line.find(&pattern) {
            Some(_) => {
             line.split_ascii_whitespace().for_each(|w|{
                if w == pattern {
                    print!("{} ", pattern.green());
                    found = true;
                } else {
                    print!("{} ", w);
                }
                match std::io::stdout().flush() {
                    Ok(()) => {}
                    Err(_) => {}
                }
             });

             println!();
            } 
            None => {}
        }
    });

    if !found {
        println!("[!] '{}' is not found in {:?}", pattern.red(), filepath);
    }
    Ok(())
}