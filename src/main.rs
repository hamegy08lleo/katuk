use std::{process, error::Error};

fn main() {
    match katuk::run() { 
        Err(err) => { 
            katuk::print_err(err); 
        }
        Ok(()) => ()
    }
}

