use std::{error::Error, collections::BTreeMap, fs::{self, File}, io::prelude::*};
use crate::display;
use crate::bookmark::Bookmark;

type Map<K, V> = BTreeMap<K, V>;

pub struct Dao {
    pub map: Map<String, String>
} 

impl Dao { 
    pub fn new() -> Self { 
        let data = match fs::read_to_string("/mnt/c/Users/katuk812/Code/rust/katuk/data.txt") { 
            Ok(data) => data,
            Err(err) => display::print_err(Box::new(err))
        };
        let mut map: Map<String, String> = Map::new();
        let data: Vec<&str> = data.lines().collect(); 
        for line in data { 
            let word: Vec<&str> = line.split_whitespace().collect();
            map.insert(word[0].to_string(), word[1].to_string());
        }
        Self { map } 
    }

    pub fn get_path(& self, name: &str) -> Result<String, Box<dyn Error>> { 
        match self.map.get(name) { 
            Some(path) => { 
                return Ok(path.to_string()) 
            },
            None => {
                return Err(format!("Bookmarks doesn't exists").into())
            }
        };
    }

    pub fn add_bookmark(&mut self, name: &str, path: &str) -> Result<(), Box<dyn Error>> { 
        match self.map.get(name) { 
            Some(path) => { 
                Err(format!("Exist a bookmarks with a same name\n{} -> {}", name, path).into())
            },
            None => { 
                self.map.insert(name.to_string(), path.to_string());
                Ok(())
            }
        }
    }

    pub fn edit_bookmark(&mut self, name: &str, path: &str) -> Result<(), Box<dyn Error>> { 
        match self.map.get(name) { 
            Some(..) => { 
                self.map.insert(name.to_string(), path.to_string()); 
                Ok(())
            }
            None => { 
                Err(format!("Bookmarks does not exists").into())
            }
        }
    }

    pub fn list_bookmark(& self) -> Result<(), Box<dyn Error>> { 
        for (key, value) in &self.map { 
            display::print_bookmark(Bookmark { name: key.clone(), path: value.clone() } );
        }
        Ok(())
    }

    pub fn write(& self) -> () { 
        let mut file = File::create("/mnt/c/Users/katuk812/Code/rust/katuk/data.txt")
            .expect("Cant create file");
        for (key, value) in &self.map { 
            let _ = writeln!(file, "{} {}", key, value); 
        }
    }

}

