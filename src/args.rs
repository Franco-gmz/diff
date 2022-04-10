use crate::errors::*;
use std::env;

///Structure that contains the console parameters.
#[derive(Debug, PartialEq)]
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
            Err(Errors::ArgError(
                "Insuficiente cantidad de parametros".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::args::Arguments;
    use crate::errors::*;

    #[test]
    fn no_args_test() {
        let expected = Errors::ArgError("Insuficiente cantidad de parametros".to_string());
        assert_eq!(Err(expected), Arguments::new());
    }
}
