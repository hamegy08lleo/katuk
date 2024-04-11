use std::{collections::{self, BTreeMap}, env, fs::{self, File}, io::{self, BufReader}, process, error::Error};

pub mod add; 
use add::Add;

type Map<K, V> = BTreeMap<K, V>;

pub fn run() -> Result<(), Box<dyn Error>> { 
    let args: Vec<String> = env::args().collect(); 


    if args.len() < 2 { 
        return Err(format!("Invalid Arguments").into());
    }
    else { 
        match args[1].as_str() { 
            "-n" => { 
                // TODO: add bookmark
                println!("add bookmark");
            }, 
            "-p" => { 
                // get path
                let name = match args.get(2) { 
                    Some(name) => name, 
                    None => return Err(format!("Invalid Arguments!").into()) 
                };
                let mut map = Map::new();
                match get_path(name, &mut map) { 
                    Ok(path) => { 
                        println!("{}", path); 
                    }
                    Err(err) => { 
                        return Err(err);
                    }
                }
            }
            _ => { 
                return Err(format!("Invalid Arguments!").into()); 
            }

        }
    }

    return Ok(()); 

    // let add = match Add::new(args) { 
    //     Ok(add) => add, 
    //     Err(err) => print_err(err) 
    // };
   

    Ok(())
}

fn read_data(map: &mut Map<String, String>) -> () { 
    let data = match fs::read_to_string("/mnt/c/Users/katuk812/Code/rust/katuk/data.txt") { 
        Ok(data) => data,
        Err(err) => print_err(Box::new(err))
    };
    let data: Vec<&str> = data.lines().collect(); 
    for line in data { 
        let word: Vec<&str> = line.split_whitespace().collect();
        map.insert(word[0].to_string(), word[1].to_string());
    }
}

fn get_path(name: &str, map: &mut Map<String, String>) -> Result<String, Box<dyn Error>> { 
    read_data(map); 
    match map.get(name) { 
        Some(path) => { 
            return Ok(path.to_string()) 
        },
        None => {
            return Err(format!("Bookmarks doesn't exists").into())
        }
    };
}

pub fn print_err(err: Box<dyn Error>) -> ! { 
    println!("Failed: {}", err); 
    process::exit(1);
}
