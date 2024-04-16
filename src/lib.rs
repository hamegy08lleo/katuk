pub mod dao; 
pub mod display;
pub mod bookmark;

use std::{env, error::Error};
use std::path::Path;
use dao::Dao;


pub fn run() -> Result<(), Box<dyn Error>> { 
    let args: Vec<String> = env::args().collect(); 


    if args.len() < 2 { 
        return Err(format!("Invalid arguments").into());
    }
    else { 
        let mut dao: Dao = Dao::new();
        match args[1].as_str() { 
            "-a" => { 
                let name = match args.get(2) { 
                    Some(name) => name, 
                    None => return Err(format!("Invalid arguments").into()) 
                };

                let current_dir = env::current_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                let path = match args.get(3) { 
                    Some(path) => { 
                        if Path::new(&path).exists() { 
                            path
                        }
                        else { 
                            return Err(format!("Path does not exist").into()); 
                        }
                    }, 
                    None => { 
                        &current_dir
                    }
                };

                match dao.add_bookmark(name, path) { 
                    Ok(()) => { 
                        println!("add");
                        display::print_ok(&format!("add bookmark {} -> {}", name, path));
                        dao.write(); 
                    },
                    Err(err) => { 
                        display::print_err(err);
                    }
                }
            }, 
            "-p" => { 
                // get path
                let name = match args.get(2) { 
                    Some(name) => name, 
                    None => return Err(format!("Invalid arguments").into()) 
                };

                match dao.get_path(&name) { 
                    Ok(path) => { 
                        println!("cd"); 
                        println!("{}", path); 
                    }
                    Err(err) => { 
                        return Err(err);
                    }
                }
            }, 
            "-l" => { 
                println!("ls");
                let _ = dao.list_bookmark(); 
            },
            "-e" => { 
                let name = match args.get(2) { 
                    Some(name) => name, 
                    None => return Err(format!("Invalid arguments").into()) 
                };

                let current_dir = env::current_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                let path = match args.get(3) { 
                    Some(path) => { 
                        if Path::new(&path).exists() { 
                            path
                        }
                        else { 
                            return Err(format!("Path does not exist").into()); 
                        }
                    }, 
                    None => { 
                        &current_dir
                    }
                };

                match dao.edit_bookmark(name, path) { 
                    Ok(()) => { 
                        println!("edit");
                        display::print_ok(&format!("edit bookmark {} -> {}", name, path));
                        dao.write(); 
                    },
                    Err(err) => { 
                        display::print_err(err);
                    }
                }
            }
            _ => { 
                return Err(format!("Invalid Arguments!").into()); 
            }
        }
    }
    Ok(())
}


