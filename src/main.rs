use katuk::display;

fn main() {
    match katuk::run() { 
        Err(err) => { 
            display::print_err(err); 
        }
        Ok(()) => ()
    }
    
}
