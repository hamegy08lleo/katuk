pub struct Add { 
    name: String, 
    path: String,
}

impl Add { 
    pub fn new(args: Vec<String>) -> Result<Self, &'static str> { 
        if args.len() < 3 { 
            return Err("Not enough arguments"); 
        }
        let name = args[1].clone();
        let path = args[2].clone(); 
        Ok(Self {name, path} ) 
    }
}
