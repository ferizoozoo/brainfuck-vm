use std::{
    env::{self},
    fs::File,
    io::{stdin, stdout, Read},
};

use machine::Machine;

mod machine;

fn main() {
    let filename = &env::args().collect::<Vec<String>>()[1];
    match File::open(filename) {
        Ok(mut source) => {
            let mut code = String::new();
            source.read_to_string(&mut code).unwrap();
            let mut machine = Machine::new(code, Box::new(stdin()), Box::new(stdout()));
            machine.execute();
        }
        Err(e) => panic!("{}", e),
    }
}
