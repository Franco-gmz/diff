use std::env;
use crate::errors::*;

///Structure that contains the console parameters.
pub struct Arguments {
    pub arg1: String,
    pub arg2: String,
}

impl Arguments {
    ///It gets the parameters to execute diff between two files. So if the number of
    /// parameters is distinct to three, it returns an error as String.
    /// Arguments must be three because the first is the name of the executable.
    pub fn new() -> Result<Self, Errors> {
        let args: Vec<String> = env::args().collect();
        if args.len() == 3 {
            Ok(Arguments {
                arg1: args[1].clone(),
                arg2: args[2].clone(),
            })
        } else {
            Err(Errors::ArgError("Insuficiente cantidad de parametros".to_string()))
        }
    }
}
