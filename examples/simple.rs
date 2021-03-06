// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate ispell;
use ispell::{SpellLauncher, IspellError};

fn display(errors: &[IspellError]) {
    if errors.is_empty() {
        println!("No error, congratulations!");
    } else {
        for e in errors {
            print!("'{}' (at pos {}) is misspelled.", e.misspelled, e.position);
            if !e.suggestions.is_empty() {
                print!(" Maybe you meant '{}'?", e.suggestions[0]);
            }
            println!("");
        }
    }    
}

fn main() {
    let checker = SpellLauncher::new()
        .dictionary("en_GB")
        .command("hunspell")
        .launch();
    match checker {
        Ok(mut checker) => {
            let res = checker.check("test of a msitake").unwrap();
            display(&res);
            let res = checker.check("test without mistake (?)").unwrap();
            display(&res);
            let res = checker.check("Another test wiht a mistake").unwrap();
            display(&res);
        },
        Err(err) => println!("Error: {}", err)
    }
}
