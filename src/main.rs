use std::io::Read;
use std::path::Path;
use std::fs::File;
use colored::*;

fn main() -> std::io::Result<()>{

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 { 
        println!("Usage {} filename pattern", args[0]);
        return Ok(());
    }    

    let filepath = Path::new(&args[1]);
    let pattern = &args[2];

    if !filepath.exists() {
        println!("[!] File does not exist");
        return Ok(());
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
        match line.find(pattern) {
            Some(n) => {
              println!("{}{}{}", &line[..n], pattern.green(), &line[n+pattern.len()..]);
              found = true;
            } 
            None => {}
        }
    });

    if !found {
        println!("[!] '{}' is not found in {:?}", pattern.red(), filepath);
    }

    Ok(())
}