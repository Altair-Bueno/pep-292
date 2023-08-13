//! Try the example using the following command
//!
//! ```sh
//! cargo run --example envsubstr <<EOF
//! My shell is $SHELL
//! My current working directory is $PWD
//! EOF
//! ```

use pep_292::Template;
use std::collections::HashMap;
use std::env::vars;
use std::io::read_to_string;
use std::io::stdin;

fn main() {
    let stdin = stdin().lock();
    let input = read_to_string(stdin).expect("Only valid UTF-8 strings are supported");
    let environment = vars().collect::<HashMap<String, String>>();
    let output = input.substitute(&environment).expect("Substitution error");
    print!("{output}")
}
