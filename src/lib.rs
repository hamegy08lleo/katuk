use std::{collections::{self, BTreeMap}, env, fs::{self, File}, io::{self, BufReader}, process, error::Error};

pub mod add; 
use add::Add;

type Map<K, V> = BTreeMap<K, V>;

pub fn run() -> Result<(), Box<dyn Error>> { 
    let args: Vec<String> = env::args().collect(); 

    let add = match Add::new(args) { 
        Ok(add) => add, 
        Err(err) => print_err(err) 
    };
    
    let data = read_data(); 
    let mut map = Map::new(); 
    for line in data { 
        println!("{} {}", line[0], line[1]); 
        map.insert(line[0].clone(), line[1].clone()); 
    }

    Ok(())
}

fn read_data() -> Vec<Vec<String>> { 
    let data = match fs::read_to_string("data.txt") { 
        Ok(data) => data,
        Err(..) => print_err("Failed")
    };
    let data: Vec<String> = data.lines().map(|line| line.to_string()).collect(); 
    let mut tmp: Vec<Vec<String>> = Vec::new(); 
    for line in data { 
        let line: Vec<String> = line.split_whitespace().map(|word| word.to_string()).collect();
        tmp.push(line);
    };
    tmp
}

fn print_err(err: &'static str) -> ! { 
    println!("Failed: {}", err); 
    process::exit(1);
}


