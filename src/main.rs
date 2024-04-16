use katuk_rs::display;

fn main() {
    match katuk_rs::run() { 
        Err(err) => { 
            display::print_err(err); 
        }
        Ok(()) => ()
    }
    
}
