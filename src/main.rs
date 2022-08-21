mod utils;

use crate::utils::{scrape, print_vec};

fn main() {
    let transl_word = scrape();

    match transl_word {
        Ok(x) => print_vec(x),
        _ => println!("{}", "Could not find the word."),
    }
}

